use crate::players::player::play_song_example;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub enum CommFromMusicThread {
    SongStarted,
    SongPlayingUpdate,
    SongEnded,
}
type CommFromMusicThreadTx = Sender<CommFromMusicThread>;
pub type CommFromMusicThreadRx = Receiver<CommFromMusicThread>;
pub fn main_music_thread() -> (JoinHandle<()>, CommFromMusicThreadRx) {
    let (tx, rx) = mpsc::channel::<CommFromMusicThread>();
    let thread = thread::spawn(|| {
        music_thread(tx);
    });
    (thread, rx)
}

fn music_thread(tx: CommFromMusicThreadTx) {
    tx.send(CommFromMusicThread::SongStarted).unwrap();
    play_song_example();
    tx.send(CommFromMusicThread::SongEnded).unwrap();
}
