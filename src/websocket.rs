use futures_util::StreamExt;
use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{debug, error, info};

use crate::{Kind, Operation};

pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
}

pub async fn send_message_on_receive(io: SocketIo, templates: tera::Tera) {
    let url =
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post";

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    while let Some(msg) = ws_stream.next().await {
        let decoded_msg = serde_json::from_str::<crate::Message>(msg.unwrap().to_text().unwrap());

        match decoded_msg {
            Ok(msg) => {
                debug!("Received message: {:?}", msg);

                if let Kind::Commit { commit } = msg.kind {
                    if let Operation::Create {
                        record,
                        collection: _,
                        rkey: _,
                        cid: _,
                    } = commit.operation
                    {
                        let mut context = tera::Context::new();
                        context.insert("message", &record.text);

                        let message = templates.render("message.html", &context).unwrap();

                        io.emit("message", &message).await.unwrap();
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
