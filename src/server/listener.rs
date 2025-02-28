use crate::players::play_queue::play_song_in_queue;

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
pub fn manage_client_request_if_valid(request: String) -> Option<String> {
    match sanitize_client_request(request) {
        Some(WSClientRequest::PlaySong) => {
            play_song_in_queue();
            Some("OK".into())
        }
        Some(WSClientRequest::GetPlayQueueState) => {
            // TODO: Maybe this is useless since Server should keep Clients periodically updated...
            Some("PLAY QUEUE info..".into())
        }
        _ => None,
    }
}
