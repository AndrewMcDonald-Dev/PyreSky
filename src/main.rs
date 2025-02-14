use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension,
};
use firesky::{on_connect, send_message_on_receive};
use lazy_static::lazy_static;
use socketioxide::SocketIo;
use tera::Tera;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
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
    // Needed for websockets
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider.");

    // Lgging
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // Biuld SocketIO layer
    let (layer, io) = SocketIo::builder().max_buffer_size(1024).build_layer();

    io.ns("/", on_connect);

    tokio::spawn(send_message_on_receive(io, TEMPLATES.clone()));

    // Build Axum app
    let app = axum::Router::new().route("/", get(handler)).layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer)
            .layer(Extension(TEMPLATES.clone())),
    );

    // Start server
    info!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
