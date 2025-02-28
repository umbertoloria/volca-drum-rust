use crate::players::player::play_song_example;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

// PLAY QUEUE
type PlayQueueThread = JoinHandle<()>;
static PLAY_QUEUE: Mutex<Vec<PlayQueueThread>> = Mutex::new(Vec::new());
const DEFAULT_SONG_ID: usize = 7;

// EXPOSED
pub fn play_song_in_queue() {
    let mut play_queue = PLAY_QUEUE.lock().unwrap();
    if play_queue.is_empty() {
        let play_queue_thread: PlayQueueThread = thread::spawn(move || {
            play_song_example();
        });
        // Inserting as first element.
        play_queue.push(play_queue_thread);
    } else {
        // Checking the last element.
        let last_thread = play_queue.last().unwrap();
        if last_thread.is_finished() {
            let play_queue_thread: PlayQueueThread = thread::spawn(move || {
                play_song_example();
            });
            // Inserting as last element.
            play_queue.push(play_queue_thread);
        } else {
            // TODO: Add in queue...
            println!("Unable to play another song!");
        }
    }
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
        let play_queue_item = &play_queue[i];
        result.items.push(
            //
            PlayQueueStateItem {
                id: i,
                state: if play_queue_item.is_finished() {
                    PlayQueueStateItemState::Running
                } else {
                    PlayQueueStateItemState::Stopped
                },
            },
        );
    }
    result
}
