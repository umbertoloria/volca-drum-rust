use crate::music_thread::music_library::MusicLibrary;
use crate::music_thread::music_thread_comm::{MusicThreadCommSender, MusicThreadCommand};
use crate::song::yaml_patch_reader::{parse_patch_from_yaml, YamlPatchFile};

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
                self.music_thread_comm_sender
                    .send(MusicThreadCommand::PlaySong());

                "OK".into()
            }
            Some(WSClientRequest::GetPlayQueueState) => {
                // TODO: Maybe this is useless since Server should keep Clients periodically updated...
                "PLAY QUEUE info..".into()
            }
            Some(WSClientRequest::ApplyPatch(yaml_patch_file)) => {
                self.music_thread_comm_sender
                    .send(MusicThreadCommand::ApplyVolcaDrumPatch(yaml_patch_file));

                "PATCH APPLIED".into()
            }
            Some(WSClientRequest::ReadMusicLibrary) => {
                let songs = MusicLibrary::get_songs();

                let mut result = String::new();
                // TODO: Convert list in JSON
                result.push_str("[]");

                result.into()
            }
            None => "KO".into(),
        }
    }
}

enum WSClientRequest {
    PlaySong,
    GetPlayQueueState,
    ApplyPatch(YamlPatchFile),
    ReadMusicLibrary,
}
fn sanitize_client_request(request: String) -> Option<WSClientRequest> {
    if request == "PLAY_SONG" {
        return Some(WSClientRequest::PlaySong);
    }
    if request == "GET_PLAY_QUEUE_STATE" {
        return Some(WSClientRequest::GetPlayQueueState);
    }
    if request == WS_CLIENT_REQUEST_READ_MUSIC_LIBRARY {
        return Some(WSClientRequest::ReadMusicLibrary);
    }
    if request.len() > 12 {
        let initial_request = &request[..12];
        if initial_request == "APPLY_PATCH\n" {
            let potential_yaml = &request[12..];
            match parse_patch_from_yaml(potential_yaml) {
                Some(yaml_patch_file) => {
                    // println!("PATCH SET! {:?}", yaml_patch_file);
                    return Some(WSClientRequest::ApplyPatch(yaml_patch_file));
                }
                None => {
                    println!("Unable to parse YAML string: {}", potential_yaml);
                }
            }
        }
    }
    println!("Unknown request: {:?}", request);
    None
}

pub const WS_CLIENT_REQUEST_READ_MUSIC_LIBRARY: &'static str = "READ_MUSIC_LIBRARY";
