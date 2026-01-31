use anyhow::Result;
use async_trait::async_trait;
use mcp_router_core::{ServerId, Tool};

#[async_trait]
pub trait Router: Send + Sync {
    /// Given a prompt, return the best matching tool definition (if any)
    async fn resolve(&self, prompt: &str) -> Result<Option<(ServerId, Tool)>>;

    /// Add a tool to the router's index
    async fn add_tool(&self, server_id: &str, tool: Tool) -> Result<()>;
}

pub mod keyword;
pub use keyword::KeywordRouter;
