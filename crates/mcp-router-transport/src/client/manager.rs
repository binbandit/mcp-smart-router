use anyhow::{Context, Result};
use hashbrown::HashMap;
use std::{process::Stdio, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::RwLock,
};

pub struct ClientManager {
    clients: RwLock<HashMap<String, Arc<DownstreamClient>>>,
}

pub struct DownstreamClient {
    pub id: String,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
        }
    }

    pub async fn spawn_client(&self, id: String, command: &str, args: &[String]) -> Result<()> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn downstream MCP server")?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");

        // placeholer reader loop
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                tracing::info!("Received from child: {}", line);
            }
        });

        // TODO: finish this
        tokio::spawn(async move {
            let _stdin = stdin;
        });

        let client = Arc::new(DownstreamClient { id: id.clone() });
        self.clients.write().await.insert(id.clone(), client);

        tracing::info!("Successfully spawned client: {} {:?}", command, args);
        Ok(())
    }
}
