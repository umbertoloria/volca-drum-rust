use crate::music_thread::music_thread::{MusicThreadRequest, MusicThreadRequestsTx};

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
pub fn manage_client_request_if_valid(
    request: String,
    music_thread_requests_tx: &MusicThreadRequestsTx,
) -> Option<String> {
    match sanitize_client_request(request) {
        Some(WSClientRequest::PlaySong) => {
            // TODO: Extract method in class wrapper
            music_thread_requests_tx
                .send(MusicThreadRequest::PlaySong())
                .unwrap();

            Some("OK".into())
        }
        Some(WSClientRequest::GetPlayQueueState) => {
            // TODO: Maybe this is useless since Server should keep Clients periodically updated...
            Some("PLAY QUEUE info..".into())
        }
        _ => None,
    }
}
