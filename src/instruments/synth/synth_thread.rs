use crate::music::note::Note;
use crate::synth::digital_mono_synth_sample_source::{
    DigitalMonoSynthSampleSource, DigitalMonoSynthSamplesConverter,
};
use crate::synth::lfo::lfo::LFO;
use crate::synth::mono_synth::MonoSynth;
use crate::synth::mono_synth_player::MonoSynthPlayer;
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
    wave_table: Vec<f32>,
    lfo: LFO,
    volume: f32,
    synth_command_receiver: ThreadCommReceiver<SynthCommand>,
    enable_logging: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        // TODO: Take SynthGenerator from parameters
        let synth_generator = SynthGenerator::new(wave_table, lfo);
        // Synths
        let mut mono_synth_players = Vec::new();

        for command in synth_command_receiver.get_recv_iter() {
            match command {
                SynthCommand::StartNotes(notes) => {
                    if enable_logging {
                        println!("{synth_thread_name} -> play {:?}", notes);
                    }

                    let now_millis = get_now_millis();

                    for note in notes {
                        let samples_converter = synth_generator.generate(&note, now_millis);

                        let mut mono_synth_player = MonoSynthPlayer::new(volume);
                        mono_synth_player.play(samples_converter);
                        mono_synth_players.push(mono_synth_player);
                    }
                }
                SynthCommand::StopNote => {
                    if enable_logging {
                        println!("{synth_thread_name} -> stop");
                    }

                    for synth in &mut mono_synth_players {
                        synth.pause();
                    }
                    mono_synth_players.clear();
                }
                SynthCommand::CloseThread => {
                    if enable_logging {
                        println!("{synth_thread_name} -> close");
                    }

                    for synth in &mut mono_synth_players {
                        synth.pause();
                    }
                    mono_synth_players.clear();
                    break;
                }
            }
        }
    })
}

struct SynthGenerator {
    wave_table: Vec<f32>,
    lfo: LFO,
}
impl SynthGenerator {
    pub fn new(wave_table: Vec<f32>, lfo: LFO) -> Self {
        Self { wave_table, lfo }
    }
    pub fn generate(&self, note: &Note, now_millis: u128) -> DigitalMonoSynthSamplesConverter {
        let sine_wt_oscillator = WaveTableOscillator::new(create_sine_wave_table());

        // FIXME: Avoid cloning LFO
        let lfo = self.lfo.clone();

        let mut mono_synth = MonoSynth::new(sine_wt_oscillator, lfo, now_millis);
        mono_synth.set_frequency(note.get_frequency());

        let sample_source = DigitalMonoSynthSampleSource::new(mono_synth);
        sample_source.convert_samples()
    }
}
