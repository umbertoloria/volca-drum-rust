use crate::music_thread::music_thread::WSMusicThreadResponse;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone, Debug)]
pub enum WSResponse {
    FromMusicThread(WSMusicThreadResponse),
    FromWSClient(WSClientResponse),
}
#[derive(Clone, Debug)]
pub enum WSClientResponse {
    SimpleResponse(String),
}

pub type BroadcastReceiverToServerThread = Receiver<WSResponse>;
pub fn create_channel_for_server_thread() -> (MainThreadCommSender, BroadcastReceiverToServerThread)
{
    // TODO: Adjust BUFFER_SIZE
    const BUFFER_SIZE: usize = 32;
    let (tx, rx_to_web_server) = broadcast::channel::<WSResponse>(BUFFER_SIZE);
    let main_thread_comm_sender = MainThreadCommSender::new(tx);
    (main_thread_comm_sender, rx_to_web_server)
}

// SENDER
pub struct MainThreadCommSender {
    tx: Sender<WSResponse>,
}
impl MainThreadCommSender {
    pub fn new(tx: Sender<WSResponse>) -> Self {
        Self { tx }
    }
    pub fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
    pub fn notify_from_music_thread_song_started(&self) {
        let ws_response = WSResponse::FromMusicThread(WSMusicThreadResponse::SongStarted);
        self.tx.send(ws_response).unwrap();
    }
    pub fn notify_from_music_thread_song_ended(&self) {
        let ws_response = WSResponse::FromMusicThread(WSMusicThreadResponse::SongEnded);
        self.tx.send(ws_response).unwrap();
    }
    pub fn notify_from_client_thread_a_response(&self, message: String) {
        let ws_response = WSResponse::FromWSClient(WSClientResponse::SimpleResponse(message));
        self.tx.send(ws_response).unwrap();
    }
}
