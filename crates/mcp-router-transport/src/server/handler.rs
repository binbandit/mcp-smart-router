use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

use crate::client::manager::ClientManager;

#[derive(Clone)]
pub struct RouterServerHandler {
    client_manager: Arc<ClientManager>,
}

impl RouterServerHandler {
    pub fn new(client_manager: Arc<ClientManager>) -> Self {
        Self { client_manager }
    }

    pub async fn handle_list_tools(&self) -> Result<Value> {
        let mut all_tools = Vec::new();
        let clients = self.client_manager.list_clients().await;

        for (server_id, _client) in clients {
            if let Ok(tools) = self.client_manager.list_tools(&server_id).await {
                if let Some(tool_list) = tools["result"]["tools"].as_array() {
                    for tool in tool_list {
                        all_tools.push(tool.clone());
                    }
                } else if let Some(tool_list) = tools["tools"].as_array() {
                    for tool in tool_list {
                        let mut tool = tool.clone();
                        if let Some(obj) = tool.as_object_mut() {
                            if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                                obj.insert(
                                    "name".to_string(),
                                    serde_json::json!(format!("{}__{}", server_id, name)),
                                );
                            }
                        }
                        all_tools.push(tool);
                    }
                }
            }
        }

        Ok(serde_json::json!({
            "tools": all_tools
        }))
    }

    pub async fn handle_call_tool(&self, name: &str, args: Value) -> Result<Value> {
        Err(anyhow::anyhow!("Tool routing not implemnented yet"))
    }
}
