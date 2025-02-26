use crate::players::player::play_song_example;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

// PLAY QUEUE
type PlayQueueThread = JoinHandle<()>;
static PLAY_QUEUE: Mutex<Vec<PlayQueueThread>> = Mutex::new(Vec::new());
fn add_playing_song_to_list(thread: PlayQueueThread) {
    PLAY_QUEUE.lock().unwrap().push(thread);
}

// EXPOSED
pub fn play_song_in_queue() {
    let thread: PlayQueueThread = thread::spawn(move || {
        play_song_example();
    });
    add_playing_song_to_list(thread);
}

pub struct PlayQueueState {
    items: Vec<PlayQueueStateItem>,
}
struct PlayQueueStateItem {
    id: usize,
    state: PlayQueueStateItemState,
}
enum PlayQueueStateItemState {
    Running,
    Stopped,
}
pub fn get_play_queue_state() -> PlayQueueState {
    let play_queue = PLAY_QUEUE.lock().unwrap();
    let mut result = PlayQueueState { items: Vec::new() };
    for i in 0..play_queue.len() {
        result.items.push(PlayQueueStateItem {
            id: i,
            state: PlayQueueStateItemState::Running, // TODO: Update state
        });
    }
    result
}
