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
            if let Ok(msg) = serde_json::from_str::<crate::json_rpc::JsonRpcMessage>(&line) {
                let _ = &self.handler;
            }
        }
        Ok(())
    }
}
