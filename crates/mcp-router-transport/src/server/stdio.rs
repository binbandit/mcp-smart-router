use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::server::handler::RouterServerHandler;

pub struct StdioServer {
    handler: RouterServerHandler,
}

impl StdioServer {
    pub fn new(handler: RouterServerHandler) -> Self {
        Self { handler }
    }

    pub async fn run(&self) -> Result<()> {
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            tracing::info!("Server received: {}", line);
        }
        Ok(())
    }
}
