use crate::music_thread::music_thread::WSMusicThreadResponse;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
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

pub fn create_channel_for_server_thread() -> (MainThreadCommSender, MainThreadCommReceiver) {
    // TODO: Adjust BUFFER_SIZE
    const BUFFER_SIZE: usize = 32;
    let (tx, rx) = broadcast::channel::<WSResponse>(BUFFER_SIZE);
    let main_thread_comm_sender = MainThreadCommSender::new(tx);
    let main_thread_comm_receiver = MainThreadCommReceiver::new(rx);
    (main_thread_comm_sender, main_thread_comm_receiver)
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

// RECEIVER
pub struct MainThreadCommReceiver {
    rx: Receiver<WSResponse>,
}
impl MainThreadCommReceiver {
    pub fn new(rx: Receiver<WSResponse>) -> Self {
        Self { rx }
    }
    pub fn resubscribe(&self) -> MainThreadCommReceiver {
        Self::new(self.rx.resubscribe())
    }
    pub async fn async_new_message(&mut self) -> Result<WSResponse, RecvError> {
        self.rx.recv().await
    }
}
