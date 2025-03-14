use crate::music::note::{get_frequency_from_note_info, Note};
use crate::synth::synth::WaveTableOscillator;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use core::time::Duration;
use rodio::source::SamplesConverter;
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
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

                    let source = create_source_from_note(&note);
                    let _result = stream_handle.play_raw(source);
                    sleep(Duration::from_millis(350));
                    // TODO: Killing "stream_handle" would mean stopping the sound
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

fn create_sine_wave_table() -> Vec<f32> {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }
    wave_table
}
fn create_source_from_note(note: &Note) -> SamplesConverter<WaveTableOscillator, f32> {
    // Sine Oscillator
    let sample_rate = 48000;
    let wave_table = create_sine_wave_table();
    let mut oscillator = WaveTableOscillator::new(sample_rate, wave_table);

    // Source
    let frequency = get_frequency_from_note_info(note);
    oscillator.set_frequency(frequency);
    oscillator.convert_samples()
}
