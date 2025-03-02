use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use std::thread;
use std::thread::JoinHandle;

pub enum VolcaDrumCommand {
    CloseThread,
}
type VolcaDrumCommandSender = ThreadCommSender<VolcaDrumCommand>;
type VolcaDrumCommandReceiver = ThreadCommReceiver<VolcaDrumCommand>;
pub fn create_volca_drum_thread_comm() -> (VolcaDrumCommandSender, VolcaDrumCommandReceiver) {
    create_thread_comm_instances::<VolcaDrumCommand>()
}

pub fn volca_drum_thread(volca_drum_command_receiver: VolcaDrumCommandReceiver) -> JoinHandle<()> {
    // TODO: Use a Volca Drum instance to apply patches
    thread::spawn(move || {
        for command in volca_drum_command_receiver.get_recv_iter() {
            match command {
                VolcaDrumCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}
