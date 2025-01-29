use firesky::{Connection, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define funtion to handle messages
    let handle_message = |msg: Message| {
        println!("Received message: {:?}\n", msg);
    };

    let handle_error = |e: Box<dyn std::error::Error>| {
        println!("Error: {:?}\n", e);
    };

    // Create a new connection
    let mut connection = Connection::new(handle_message, handle_error);

    // Run the connection
    connection.run()?;

    Ok(())
}
