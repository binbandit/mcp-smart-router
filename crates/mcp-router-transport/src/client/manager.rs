use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::process::Command;

pub struct ClientManager;

impl ClientManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn spawn_client(&self, command: &str, args: &[String]) -> Result<()> {
        let _child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn downstream MCP server")?;

        tracing::info!("Successfully spawned client: {} {:?}", command, args);
        Ok(())
    }
}
