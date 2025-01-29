use crate::Message;

pub struct Connection {
    pub connection: ConnectionHandle,
    handle_message: fn(Message),
    handle_error: fn(Box<dyn std::error::Error>),
}

type ConnectionHandle =
    tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;

impl Connection {
    pub fn new(handle_message: fn(Message), handle_error: fn(Box<dyn std::error::Error>)) -> Self {
        let connection = Connection::get_connection().unwrap();

        Self {
            connection,
            handle_message,
            handle_error,
        }
    }

    pub fn get_connection() -> Result<ConnectionHandle, Box<dyn std::error::Error>> {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider.");

        let connection = tungstenite::connect(
            "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post",
        )?
        .0;

        Ok(connection)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.connection.can_read() {
            let msg = self.connection.read()?;
            let decoded_msg = serde_json::from_str::<Message>(&msg.to_string());

            match decoded_msg {
                Ok(msg) => {
                    (self.handle_message)(msg);
                }
                Err(e) => {
                    (self.handle_error)(Box::new(e));
                }
            }
        }

        Ok(())
    }
}
