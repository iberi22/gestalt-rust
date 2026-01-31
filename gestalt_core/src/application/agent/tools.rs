use synapse_agentic::prelude::*;
use crate::ports::outbound::repo_manager::{VectorDb, RepoManager};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::context::scanner;
use std::path::Path;

pub async fn create_gestalt_tools(
    _repo_manager: Arc<dyn RepoManager>,
    vector_db: Arc<dyn VectorDb>,
) -> ToolRegistry {
    let registry = ToolRegistry::new();
    registry.register_tool(ScanWorkspaceTool).await;
    registry.register_tool(SearchCodeTool { vector_db }).await;
    registry
}

pub struct ScanWorkspaceTool;

#[async_trait]
impl Tool for ScanWorkspaceTool {
    fn name(&self) -> &str { "scan_workspace" }
    fn description(&self) -> &str { "Generates a directory tree of the current project workspace." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "depth": { "type": "integer", "default": 2 }
            }
        })
    }

    async fn call(&self, _ctx: &dyn ToolContext, args: Value) -> anyhow::Result<Value> {
        let depth = args.get("depth").and_then(|v| v.as_u64()).unwrap_or(2) as usize;
        let root = Path::new(".");
        let tree = scanner::generate_directory_tree(root, depth);
        Ok(json!({ "tree": tree }))
    }
}

pub struct SearchCodeTool {
    vector_db: Arc<dyn VectorDb>,
}

#[async_trait]
impl Tool for SearchCodeTool {
    fn name(&self) -> &str { "search_code" }
    fn description(&self) -> &str { "Search for similar code fragments in the vector database." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string" },
                "limit": { "type": "integer", "default": 5 }
            },
            "required": ["query"]
        })
    }

    async fn call(&self, _ctx: &dyn ToolContext, args: Value) -> anyhow::Result<Value> {
        let _query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

        // In a real implementation, we'd embed the query first.
        // For now, using a dummy zero vector just to verify trait linkage
        let similar = self.vector_db.search_similar("code", vec![0.0; 384], limit).await?;

        Ok(json!({ "results": similar }))
    }
}
