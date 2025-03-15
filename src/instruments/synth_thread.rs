use crate::music::note::{get_frequency_from_note_info, Note};
use crate::synth::digital_mono_synth::{
    DigitalMonoSynth, WaveTableOscillator, WaveTableOscillatorSample,
};
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use rodio::source::Source;
use std::thread;
use std::thread::JoinHandle;

pub enum SynthCommand {
    StartNotes(Vec<Note>),
    StopNote,
    CloseThread,
}
pub fn create_synth_thread_comm() -> (
    ThreadCommSender<SynthCommand>,
    ThreadCommReceiver<SynthCommand>,
) {
    create_thread_comm_instances::<SynthCommand>()
}

pub fn synth_thread(
    synth_thread_name: String,
    volume: f32,
    synth_command_receiver: ThreadCommReceiver<SynthCommand>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        // Synths
        let mut synths = Vec::new();

        for command in synth_command_receiver.get_recv_iter() {
            match command {
                SynthCommand::StartNotes(notes) => {
                    // println!("{synth_thread_name} -> play {:?}", notes);

                    for note in notes {
                        let source = create_source_from_note(&note);
                        let mut mono_synth = DigitalMonoSynth::new(volume);
                        mono_synth.play(source);
                        synths.push(mono_synth);
                    }
                }
                SynthCommand::StopNote => {
                    // println!("{synth_thread_name} -> stop");

                    for synth in &mut synths {
                        synth.pause();
                    }
                    synths.clear();
                }
                SynthCommand::CloseThread => {
                    // println!("{synth_thread_name} -> close");

                    for synth in &mut synths {
                        synth.pause();
                    }
                    synths.clear();
                    break;
                }
            }
        }
    })
}

// Actual Oscillator
fn create_sine_wave_table() -> Vec<f32> {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }
    wave_table
}
fn create_source_from_note(note: &Note) -> WaveTableOscillatorSample {
    // Sine Oscillator
    let sample_rate = 48000;
    let wave_table = create_sine_wave_table();
    let mut oscillator = WaveTableOscillator::new(sample_rate, wave_table);

    // Source
    let frequency = get_frequency_from_note_info(note);
    oscillator.set_frequency(frequency);
    oscillator.convert_samples()
}
