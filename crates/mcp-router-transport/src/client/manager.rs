use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub struct ClientManager;

impl ClientManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn spawn_client(&self, command: &str, args: &[String]) -> Result<()> {
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

        tracing::info!("Successfully spawned client: {} {:?}", command, args);
        Ok(())
    }
}
