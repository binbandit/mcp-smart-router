use anyhow::{Context, Result};
use hashbrown::HashMap;
use serde_json::Value;
use std::{process::Stdio, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::{RwLock, mpsc, oneshot},
};

use crate::json_rpc::{JsonRpcMessage, JsonRpcRequest};

pub struct ClientManager {
    clients: RwLock<HashMap<String, Arc<DownstreamClient>>>,
}

pub struct DownstreamClient {
    pub id: String,
    pub tx: mpsc::Sender<JsonRpcMessage>,
    pub pending_requests: Arc<RwLock<HashMap<String, oneshot::Sender<Result<Value>>>>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn list_clients(&self) -> HashMap<String, Arc<DownstreamClient>> {
        self.clients.read().await.clone()
    }

    pub async fn spawn_client(&self, id: String, command: &str, args: &[String]) -> Result<()> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn downstream MCP server")?;

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");

        let (tx, mut rx) = mpsc::channel::<JsonRpcMessage>(32);
        let pending_requests: Arc<RwLock<HashMap<String, oneshot::Sender<Result<Value>>>>> =
            Arc::new(RwLock::new(HashMap::new()));

        let pending = pending_requests.clone();

        // placeholer reader loop
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(msg) = serde_json::from_str::<JsonRpcMessage>(&line) {
                    match msg {
                        JsonRpcMessage::Response(res) => {
                            if let Value::String(id_str) = res.id {
                                if let Some(sender) = pending.write().await.remove(&id_str) {
                                    let _ = sender.send(Ok(res.result.unwrap_or(Value::Null)));
                                }
                            }
                        }
                        _ => {} // Ignore requests/notifications from server for now
                    }
                } else {
                    tracing::warn!("Failed to parse line from child: {}", line);
                }
            }
        });

        // TODO: finish this
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = stdin.write_all(json.as_bytes()).await;
                    let _ = stdin.write_all(b"\n").await;
                }
            }
        });

        let client = Arc::new(DownstreamClient {
            id: id.clone(),
            tx,
            pending_requests, // Use the shared Arc
        });

        self.clients.write().await.insert(id.clone(), client);
        tracing::info!("Successfully spawned client: {} {:?}", command, args);
        Ok(())
    }

    pub async fn list_tools(&self, server_id: &str) -> Result<Value> {
        let clients = self.clients.read().await;
        let client = clients.get(server_id).context("Server not connected")?;

        let id_val = Value::String(uuid::Uuid::new_v4().to_string());

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: id_val.clone(),
            method: "tools/list".to_string(),
            params: None,
        };

        // Register pending request
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = client.pending_requests.write().await;
            pending.insert(id_val.as_str().unwrap().to_string(), tx);
        }

        // Send
        if let Err(_) = client.tx.send(JsonRpcMessage::Request(request)).await {
            return Err(anyhow::anyhow!("Failed to send to client channel"));
        }

        // Wait for response
        match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(res)) => Ok(res?),
            Ok(Err(_)) => Err(anyhow::anyhow!("Sender dropped")),
            Err(_) => {
                let mut pending = client.pending_requests.write().await;
                pending.remove(id_val.as_str().unwrap());
                Err(anyhow::anyhow!("List tools timed out after 30s"))
            }
        }
    }

    pub async fn call_tool(
        &self,
        server_id: &str,
        tool_name: &str,
        arguments: Value
    ) -> Result<Value> {
        let clients = self.clients.read().await;
        let client = clients.get(server_id).context("Server not connected")?;

        let id_val = Value::String(uuid::Uuid::new_v4().to_string());

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: id_val.clone(),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": tool_name,
                "arguments": arguments
            })),
        };

        // Register pending
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = client.pending_requests.write().await;
            pending.insert(id_val.as_str().unwrap().to_string(), tx);
        }

        // Send
        if let Err(_) = client.tx.send(JsonRpcMessage::Request(request)).await {
            return Err(anyhow::anyhow!("Failed to send to client channel"));
        }

        // Wait
        match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(res)) => Ok(res?),
            Ok(Err(_)) => Err(anyhow::anyhow!("Sender dropped")),
            Err(_) => {
                let mut pending = client.pending_requests.write().await;
                pending.remove(id_val.as_str().unwrap());
                Err(anyhow::anyhow!("Call tool timed out after 30s"))
            }
        }
    }
}
