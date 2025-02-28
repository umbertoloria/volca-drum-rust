use crate::music_thread::music_thread_comm::MusicThreadCommSender;

pub struct ServerRequestManager {
    music_thread_comm_sender: MusicThreadCommSender,
}
impl ServerRequestManager {
    pub fn new(music_thread_comm_sender: MusicThreadCommSender) -> Self {
        Self {
            music_thread_comm_sender,
        }
    }
    pub fn manage(&self, request: String) -> String {
        match sanitize_client_request(request) {
            Some(WSClientRequest::PlaySong) => {
                self.music_thread_comm_sender.request_play_song();

                "OK".into()
            }
            Some(WSClientRequest::GetPlayQueueState) => {
                // TODO: Maybe this is useless since Server should keep Clients periodically updated...
                "PLAY QUEUE info..".into()
            }
            _ => "KO".into(),
        }
    }
}

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
