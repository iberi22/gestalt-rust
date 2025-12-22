//! SurrealDB client implementation

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use tracing::{info, debug};

/// SurrealDB client wrapper for timeline operations.
#[derive(Clone)]
pub struct SurrealClient {
    db: Arc<Surreal<Any>>,
}

impl SurrealClient {
    /// Connect to SurrealDB using environment variables.
    ///
    /// Environment variables:
    /// - SURREAL_URL: URL (default: mem:// for in-memory)
    ///   - mem:// for in-memory database
    ///   - file://path for persistent RocksDB
    ///   - ws://host:port for WebSocket
    /// - SURREAL_USER: Username (default: root)
    /// - SURREAL_PASS: Password (default: root)
    /// - SURREAL_NS: Namespace (default: gestalt)
    /// - SURREAL_DB: Database (default: timeline)
    pub async fn connect() -> Result<Self> {
        let url = std::env::var("SURREAL_URL").unwrap_or_else(|_| "mem://".to_string());
        let user = std::env::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string());
        let pass = std::env::var("SURREAL_PASS").unwrap_or_else(|_| "root".to_string());
        let ns = std::env::var("SURREAL_NS").unwrap_or_else(|_| "gestalt".to_string());
        let db_name = std::env::var("SURREAL_DB").unwrap_or_else(|_| "timeline".to_string());

        info!("Connecting to SurrealDB at {}", url);

        let db = surrealdb::engine::any::connect(&url)
            .await
            .context("Failed to connect to SurrealDB")?;

        // Only sign in for remote connections
        if url.starts_with("ws://") || url.starts_with("wss://") || url.starts_with("http://") || url.starts_with("https://") {
            db.signin(Root {
                username: &user,
                password: &pass,
            })
            .await
            .context("Failed to authenticate with SurrealDB")?;
        }

        db.use_ns(&ns).use_db(&db_name).await?;

        info!("Connected to SurrealDB: {}:{}", ns, db_name);

        // Initialize schema
        Self::init_schema(&db).await?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Initialize database schema.
    async fn init_schema(db: &Surreal<Any>) -> Result<()> {
        debug!("Initializing database schema");

        // Create tables with indexes
        db.query(
            r#"
            DEFINE TABLE timeline_events SCHEMAFULL;
            DEFINE FIELD timestamp ON timeline_events TYPE string;
            DEFINE FIELD agent_id ON timeline_events TYPE string;
            DEFINE FIELD event_type ON timeline_events TYPE string;
            DEFINE FIELD project_id ON timeline_events TYPE option<string>;
            DEFINE FIELD task_id ON timeline_events TYPE option<string>;
            DEFINE FIELD payload ON timeline_events TYPE option<object>;
            DEFINE FIELD metadata ON timeline_events TYPE option<object>;
            DEFINE INDEX idx_timestamp ON timeline_events FIELDS timestamp;
            DEFINE INDEX idx_project ON timeline_events FIELDS project_id;
            DEFINE INDEX idx_agent ON timeline_events FIELDS agent_id;

            DEFINE TABLE projects SCHEMAFULL;
            DEFINE FIELD name ON projects TYPE string;
            DEFINE FIELD status ON projects TYPE string;
            DEFINE FIELD priority ON projects TYPE int;
            DEFINE FIELD created_at ON projects TYPE string;
            DEFINE FIELD updated_at ON projects TYPE string;
            DEFINE FIELD created_by ON projects TYPE string;
            DEFINE INDEX idx_name ON projects FIELDS name UNIQUE;

            DEFINE TABLE tasks SCHEMAFULL;
            DEFINE FIELD project_id ON tasks TYPE string;
            DEFINE FIELD description ON tasks TYPE string;
            DEFINE FIELD status ON tasks TYPE string;
            DEFINE FIELD created_at ON tasks TYPE string;
            DEFINE FIELD updated_at ON tasks TYPE string;
            DEFINE FIELD completed_at ON tasks TYPE option<string>;
            DEFINE FIELD created_by ON tasks TYPE string;
            DEFINE FIELD executed_by ON tasks TYPE option<string>;
            DEFINE INDEX idx_project_id ON tasks FIELDS project_id;
            DEFINE INDEX idx_status ON tasks FIELDS status;
            "#,
        )
        .await
        .context("Failed to initialize schema")?;

        debug!("Schema initialized successfully");
        Ok(())
    }

    /// Create a record in a table.
    pub async fn create<T: Serialize + DeserializeOwned>(&self, table: &str, data: &T) -> Result<T> {
        let results: Vec<T> = self.db.create(table).content(data).await?;
        results.into_iter().next().ok_or_else(|| anyhow::anyhow!("Failed to create record"))
    }

    /// Select all records from a table.
    pub async fn select_all<T: DeserializeOwned>(&self, table: &str) -> Result<Vec<T>> {
        let results: Vec<T> = self.db.select(table).await?;
        Ok(results)
    }

    /// Select a record by ID.
    pub async fn select_by_id<T: DeserializeOwned>(&self, table: &str, id: &str) -> Result<Option<T>> {
        let result: Option<T> = self.db.select((table, id)).await?;
        Ok(result)
    }

    /// Update a record.
    pub async fn update<T: Serialize + DeserializeOwned>(&self, table: &str, id: &str, data: &T) -> Result<T> {
        let result: Option<T> = self.db.update((table, id)).content(data).await?;
        result.ok_or_else(|| anyhow::anyhow!("Failed to update record"))
    }

    /// Execute a raw query.
    pub async fn query(&self, query: &str) -> Result<surrealdb::Response> {
        let result = self.db.query(query).await?;
        Ok(result)
    }

    /// Execute a query with bindings.
    pub async fn query_with<T: DeserializeOwned>(
        &self,
        query: &str,
        bindings: impl Serialize,
    ) -> Result<Vec<T>> {
        let mut response = self.db.query(query).bind(bindings).await?;
        let results: Vec<T> = response.take(0)?;
        Ok(results)
    }
}
