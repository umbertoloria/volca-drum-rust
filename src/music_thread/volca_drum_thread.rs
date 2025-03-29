use crate::devices::volca_drum::volca_drum::VolcaDrum;
use crate::devices::volca_drum::volca_drum_patch::VolcaDrumPatch;
use crate::devices::volca_drum::volca_drum_patches::get_volca_drum_patch_1;
use crate::midi::midi_controller::init_midi_controller;
use crate::midi::midi_device::MidiDeviceConcrete;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use std::thread;
use std::thread::JoinHandle;

pub enum VolcaDrumCommand {
    ApplyPatch(VolcaDrumPatch),
    CloseThread,
}
pub fn create_volca_drum_thread_comm() -> (
    ThreadCommSender<VolcaDrumCommand>,
    ThreadCommReceiver<VolcaDrumCommand>,
) {
    create_thread_comm_instances::<VolcaDrumCommand>()
}

pub fn volca_drum_thread(
    volca_drum_command_receiver: ThreadCommReceiver<VolcaDrumCommand>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let midi_device = MidiDeviceConcrete::new(init_midi_controller("DRUMS", Some(1)).unwrap());
        // let midi_device = MidiDeviceGhost::new(false);
        let mut volca_drum = VolcaDrum::new(Box::new(midi_device));

        // Sounds
        let patch1 = get_volca_drum_patch_1();
        // TODO: Make sure it always sounds ok from the first hit
        volca_drum.apply_sound(patch1);

        for command in volca_drum_command_receiver.get_recv_iter() {
            match command {
                VolcaDrumCommand::ApplyPatch(volca_drum_patch) => {
                    volca_drum.apply_sound(volca_drum_patch);
                }
                VolcaDrumCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}
