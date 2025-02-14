use axum::routing::get;
use firesky::{
    Connection, {on_connect, send_message_on_receive},
};
use socketioxide::SocketIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Create a new connection
    let connection = Connection::new();

    // Biuld SocketIO layer
    let (layer, io) = SocketIo::builder().max_buffer_size(1024).build_layer();

    io.ns("/", on_connect);

    tokio::spawn(send_message_on_receive(io, connection.connection));

    // Build Axum app
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    // Start server
    info!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
