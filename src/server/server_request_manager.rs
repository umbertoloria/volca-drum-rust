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
            None => "KO".into(),
        }
    }
}

enum WSClientRequest {
    PlaySong,
    GetPlayQueueState,
    ApplyPatch(YamlPatchFile),
}
fn sanitize_client_request(request: String) -> Option<WSClientRequest> {
    if request == "PLAY_SONG" {
        return Some(WSClientRequest::PlaySong);
    }
    if request == "GET_PLAY_QUEUE_STATE" {
        return Some(WSClientRequest::GetPlayQueueState);
    }
    if request.len() > 12 {
        let initial_request = &request[..12];
        if initial_request == "APPLY_PATCH\n" {
            let potential_yaml = &request[12..];
            match parse_patch_from_yaml(potential_yaml) {
                Some(yaml_patch_file) => {
                    // println!("PATCH SET! {:?}", yaml_patch_file);
                    /*
                    // DEFAULT PATCH FOR DEBUGGING...
                    let yaml_patch_file = YamlPatchFile {
                        kick: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                        hh: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                        snare: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                        sound4: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                        sound5: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                        sound6: YamlPatchLayout {
                            sound_src_type: YamlPatchLayoutSoundSrcType::WaveSine,
                            mod_type: YamlPatchLayoutModulationType::ModTri,
                            amp_eg: YamlPatchLayoutAmpEg::EnvExp,
                            level: 10,
                            pitch: 10,
                            eg_attack: 10,
                            eg_release: 10,
                            mod_amount: 10,
                            mod_rate: 10,
                        },
                    };
                    */
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
