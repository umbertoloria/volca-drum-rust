use crate::music_thread::music_thread_comm::MusicThreadCommSender;
use crate::server::main_server_comm::{
    MainThreadCommReceiver, MainThreadCommSender, WSClientResponse, WSMusicThreadResponse,
    WSResponse,
};
use crate::server::server_request_manager::ServerRequestManager;
use crate::thread_comm::thread_comm::ThreadCommSenderWrapperTrait;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::thread;
use std::thread::JoinHandle;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

pub fn main_server_thread(
    main_thread_comm_receiver: MainThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
    music_thread_comm_sender: MusicThreadCommSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        main_server(
            main_thread_comm_receiver,
            main_thread_comm_sender,
            music_thread_comm_sender,
        );
    })
}

#[tokio::main]
pub async fn main_server(
    main_thread_comm_receiver: MainThreadCommReceiver,
    main_thread_comm_sender: MainThreadCommSender,
    music_thread_comm_sender: MusicThreadCommSender,
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

        let main_thread_comm_sender_for_him = main_thread_comm_sender.clone();
        let mut main_thread_comm_receiver_for_him = main_thread_comm_receiver.resubscribe();
        let server_request_manager = ServerRequestManager::new(music_thread_comm_sender.clone());

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

            // Update clients
            let thread_connection_recv_from_outside = tokio::spawn(async move {
                loop {
                    let message = main_thread_comm_receiver_for_him.async_new_message().await;
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
                            send_message_to_sender(&mut sender, payload).await;
                        }
                        Err(_) => {}
                    }
                }
            });

            // Handle incoming messages
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let request = text.chars().collect::<String>();
                        let message = server_request_manager.manage(request);

                        // send_message_to_sender(&mut sender, message).await; // Direct send.
                        main_thread_comm_sender_for_him
                            .notify_from_client_thread_a_response(message);
                    }
                    Ok(Message::Close(_)) => {
                        thread_connection_recv_from_outside.abort();
                        break;
                    }
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error processing message: {}", e);
                        break;
                    }
                }
            }

            thread_connection_recv_from_outside.abort();

            // println!("Closing connection");
        });
    }
}

async fn send_message_to_sender(
    sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    payload: String,
) {
    let message = Message::Text(payload.into());
    if let Err(e) = sender.send(message).await {
        println!("Error sending message: {}", e);
    }
}
