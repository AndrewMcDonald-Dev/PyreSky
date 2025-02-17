use futures_util::StreamExt;
use tokio::sync::broadcast;
use tracing::{debug, error};

use crate::{Kind, Operation};

pub async fn send_message_on_receive(tx: broadcast::Sender<String>, templates: tera::Tera) {
    let url =
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post";

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();

    while let Some(msg) = ws_stream.next().await {
        let decoded_msg = serde_json::from_str::<crate::Message>(msg.unwrap().to_text().unwrap());

        match decoded_msg {
            Ok(msg) => {
                debug!("Received message: {:?}", msg);
                let mut context = tera::Context::new();

                context.insert("did", &msg.did);

                if let Kind::Commit { commit } = msg.kind {
                    if let Operation::Create {
                        record,
                        collection: _,
                        rkey,
                        cid: _,
                    } = commit.operation
                    {
                        context.insert("message", &record.text);
                        context.insert("cid", &rkey);

                        let message = templates.render("message.html", &context).unwrap();

                        if let Err(e) = tx.send(message) {
                            error!("Error sending message: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
