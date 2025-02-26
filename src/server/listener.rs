use crate::players::play_queue::play_song_in_queue;

pub fn listener_manage(request: String) -> String {
    let request = sanitize_request(request);
    match request {
        Some(WSRequest::PlaySong) => {
            play_song_in_queue();
            "OK".into()
        }
        Some(WSRequest::GetPlayQueueState) => {
            // get_play_queue_state();
            // TODO: Try to send asynchronously these data
            "PLAY QUEUE info..".into()
        }
        _ => "KO".into(),
    }
}
enum WSRequest {
    PlaySong,
    GetPlayQueueState,
}
fn sanitize_request(request: String) -> Option<WSRequest> {
    if request == "PLAY_SONG" {
        return Some(WSRequest::PlaySong);
    }
    if request == "GET_PLAY_QUEUE_STATE" {
        return Some(WSRequest::GetPlayQueueState);
    }
    None
}
