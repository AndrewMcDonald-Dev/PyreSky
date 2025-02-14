use futures_util::StreamExt;
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{debug, error, info};

pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
}

pub async fn send_message_on_receive(io: SocketIo) {
    let url =
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post";

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    while let Some(msg) = ws_stream.next().await {
        let decoded_msg = serde_json::from_str::<crate::Message>(msg.unwrap().to_text().unwrap());

        match decoded_msg {
            Ok(msg) => {
                debug!("Received message: {:?}", msg);

                io.broadcast().emit("message", &msg).await.unwrap();
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
