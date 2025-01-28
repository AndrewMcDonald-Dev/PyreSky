use firesky::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider.");
    let mut connection = tungstenite::connect(
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post",
    )?
    .0;

    while connection.can_read() {
        let msg = connection.read()?;
        let decoded_msg = serde_json::from_str::<Message>(&msg.to_string());

        match decoded_msg {
            Ok(_msg) => {
                //println!("Received message: {:?}\n", msg);
            }
            Err(e) => {
                println!("Messag: {:?}", msg);
                println!("Error: {:?}\n", e);
            }
        }
    }

    Ok(())
}
