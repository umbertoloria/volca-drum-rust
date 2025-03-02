use crate::song::yaml_patch_reader::YamlPatchFile;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};

pub enum MusicThreadCommand {
    PlaySong(), // TODO: Add Song ID here
    ApplyVolcaDrumPatch(YamlPatchFile),
}

pub fn create_music_thread_comm_instances() -> (MusicThreadCommSender, MusicThreadCommReceiver) {
    let (sender, receiver) = create_thread_comm_instances::<MusicThreadCommand>();
    (sender, receiver)
}

pub type MusicThreadCommSender = ThreadCommSender<MusicThreadCommand>;
pub type MusicThreadCommReceiver = ThreadCommReceiver<MusicThreadCommand>;
