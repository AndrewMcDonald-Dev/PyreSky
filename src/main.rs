use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
    routing::{any, get},
    Extension,
};
use firesky::send_message_on_receive;
use lazy_static::lazy_static;
use tera::Tera;
use tokio::{net::TcpListener, sync::broadcast};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*.html";
        Tera::new(source).unwrap()
    };
}

async fn handler() -> impl IntoResponse {
    let page_content = TEMPLATES
        .render("index.html", &tera::Context::new())
        .unwrap();

    Html(page_content)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Needed for websocket connection to bsky
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider.");

    // Logging
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Create broadcast channel
    let (tx, _rx) = broadcast::channel(100);

    // Start websocket receiver
    tokio::spawn(send_message_on_receive(tx.clone(), TEMPLATES.clone()));

    // Build Axum app
    let app = axum::Router::new()
        .route("/", get(handler))
        .route("/ws", any(ws_handler))
        .with_state(tx)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(Extension(TEMPLATES.clone())),
        );

    // Start server
    info!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(tx): State<broadcast::Sender<String>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    info!("New connection from {:?}", addr);

    ws.on_upgrade(move |socket| handle_socket(socket, tx, addr))
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<String>, addr: SocketAddr) {
    let mut rx = tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if (socket.send(Message::text(msg)).await).is_ok() {
            debug!("Message sent to {:?}", addr);
            continue;
        }
        break;
    }
}
