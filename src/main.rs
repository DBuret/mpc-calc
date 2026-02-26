mod error;
mod handlers;
mod mcp;
mod state;
use crate::handlers::messages::JsonRpcRequest;
use crate::handlers::messages::tool_definition;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures::stream::{self, Stream};
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    let log_level = std::env::var("MCP_CALC_LOG").unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(log_level).init();

    let (tx, _) = broadcast::channel(100);
    let state = Arc::new(AppState::new(tx));

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/sse", get(sse_handler).post(messages_handler))
        .route("/messages", post(messages_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = std::env::var("MCP_CALC_PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse::<u16>()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🧮 MCP Math server started on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(msg) => Some((Ok(Event::default().data(msg)), rx)),
            Err(_) => None,
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::new())
}

async fn messages_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JsonRpcRequest>,
) -> impl IntoResponse {
    let tx = state.tx.clone();
    let method = payload.method.clone();
    let request_id = payload.id.clone().unwrap_or(Value::Null);

    // Handshake initialize : réponse HTTP directe (évite le timeout LM Studio)
    if method == "initialize" {
        info!("Handling 'initialize' via direct HTTP response");
        let result = mcp::handle_initialize_result();
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "result": result
        });
        return (StatusCode::OK, Json(response)).into_response();
    }

    // Autres méthodes : traitement async + envoi via SSE
    let state_clone = state.clone();
    tokio::spawn(async move {
        if request_id.is_null() && method != "notifications/initialized" {
            return;
        }

        let result = match method.as_str() {
            "tools/list" => mcp::handle_list_tools_result(),
            "tools/call" => mcp::handle_call_tool_result(
                payload.params,
                &mut *state_clone.math_state.lock().await,
            ),
            "notifications/initialized" => {
                info!("Client initialized");
                return;
            }
            other => {
                tracing::warn!(method = %other, "Method not supported");
                serde_json::json!({
                    "isError": true,
                    "content": [{ "type": "text", "text": format!("Method {} not supported", other) }]
                })
            }
        };

        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "result": result
        });

        if let Ok(json_msg) = serde_json::to_string(&response) {
            let mut delivered = false;
            for _ in 0..3 {
                if tx.send(json_msg.clone()).is_ok() {
                    delivered = true;
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            if !delivered {
                tracing::warn!("Could not deliver {} via SSE (no client connected)", method);
            }
        }
    });

    StatusCode::ACCEPTED.into_response()
}
