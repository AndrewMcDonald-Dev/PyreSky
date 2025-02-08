use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub async fn websocket_server(listener: TcpListener, tx: broadcast::Sender<String>) {
    let tx = tx.clone();
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            let tx = tx.clone();
            tokio::spawn(async move {
                handle_client(socket, tx).await;
            });
        }
    });
}

async fn handle_client(socket: tokio::net::TcpStream, tx: broadcast::Sender<String>) {
    let ws_stream = accept_async(socket).await.unwrap();

    let mut rx = tx.subscribe();

    let (mut ws_sender, _ws_receiver) = ws_stream.split();

    loop {
        if let Ok(msg) = rx.recv().await {
            if let Err(_e) = ws_sender.send(Message::text(msg)).await {
                break;
            }
        }
    }
}
