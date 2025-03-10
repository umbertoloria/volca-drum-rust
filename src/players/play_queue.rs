use crate::players::player::play_song_example;
use crate::server::web_thread_comm::WebThreadCommSender;
use std::sync::Mutex;
use std::thread::JoinHandle;

// PLAY QUEUE
type PlayQueueThread = JoinHandle<()>;
static PLAY_QUEUE: Mutex<Vec<PlayQueueThread>> = Mutex::new(Vec::new());
const DEFAULT_SONG_ID: usize = 7;

pub fn play_song_example_with_updates(web_thread_comm_sender: WebThreadCommSender) {
    web_thread_comm_sender.notify_from_music_thread_song_started();

    play_song_example();

    web_thread_comm_sender.notify_from_music_thread_song_ended();
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
