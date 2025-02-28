use crate::music_thread::music_thread::WSMusicThreadResponse;
use crate::server::listener::manage_client_request_if_valid;
use futures::{SinkExt, StreamExt};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::thread;
use std::thread::JoinHandle;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

// Server Thread Communications
#[derive(Clone, Debug)]
pub enum WSResponse {
    FromMusicThread(WSMusicThreadResponse),
    FromWSClient(WSClientResponse),
}
#[derive(Clone, Debug)]
enum WSClientResponse {
    SimpleResponse(Message),
}
pub fn wrap_ws_response_from_music_thread(
    ws_music_thread_response: WSMusicThreadResponse,
) -> WSResponse {
    WSResponse::FromMusicThread(ws_music_thread_response)
}
pub fn wrap_ws_response_from_ws_client_message(message: Message) -> WSResponse {
    WSResponse::FromWSClient(WSClientResponse::SimpleResponse(message))
}

// TODO: Adjust BUFFER_SIZE
const BUFFER_SIZE: usize = 32;
type BroadcastSenderToServerThread = broadcast::Sender<WSResponse>;
pub fn main_server_thread() -> (JoinHandle<()>, BroadcastSenderToServerThread) {
    let (tx_to_web_server, rx_to_web_server) = broadcast::channel::<WSResponse>(BUFFER_SIZE);
    // TODO: It is wise to clone this TX?
    let tx_to_web_server_clone = tx_to_web_server.clone();
    let thread = thread::spawn(move || {
        main_server(tx_to_web_server_clone, rx_to_web_server);
    });
    (thread, tx_to_web_server)
}

#[tokio::main]
pub async fn main_server(
    tx_to_web_server: BroadcastSenderToServerThread,
    mut rx_to_web_server: Receiver<WSResponse>,
) {
    // Get the address to bind to
    /*let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8666".to_string());*/
    let addr = "127.0.0.1:8666".to_string();
    let addr: SocketAddr = addr.parse().expect("Invalid address");

    // Create the TCP listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    while let Ok((stream, socket_addr)) = listener.accept().await {
        println!("Connection with {:?}", socket_addr);

        let tx_to_web_server_for_him = tx_to_web_server.clone();
        let mut rx_to_web_server_for_him = rx_to_web_server.resubscribe();

        tokio::spawn(async move {
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
            // Sending first message to client.
            sender.send("Welcome!".into()).await.unwrap();

            // TODO: Try to use "sender" in multiple threads
            // let arc_sender = Arc::new(Mutex::new(sender));
            // Update clients

            let thread_connection_recv_from_outside = tokio::spawn(async move {
                loop {
                    let message = rx_to_web_server_for_him.recv().await;
                    match message {
                        Ok(message) => {
                            let payload = match message {
                                WSResponse::FromMusicThread(music_thread_response) => {
                                    match music_thread_response {
                                        WSMusicThreadResponse::SongStarted => "Song started".into(),
                                        WSMusicThreadResponse::SongPlayingUpdate => {
                                            "Song playing update".into()
                                        }
                                        WSMusicThreadResponse::SongEnded => "Song ended".into(),
                                    }
                                }
                                WSResponse::FromWSClient(client_response) => {
                                    match client_response {
                                        WSClientResponse::SimpleResponse(message) => message,
                                    }
                                }
                            };
                            if let Err(e) = sender.send(payload).await {
                                println!("Error sending message: {}", e);
                            }
                        }
                        Err(_) => {}
                    }
                }
            });

            // Handle incoming messages
            // let mut sender_2 = arc_sender.clone();
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let request = text.chars().collect::<String>();
                        let client_response = manage_client_request_if_valid(request);
                        let message = match client_response {
                            Some(client_response) => Message::Text(client_response.into()),
                            None => Message::Text("KO".into()),
                        };
                        tx_to_web_server_for_him
                            .send(wrap_ws_response_from_ws_client_message(message))
                            .unwrap();
                        /*// let mut sender = sender_2.lock().unwrap();
                        if let Err(e) = sender.send(message).await {
                            println!("Error sending message: {}", e);
                        }*/
                    }
                    Ok(Message::Close(_)) => {
                        // TODO: Abort is best here?
                        // thread_connection_recv_from_outside.abort();
                        break;
                    }
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error processing message: {}", e);
                        break;
                    }
                }
            }

            // TODO: Abort is best here?
            // thread_connection_recv_from_outside.abort();

            // println!("Closing connection");
        });
    }
}
