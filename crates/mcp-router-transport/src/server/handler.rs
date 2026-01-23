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
        Ok(serde_json::json!({
            "tools": []
        }))
    }

    pub async fn handle_call_tool(&self, name: &str, args: Value) -> Result<Value> {
        Err(anyhow::anyhow!("Tool routing not implemnented yet"))
    }
}
