use crate::music::note::{get_frequency_from_note_info, Note};
use crate::synth::mono_synth_player::MonoSynthPlayer;
use crate::synth::digital_mono_synth_sample_source::{
    DigitalMonoSynthSampleSource, DigitalMonoSynthSamplesConverter,
};
use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
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
                        let samples_converter = create_source_from_note(&note, &lfo, now_millis);
                        let mut mono_synth = MonoSynthPlayer::new(volume);
                        mono_synth.play(samples_converter);
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
fn create_source_from_note(
    note: &Note,
    lfo: &LFO,
    now_millis: u128,
) -> DigitalMonoSynthSamplesConverter {
    // Sine Oscillator
    let sine_wt_oscillator = WaveTableOscillator::new(create_sine_wave_table());

    // Sample Source
    let sample_rate = 48000;
    let mut sample_source = DigitalMonoSynthSampleSource::new(
        //
        sample_rate,
        sine_wt_oscillator,
        lfo.clone(), // FIXME: Avoid cloning LFO
        now_millis,
    );

    // Sample & Source
    let frequency = get_frequency_from_note_info(note);
    sample_source.set_frequency(frequency);
    sample_source.convert_samples()
}
