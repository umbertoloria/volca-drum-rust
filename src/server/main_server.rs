use crate::server::listener::listener_manage;
use futures::{SinkExt, StreamExt};
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

// TODO: Adjust BUFFER_SIZE
const BUFFER_SIZE: usize = 32;

#[tokio::main]
pub async fn main_server() {
    // Get the address to bind to
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8666".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid address");

    // Create the TCP listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    while let Ok((stream, socket_addr)) = listener.accept().await {
        println!("{:?}", socket_addr);
        let (tx, mut rx) = mpsc::channel(BUFFER_SIZE);
        // Spawn a new task for each connection
        tokio::spawn(handle_connection(stream, tx));
    }
}

pub enum WSThreadComm {}

async fn handle_connection(stream: TcpStream, tx: Sender<WSThreadComm>) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let request = text.chars().collect::<String>();
                let response = listener_manage(request);
                let message = Message::Text(response.into());
                if let Err(e) = sender.send(message).await {
                    println!("Error sending message: {}", e);
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                println!("Error processing message: {}", e);
                break;
            }
        }
    }

    // println!("Closing connection");
}
