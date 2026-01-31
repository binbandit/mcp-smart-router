use crate::Router;
use anyhow::Result;
use async_trait::async_trait;
use hashbrown::HashMap;
use mcp_router_core::{ServerId, Tool};
use tokio::sync::RwLock;

pub struct KeywordRouter {
    tools: RwLock<HashMap<ServerId, Vec<Tool>>>,
}

impl KeywordRouter {
    pub fn new() -> Self {
        Self {
            tools: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Router for KeywordRouter {
    async fn resolve(&self, prompt: &str) -> Result<Option<(ServerId, Tool)>> {
        let tools = self.tools.read().await;

        for (server_id, tool_list) in tools.iter() {
            for tool in tool_list {
                if prompt.contains(&tool.name) {
                    return Ok(Some((server_id.clone(), tool.clone())));
                }
            }
        }

        Ok(None)
    }

    async fn add_tool(&self, server_id: &str, tool: Tool) -> Result<()> {
        let mut tools = self.tools.write().await;
        tools
            .entry(server_id.to_string())
            .or_insert_with(Vec::new)
            .push(tool);

        Ok(())
    }
}
