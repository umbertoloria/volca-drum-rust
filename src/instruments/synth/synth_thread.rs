use crate::instruments::lib::drum_sounds::DrumSound;
use crate::music::note::Note;
use crate::synth::audio_channel::AudioChannel;
use crate::synth::digital_mono_synth_sample_source::DigitalMonoSynthSampleSource;
use crate::synth::mono_synth::{MonoSynth, SynthSoundType};
use crate::synth::mono_synth_player::MonoSynthPlayer;
use crate::synth::sound::synth_patch_injector::SynthPatchInjector;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};
use crate::utils::timing::get_now_millis;
use rodio::Source;
use std::thread;
use std::thread::JoinHandle;

pub enum SynthCommand {
    StartNotes(Vec<Note>),
    StartSounds(Vec<DrumSound>),
    Silence,
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
    synth_thread_audio_channel: SynthThreadAudioChannel,
    synth_patch_injector: SynthPatchInjector,
    synth_command_receiver: ThreadCommReceiver<SynthCommand>,
    enable_logging: bool,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let audio_channel = match synth_thread_audio_channel {
            SynthThreadAudioChannel::Main => AudioChannel::new_from_main(),
            // TODO: Use a secondary Audio Channel for Metronome
            SynthThreadAudioChannel::MetronomeClick => AudioChannel::new_from_main(),
        };

        // Synths
        let mut mono_synth_players = Vec::new();

        for command in synth_command_receiver.get_recv_iter() {
            match command {
                SynthCommand::StartNotes(notes) => {
                    if enable_logging {
                        println!("{synth_thread_name} -> play {:?}", notes);
                    }

                    let t0_ms = get_now_millis();

                    for note in notes {
                        let synth_patch = synth_patch_injector.get_synth_patch().clone();
                        let synth_sound_type = SynthSoundType::Note(note);

                        let mono_synth = MonoSynth::new(synth_patch, synth_sound_type, t0_ms);

                        let sample_source = DigitalMonoSynthSampleSource::new(mono_synth);
                        let samples_converter = sample_source.convert_samples();

                        let mut mono_synth_player =
                            MonoSynthPlayer::new(&audio_channel, synth_patch_injector.get_volume());
                        mono_synth_player.play(samples_converter);

                        mono_synth_players.push(mono_synth_player);
                    }
                }
                SynthCommand::StartSounds(drum_sounds) => {
                    if enable_logging {
                        println!("{synth_thread_name} -> play {:?}", drum_sounds);
                    }

                    let t0_ms = get_now_millis();

                    for drum_sound in drum_sounds {
                        let synth_patch = synth_patch_injector.get_synth_patch().clone();
                        let synth_sound_type = SynthSoundType::DrumSound(drum_sound);

                        let mono_synth = MonoSynth::new(synth_patch, synth_sound_type, t0_ms);

                        let sample_source = DigitalMonoSynthSampleSource::new(mono_synth);
                        let samples_converter = sample_source.convert_samples();

                        let mut mono_synth_player =
                            MonoSynthPlayer::new(&audio_channel, synth_patch_injector.get_volume());
                        mono_synth_player.play(samples_converter);

                        mono_synth_players.push(mono_synth_player);
                    }
                }
                SynthCommand::Silence => {
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

pub enum SynthThreadAudioChannel {
    Main,
    MetronomeClick,
}
