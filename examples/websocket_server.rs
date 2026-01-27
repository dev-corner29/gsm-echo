
use std::{env, io::Error};

use futures_util::{StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (_, mut read) = ws_stream.split();

    // We should not forward messages other than text or binary.
    while let Ok(msg_opt) = read.try_next().await {
        if let Some(msg) = msg_opt {
            match msg {
                Message::Text(text) => {
                    println!("{} sent: \"{}\"", addr.to_string(), text);
                }
                _ => {
                    println!("Wrong type of message received.");
                }
            }
        }
    }
}