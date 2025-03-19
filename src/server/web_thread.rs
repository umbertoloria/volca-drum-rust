use crate::music_thread::music_thread_comm::MusicThreadCommSender;
use crate::server::server_request_manager::{
    ServerRequestManager, WS_CLIENT_REQUEST_READ_MUSIC_LIBRARY,
};
use crate::server::web_thread_comm::{
    WSClientResponse, WSMusicThreadResponse, WSResponse, WebThreadCommReceiver, WebThreadCommSender,
};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::thread;
use std::thread::JoinHandle;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

pub fn web_thread(
    web_thread_comm_receiver: WebThreadCommReceiver,
    web_thread_comm_sender: WebThreadCommSender,
    music_thread_comm_sender: MusicThreadCommSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        main_server(
            web_thread_comm_receiver,
            web_thread_comm_sender,
            music_thread_comm_sender,
        );
    })
}

#[tokio::main]
pub async fn main_server(
    web_thread_comm_receiver: WebThreadCommReceiver,
    web_thread_comm_sender: WebThreadCommSender,
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

        let web_thread_comm_sender_for_him = web_thread_comm_sender.clone();
        let mut web_thread_comm_receiver_for_him = web_thread_comm_receiver.resubscribe();
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
            {
                // First message: Welcome!
                sender.send("Welcome!".into()).await.unwrap();
                // Second message: Music Library Songs
                let message =
                    server_request_manager.manage(WS_CLIENT_REQUEST_READ_MUSIC_LIBRARY.into());
                web_thread_comm_sender_for_him.notify_from_client_thread_a_response(message);
            }

            // Update clients
            let thread_connection_recv_from_outside = tokio::spawn(async move {
                loop {
                    let message = web_thread_comm_receiver_for_him.async_new_message().await;
                    match message {
                        Ok(message) => {
                            let payload = match message {
                                WSResponse::FromMusicThread(music_thread_response) => {
                                    match music_thread_response {
                                        WSMusicThreadResponse::SongStarted => "Song started".into(),
                                        WSMusicThreadResponse::SongPlayingUpdate(
                                            tempo_snapshot,
                                        ) => {
                                            // println!("{:?}", tempo_snapshot);
                                            // TODO: Use JSON parser
                                            format!(
                                                "{{\"cur_bar\":{},\"cur_quarter\":\"{}\",\"cur_1_8\":\"{}\",\"cur_1_16\":\"{}\",\"section_bar_first\":\"{}\",\"section_bar_last\":\"{}\"}}",
                                                tempo_snapshot.cur_bar,
                                                tempo_snapshot.cur_quarter,
                                                tempo_snapshot.cur_1_8,
                                                tempo_snapshot.cur_1_16,
                                                tempo_snapshot.section_bar_first,
                                                tempo_snapshot.section_bar_last,
                                            )
                                            .into()
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
                        web_thread_comm_sender_for_him
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
