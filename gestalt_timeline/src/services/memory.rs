//! Memory Service - Persistent memory for agents using SurrealDB + Cortex.
//!
//! Provides short-term (session) and long-term (persistent) memory storage
//! with basic content-based search. Uses Cortex as primary backend with
//! SurrealDB fallback for graceful degradation.

use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::db::SurrealClient;

/// Cortex API response for memory search
#[derive(Debug, Deserialize)]
struct CortexSearchResponse {
    results: Vec<CortexMemory>,
}

/// A memory stored in Cortex
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CortexMemory {
    path: String,
    content: String,
    kind: String,
    #[serde(default)]
    metadata: serde_json::Value,
}

/// Cortex health response
#[derive(Debug, Deserialize)]
struct CortexHealthResponse {
    status: String,
}

use crate::models::timestamp::FlexibleTimestamp;

/// A fragment of memory stored by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFragment {
    /// Unique ID for this memory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<surrealdb::sql::Thing>,
    /// The agent that created this memory
    pub agent_id: String,
    /// The content of the memory
    pub content: String,
    /// Context category (e.g. "conversation", "task_result", "observation")
    pub context: String,
    /// Searchable tags
    pub tags: Vec<String>,
    /// When this memory was created (UTC)
    pub created_at: FlexibleTimestamp,
    /// Importance score 0.0 - 1.0 (used for compaction priority)
    pub importance: f32,
    /// Vector embedding of the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
}

impl MemoryFragment {
    pub fn new(
        agent_id: impl Into<String>,
        content: impl Into<String>,
        context: impl Into<String>,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id: None,
            agent_id: agent_id.into(),
            content: content.into(),
            context: context.into(),
            tags,
            created_at: FlexibleTimestamp::now(),
            importance: 0.5,
            embedding: None,
        }
    }

    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    /// Convert to Cortex memory format
    fn to_cortex_memory(&self) -> CortexMemory {
        let metadata = serde_json::json!({
            "agent_id": self.agent_id.clone(),
            "context": self.context.clone(),
            "tags": self.tags.clone(),
            "importance": self.importance,
            "created_at": self.created_at.to_rfc3339(),
        });

        CortexMemory {
            path: format!(
                "memory/{}/{}/{}",
                self.agent_id,
                self.context,
                self.created_at.0.timestamp()
            ),
            content: self.content.clone(),
            kind: self.context.clone(),
            metadata,
        }
    }
}

/// The MemoryService manages saving and retrieving agent memories.
///
/// # Design Notes
/// - Primary: Cortex HTTP API for distributed memory
/// - Fallback: SurrealDB for local-only operation
/// - Short-term: In-memory cache for recent fragments
#[derive(Clone)]
pub struct MemoryService {
    db: SurrealClient,
    cortex_client: Option<CortexClient>,
    /// In-memory cache of the most recent N fragments per agent (short-term memory)
    short_term: Arc<RwLock<std::collections::VecDeque<MemoryFragment>>>,
    short_term_capacity: usize,
}

/// Cortex HTTP client for memory operations
#[derive(Clone)]
struct CortexClient {
    client: Client,
    url: String,
    token: String,
}

impl CortexClient {
    fn new(url: String, token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");
        Self { client, url, token }
    }

    /// Check if Cortex is healthy
    async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(format!("{}/health", self.url))
            .header("X-Cortex-Token", &self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let health: CortexHealthResponse = response.json().await?;
            Ok(health.status == "ok" || health.status == "healthy")
        } else {
            Ok(false)
        }
    }

    /// Add a memory to Cortex
    async fn add_memory(&self, memory: &CortexMemory) -> Result<()> {
        let response = self
            .client
            .post(format!("{}/memory/add", self.url))
            .header("X-Cortex-Token", &self.token)
            .json(memory)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("Cortex add_memory failed: {} - {}", status, body));
        }

        Ok(())
    }

    /// Search memories in Cortex
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<CortexMemory>> {
        let response = self
            .client
            .post(format!("{}/memory/search", self.url))
            .header("X-Cortex-Token", &self.token)
            .json(&serde_json::json!({
                "query": query,
                "limit": limit
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("Cortex search failed: {} - {}", status, body));
        }

        let search_result: CortexSearchResponse = response.json().await?;
        Ok(search_result.results)
    }
}

