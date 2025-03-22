use crate::players::realtime_player::TempoSnapshot;
use crate::song::song::Song;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone, Debug)]
pub enum WSResponse {
    FromMusicThread(WSMusicThreadResponse),
    FromWSClient(WSClientResponse),
}
#[derive(Clone, Debug)]
pub enum WSMusicThreadResponse {
    SongStarted(Song),
    SongPlayingUpdate(TempoSnapshot),
    SongEnded,
}
#[derive(Clone, Debug)]
pub enum WSClientResponse {
    SimpleResponse(String),
}

pub fn create_channel_for_server_thread() -> (WebThreadCommSender, WebThreadCommReceiver) {
    // TODO: Adjust BUFFER_SIZE
    const BUFFER_SIZE: usize = 32;
    let (tx, rx) = broadcast::channel::<WSResponse>(BUFFER_SIZE);
    let web_thread_comm_sender = WebThreadCommSender::new(tx);
    let web_thread_comm_receiver = WebThreadCommReceiver::new(rx);
    (web_thread_comm_sender, web_thread_comm_receiver)
}

// SENDER
pub struct WebThreadCommSender {
    tx: Sender<WSResponse>,
}
impl WebThreadCommSender {
    pub fn new(tx: Sender<WSResponse>) -> Self {
        Self { tx }
    }
    pub fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
    pub fn notify_from_music_thread_song_started(&self, song: Song) {
        let ws_response = WSResponse::FromMusicThread(WSMusicThreadResponse::SongStarted(song));
        self.tx.send(ws_response).unwrap();
    }
    pub fn notify_from_music_thread_song_update(&self, tempo_snapshot: &TempoSnapshot) {
        // TODO: Avoid cloning Tempo Snapshot
        let ws_response = WSResponse::FromMusicThread(WSMusicThreadResponse::SongPlayingUpdate(
            tempo_snapshot.clone(),
        ));
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
pub struct WebThreadCommReceiver {
    rx: Receiver<WSResponse>,
}
impl WebThreadCommReceiver {
    pub fn new(rx: Receiver<WSResponse>) -> Self {
        Self { rx }
    }
    pub fn resubscribe(&self) -> WebThreadCommReceiver {
        Self::new(self.rx.resubscribe())
    }
    pub async fn async_new_message(&mut self) -> Result<WSResponse, RecvError> {
        self.rx.recv().await
    }
}
