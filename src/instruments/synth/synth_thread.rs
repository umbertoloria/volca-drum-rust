use crate::music::note::Note;
use crate::synth::digital_mono_synth_sample_source::DigitalMonoSynthSampleSource;
use crate::synth::mono_synth::MonoSynth;
use crate::synth::mono_synth_player::MonoSynthPlayer;
use crate::synth::synth_generator::SynthGenerator;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use crate::utils::timing::get_now_millis;
use rodio::Source;
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
    synth_generator: SynthGenerator,
    synth_command_receiver: ThreadCommReceiver<SynthCommand>,
    enable_logging: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
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
                        let synth_chain = synth_generator.generate(now_millis);

                        let mut mono_synth = MonoSynth::new(synth_chain);
                        mono_synth.set_frequency(note.get_frequency());

                        let sample_source = DigitalMonoSynthSampleSource::new(mono_synth);
                        let samples_converter = sample_source.convert_samples();

                        let mut mono_synth_player =
                            MonoSynthPlayer::new(synth_generator.get_volume());
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
