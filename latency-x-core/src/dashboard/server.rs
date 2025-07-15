use crate::dashboard::events::DashboardEvent;
use crate::persistence::db::DatabaseManager;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<DashboardEvent>,
    db: Arc<DatabaseManager>,
}

pub async fn start_dashboard_server(tx: broadcast::Sender<DashboardEvent>, db: Arc<DatabaseManager>) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app_state = AppState { tx, db };

    let app = Router::new()
        .route("/api/health", get(|| async { "OK" }))
        .route("/ws", get(websocket_handler))
        .route("/api/trades", get(get_trades_handler))
        .fallback_service(ServeDir::new("../latency-x-dashboard/dist"))
        .with_state(Arc::new(app_state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_trades_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.db.get_trades().await {
        Ok(trades) => Json(trades).into_response(),
        Err(e) => {
            tracing::error!("Failed to get trades: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[axum::debug_handler]
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();
    let mut rx = state.tx.subscribe();

    // Forward broadcast messages to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let msg = serde_json::to_string(&event).unwrap();
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle messages from the client (if any)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // We don't expect messages from the client in this app
        }
    });

    // If either task finishes, abort the other
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
} 