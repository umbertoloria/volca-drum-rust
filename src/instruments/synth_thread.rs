use crate::music::note::{get_frequency_from_note_info, Note};
use crate::synth::synth::WaveTableOscillator;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use core::time::Duration;
use rodio::{source::Source, OutputStream};
use std::thread;
use std::thread::{sleep, JoinHandle};

// TODO: Monophonic for now :)
pub enum SynthCommand {
    StartNote(Note),
    StopNote,
    CloseThread,
}
pub fn create_synth_thread_comm() -> (
    ThreadCommSender<SynthCommand>,
    ThreadCommReceiver<SynthCommand>,
) {
    create_thread_comm_instances::<SynthCommand>()
}

pub fn synth_thread(synth_command_receiver: ThreadCommReceiver<SynthCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
        for command in synth_command_receiver.get_recv_iter() {
            match command {
                SynthCommand::StartNote(note) => {
                    // println!("play {:?}", note);
                    play_note_in_synth(note);
                }
                SynthCommand::StopNote => {
                    // println!("stop playing");
                }
                SynthCommand::CloseThread => {
                    break;
                }
            }
        }
    })
}

fn play_note_in_synth(note: Note) {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    let mut oscillator = WaveTableOscillator::new(48000, wave_table);
    let frequency = get_frequency_from_note_info(&note);
    oscillator.set_frequency(frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    sleep(Duration::from_millis(350));
}
