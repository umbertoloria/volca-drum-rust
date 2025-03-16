use crate::music::note::{get_frequency_from_note_info, Note};
use crate::synth::digital_mono_synth::{
    DigitalMonoSynth, WaveTableOscillator, WaveTableOscillatorSample,
};
use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_sine::create_sine_wave_table;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use crate::utils::timing::get_now_millis;
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
    lfo: LFO,
    volume: f32,
    synth_command_receiver: ThreadCommReceiver<SynthCommand>,
    enable_logging: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        // Synths
        let mut synths = Vec::new();

        for command in synth_command_receiver.get_recv_iter() {
            match command {
                SynthCommand::StartNotes(notes) => {
                    if enable_logging {
                        println!("{synth_thread_name} -> play {:?}", notes);
                    }

                    let now_millis = get_now_millis();

                    for note in notes {
                        let source = create_source_from_note(&note, &lfo, now_millis);
                        let mut mono_synth = DigitalMonoSynth::new(volume);
                        mono_synth.play(source);
                        synths.push(mono_synth);
                    }
                }
                SynthCommand::StopNote => {
                    if enable_logging {
                        println!("{synth_thread_name} -> stop");
                    }

                    for synth in &mut synths {
                        synth.pause();
                    }
                    synths.clear();
                }
                SynthCommand::CloseThread => {
                    if enable_logging {
                        println!("{synth_thread_name} -> close");
                    }

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
fn create_source_from_note(note: &Note, lfo: &LFO, now_millis: u128) -> WaveTableOscillatorSample {
    // Sine Oscillator
    let sample_rate = 48000;
    let wave_table = create_sine_wave_table();
    let mut oscillator = WaveTableOscillator::new(
        //
        sample_rate,
        wave_table,
        lfo.clone(), // FIXME: Avoid cloning LFO
        now_millis,
    );

    // Source
    let frequency = get_frequency_from_note_info(note);
    oscillator.set_frequency(frequency);
    oscillator.convert_samples()
}
