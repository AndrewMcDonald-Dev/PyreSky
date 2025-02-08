use firesky::{stream::websocket_server, Connection, Message};
use tokio::{net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel(10);
    websocket_server(listener, tx.clone()).await;

    // Define funtion to handle messages
    let handle_message = |msg: Message, tx: broadcast::Sender<String>| {
        //println!("Received message: {:?}\n", msg);
        tx.send(serde_json::to_string(&msg).unwrap()).unwrap();
    };

    let handle_error = |e: Box<dyn std::error::Error>| {
        println!("Error: {:?}\n", e);
    };

    // Create a new connection
    let mut connection = Connection::new(handle_message, handle_error, tx);

    // Run the connection
    connection.run()?;

    Ok(())
}
