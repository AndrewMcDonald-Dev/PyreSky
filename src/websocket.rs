use socketioxide::{extract::SocketRef, SocketIo};
use tracing::{debug, error, info};

use crate::ConnectionHandle;

pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
}

pub async fn send_message_on_receive(io: SocketIo, mut connection: ConnectionHandle) {
    while connection.can_read() {
        let msg = connection.read().expect("Failed to read from connection.");
        let decoded_msg = serde_json::from_str::<crate::Message>(&msg.to_string());

        match decoded_msg {
            Ok(msg) => {
                debug!("Received message: {:?}", msg);

                io.emit("message", &msg).await.unwrap();
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
