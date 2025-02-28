use crate::players::play_queue::play_song_in_queue;

pub fn manage_client_request_if_valid(request: String) -> Option<String> {
    let client_request = sanitize_client_request(request);
    match client_request {
        Some(WSClientRequest::PlaySong) => {
            play_song_in_queue();
            Some("OK".into())
        }
        Some(WSClientRequest::GetPlayQueueState) => {
            // get_play_queue_state();
            // TODO: Try to send asynchronously these data
            Some("PLAY QUEUE info..".into())
        }
        _ => None,
    }
}

// Web Socket: Client Request
enum WSClientRequest {
    PlaySong,
    GetPlayQueueState,
}
fn sanitize_client_request(request: String) -> Option<WSClientRequest> {
    if request == "PLAY_SONG" {
        return Some(WSClientRequest::PlaySong);
    }
    if request == "GET_PLAY_QUEUE_STATE" {
        return Some(WSClientRequest::GetPlayQueueState);
    }
    None
}