impl MemoryService {
    /// Create a new MemoryService with Cortex integration.
    ///
    /// # Arguments
    /// * `db` - SurrealDB client for fallback storage
    ///
    /// Cortex URL and token are read from CORTEX_URL and CORTEX_TOKEN env vars.
    pub fn new(db: SurrealClient) -> Self {
        let url = std::env::var("CORTEX_URL")
            .ok()
            .unwrap_or_else(|| "http://localhost:8003".to_string());
        let token = std::env::var("CORTEX_TOKEN")
            .ok()
            .unwrap_or_else(|| "dev-token".to_string());

        let cortex_client = if url.is_empty() {
            None
        } else {
            Some(CortexClient::new(url, token))
        };

        Self {
            db,
            cortex_client,
            short_term: Arc::new(RwLock::new(std::collections::VecDeque::new())),
            short_term_capacity: 50,
        }
    }

    /// Create with explicit Cortex settings (from config)
    pub fn with_cortex(db: SurrealClient, cortex_url: &str, cortex_token: &str) -> Self {
        let cortex_client = if cortex_url.is_empty() {
            None
        } else {
            Some(CortexClient::new(
                cortex_url.to_string(),
                cortex_token.to_string(),
            ))
        };
        Self {
            db,
            cortex_client,
            short_term: Arc::new(RwLock::new(std::collections::VecDeque::new())),
            short_term_capacity: 50,
        }
    }

    /// Check if Cortex is available
    async fn is_cortex_available(&self) -> bool {
        if let Some(ref client) = self.cortex_client {
            if let Err(e) = client.health_check().await {
                debug!("Cortex health check failed: {}", e);
                return false;
            }
            true
        } else {
            false
        }
    }

    /// Save a memory fragment to short-term cache, Cortex (if available), and SurrealDB.
    pub async fn save(
        &self,
        agent_id: &str,
        content: impl Into<String>,
        context: impl Into<String>,
        tags: Vec<String>,
        embedding: Option<Vec<f32>>,
    ) -> Result<MemoryFragment> {
        let mut fragment = MemoryFragment::new(agent_id, content, context, tags);
        fragment.embedding = embedding;

        // Try Cortex first
        let cortex_available = self.is_cortex_available().await;
        if cortex_available {
            if let Some(ref client) = self.cortex_client {
                let cortex_mem = fragment.to_cortex_memory();
                match client.add_memory(&cortex_mem).await {
                    Ok(_) => {
                        info!(
                            "🧠 Memory saved to Cortex [agent={}] context='{}' tags={:?}",
                            fragment.agent_id, fragment.context, fragment.tags
                        );
                        // Still save to SurrealDB for backup
                        let saved = self.save_to_surreal(&fragment).await?;
                        return Ok(saved);
                    }
                    Err(e) => {
                        warn!("Failed to save to Cortex, falling back to SurrealDB: {}", e);
                    }
                }
            }
        } else {
            debug!("Cortex not available, using SurrealDB directly");
        }

        // Fall back to SurrealDB
        let saved = self.save_to_surreal(&fragment).await?;
        Ok(saved)
    }

    /// Save directly to SurrealDB (used as fallback)
    async fn save_to_surreal(&self, fragment: &MemoryFragment) -> Result<MemoryFragment> {
        let saved = self
            .db
            .create("memories", fragment)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to save memory: {}", e))?;

        // Add to short-term cache
        let mut stm = self.short_term.write().await;
        stm.push_back(saved.clone());
        if stm.len() > self.short_term_capacity {
            stm.pop_front();
        }

        info!(
            "💾 Memory saved to SurrealDB [agent={}] context='{}' tags={:?}",
            saved.agent_id, saved.context, saved.tags
        );

        Ok(saved)
    }

