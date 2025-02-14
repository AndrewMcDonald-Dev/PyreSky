pub struct Connection {
    pub connection: ConnectionHandle,
}

pub type ConnectionHandle =
    tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;

impl Connection {
    pub fn new() -> Self {
        let connection = Connection::get_connection().unwrap();

        Self { connection }
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
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}
