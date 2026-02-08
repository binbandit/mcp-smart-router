use crate::Router;
use anyhow::Result;
use arrow_array::{
    ArrayRef, FixedSizeListArray, RecordBatch, RecordBatchIterator, StringArray, types::Float32Type,
};
use arrow_schema::{DataType, Field, Schema};
use async_trait::async_trait;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use lancedb::{Connection, Table, connect};
use mcp_router_core::{ServerId, Tool};
use std::sync::Arc;

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_tool(&self, server_id: &str, tool: Tool) -> Result<()>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<(ServerId, Tool)>>;
}

pub struct LanceDbStore {
    table: Table,
    embedding_modle: TextEmbedding,
}

impl LanceDbStore {
    pub async fn new(uri: &str) -> Result<Self> {
        let conn = connect(uri).execute().await?;
        let embedding_model = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::AllMiniLML6V2,
            show_download_progress: true,
            ..Default::default()
        })?;

        Err(anyhow::anyhow!(
            "LanceDB initialization not fully implemented"
        ))
    }
}

#[async_trait]
impl VectorStore for LanceDbStore {
    async fn add_tool(&self, server_id: &str, tool: Tool) -> Result<()> {
        Ok(())
    }

    async fn search(&self, query: &str, limit: uzize) -> Result<Vec<(ServerId, Tool)>> {
        Ok(Vec![])
    }
}