    /// Search memories by tags and content substring, or vector similarity.
    /// Returns the most relevant fragments.
    pub async fn search(
        &self,
        query: &str,
        agent_id: Option<&str>,
        limit: usize,
        embedding: Option<Vec<f32>>,
    ) -> Result<Vec<MemoryFragment>> {
        info!(
            "🔍 Searching memories for '{}' (agent={:?}, limit={})",
            query, agent_id, limit
        );

        // If we have an embedding, use SurrealDB vector search
        if let Some(vector) = embedding {
            let mut query_str = String::from(
                "SELECT *, vector::similarity::cosine(embedding, $vector) AS score
                 FROM memories WHERE embedding IS NOT NONE"
            );
            let mut bindings = serde_json::json!({ "vector": vector, "limit": limit as i64 });

            if let Some(aid) = agent_id {
                query_str.push_str(" AND agent_id = $agent_id");
                bindings["agent_id"] = serde_json::json!(aid);
            }

            query_str.push_str(" ORDER BY score DESC LIMIT $limit");

            let fragments: Vec<MemoryFragment> = self.db
                .query_with(&query_str, bindings)
                .await
                .map_err(|e| anyhow::anyhow!("Vector memory search failed: {}", e))?;

            if !fragments.is_empty() {
                return Ok(fragments);
            }
        }

        // First check short-term cache for quick hits
        let stm = self.short_term.read().await;
        let query_lower = query.to_lowercase();

        let cached: Vec<MemoryFragment> = stm
            .iter()
            .filter(|m| {
                let agent_match = agent_id.is_none_or(|a| m.agent_id == a);
                let content_match = m.content.to_lowercase().contains(&query_lower)
                    || m.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower));
                agent_match && content_match
            })
            .take(limit)
            .cloned()
            .collect();

        if !cached.is_empty() {
            return Ok(cached);
        }

        // Try Cortex if available
        if let Some(ref client) = self.cortex_client {
            if self.is_cortex_available().await {
                match client.search(query, limit).await {
                    Ok(results) => {
                        let fragments: Vec<MemoryFragment> = results
                            .into_iter()
                            .filter_map(|r| {
                                let metadata = r.metadata;
                                let cortex_agent_id = metadata
                                    .get("agent_id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                // Check if agent matches filter (if provided)
                                let agent_match = agent_id
                                    .map(|filter| filter == cortex_agent_id)
                                    .unwrap_or(true);

                                if !agent_match {
                                    return None;
                                }

                                Some(MemoryFragment {
                                    id: None,
                                    agent_id: cortex_agent_id,
                                    content: r.content,
                                    context: r.kind,
                                    tags: metadata
                                        .get("tags")
                                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                                        .unwrap_or_default(),
                                    created_at: metadata
                                        .get("created_at")
                                        .and_then(|v| v.as_str())
                                        .and_then(|v| chrono::DateTime::parse_from_rfc3339(v).ok())
                                        .map(|dt| FlexibleTimestamp(dt.with_timezone(&Utc)))
                                        .unwrap_or_else(FlexibleTimestamp::now),
                                    importance: metadata
                                        .get("importance")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5)
                                        as f32,
                                    embedding: None,
                                })
                            })
                            .collect();

                        if !fragments.is_empty() {
                            info!("🔍 Found {} memories in Cortex", fragments.len());
                            return Ok(fragments);
                        }
                    }
                    Err(e) => {
                        warn!("Cortex search failed, falling back to SurrealDB: {}", e);
                    }
                }
            }
        }

        // Fall back to SurrealDB query
        let mut query_str = String::from("SELECT * FROM memories WHERE string::contains(string::lowercase(content), $q)");
        let mut bindings = serde_json::json!({ "q": query_lower, "limit": limit as i64 });

        if let Some(aid) = agent_id {
            query_str.push_str(" AND agent_id = $agent_id");
            bindings["agent_id"] = serde_json::json!(aid);
        }
        query_str.push_str(" ORDER BY created_at DESC LIMIT $limit");

        let fragments: Vec<MemoryFragment> = self.db
            .query_with(&query_str, bindings)
            .await
            .map_err(|e| anyhow::anyhow!("Memory search query failed: {}", e))?;

        Ok(fragments)
    }

    /// Retrieve recent memories for an agent (most recent first).
    pub async fn recent(&self, agent_id: &str, limit: usize) -> Result<Vec<MemoryFragment>> {
        let stm = self.short_term.read().await;
        let cached: Vec<MemoryFragment> = stm
            .iter()
            .rev()
            .filter(|m| m.agent_id == agent_id)
            .take(limit)
            .cloned()
            .collect();

        if cached.len() >= limit {
            return Ok(cached);
        }

        // Query SurrealDB for more
        let fragments: Vec<MemoryFragment> = self.db
            .query_with("SELECT * FROM memories WHERE agent_id = $agent ORDER BY created_at DESC LIMIT $limit",
                serde_json::json!({ "agent": agent_id, "limit": limit as i64 }))
            .await
            .map_err(|e| anyhow::anyhow!("Recent memories query failed: {}", e))?;

        Ok(fragments)
    }

    /// Build a context string from recent memories to inject into LLM prompts.
    pub async fn build_context_string(
        &self,
        agent_id: &str,
        query: Option<&str>,
        max_chars: usize,
        embedding: Option<Vec<f32>>,
    ) -> String {
        let fragments = if let Some(q) = query {
            self.search(q, Some(agent_id), 10, embedding).await.unwrap_or_default()
        } else {
            self.recent(agent_id, 10).await.unwrap_or_default()
        };

        if fragments.is_empty() {
            return String::new();
        }

        let mut ctx = String::from("## Relevant Memories\n");
        for f in &fragments {
            let content_preview = if f.content.len() > 300 {
                format!("{}...", &f.content[..300])
            } else {
                f.content.clone()
            };
            let line = format!(
                "- [{}] ({}): {}\n",
                f.context,
                f.created_at.0.format("%Y-%m-%d %H:%M"),
                content_preview
            );
            if ctx.len() + line.len() > max_chars {
                break;
            }
            ctx.push_str(&line);
        }
        ctx
    }
}
