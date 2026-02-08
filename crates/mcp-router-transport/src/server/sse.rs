use crate::json_rpc::JsonRpcMessage;
use crate::server::handler::RouterServerHandler;
use axum::{
    Router,
    extract::{Json, State},
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
    routing::{get, post},
};
use futures::StreamExt;
use futures::stream::Stream;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub struct SseServer {
    handler: RouterServerHandler,
    tx: broadcast::Sender<String>, // For broadcasting SSE events
}

impl SseServer {
    pub fn new(handler: RouterServerHandler) -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { handler, tx }
    }

    pub async fn run(&self, port: u16) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/sse", get(sse_handler))
            .route("/message", post(message_handler))
            .with_state(Arc::new(self.tx.clone()));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        tracing::info!("SSE Server listening on port {}", port);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn sse_handler(
    State(tx): State<Arc<broadcast::Sender<String>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx);

    // Conver broadcast stream to SSE events
    let sse_stream = stream.map(|msg| match msg {
        Ok(msg) => Ok(Event::default().data(msg)),
        Err(_) => Ok(Event::default().comment("missing message")),
    });

    Sse::new(sse_stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn message_handler(
    State(tx): State<Arc<broadcast::Sender<String>>>,
    Json(msg): Json<JsonRpcMessage>,
) -> impl IntoResponse {
    tracing::info!("Received message: {:?}", msg);
    "Accepted"
}
