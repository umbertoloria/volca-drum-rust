use crate::instruments::lib::drum_sounds::DrumSound;
use crate::synth::lfo::lfo::LFO;
use crate::synth::mono_synth::SynthSoundType;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::oscillator::wave_tables::{
    create_wt_noise, create_wt_saw, create_wt_sine, create_wt_square,
};
use crate::synth::sound::synth_patch_injector::SynthPatchInjector;
use crate::utils::timing::get_now_millis;

#[derive(Clone)]
pub enum SynthPatch {
    Bass1,
    Keys1,
    MetronomeClick,
    Drums1,
}
impl SynthPatch {
    pub fn get_frequency(&self, synth_sound_type: &SynthSoundType) -> f32 {
        match synth_sound_type {
            SynthSoundType::Note(note) => note.get_frequency(),
            SynthSoundType::DrumSound(drum_sound) => patch_drums_1_frequency(drum_sound),
        }
    }
    pub fn get_sample(&self, t0_ms: u128, index: f32, synth_sound_type: SynthSoundType) -> f32 {
        match self {
            SynthPatch::Keys1 => patch_keys_1(t0_ms, index),
            SynthPatch::Bass1 => patch_bass_1(t0_ms, index),
            SynthPatch::MetronomeClick => patch_metronome_click(t0_ms, index),
            SynthPatch::Drums1 => patch_drums_1(t0_ms, index, synth_sound_type),
        }
    }
}

// PATCH KEYS 1
pub fn make_patch_keys_1(enable: bool) -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::Keys1, if enable { 1.0 } else { 0.0 })
}
fn patch_keys_1(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_sine());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(80, 300, 0.8);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}

// PATCH BASS 1
pub fn make_patch_bass_1(enable: bool) -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::Bass1, if enable { 0.25 } else { 0.0 })
}
fn patch_bass_1(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_square());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(30, 800, 0.7);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}

// PATCH METRONOME CLICK
pub fn make_patch_metronome_click(enable: bool) -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::MetronomeClick, if enable { 0.2 } else { 0.0 })
}
fn patch_metronome_click(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_square());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(10, 300, 0.0);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}

// PATCH DRUMS 1
pub fn make_patch_drums_1() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::Drums1, 1.0)
}
fn patch_drums_1(t0_ms: u128, index: f32, synth_sound_type: SynthSoundType) -> f32 {
    match synth_sound_type {
        SynthSoundType::DrumSound(drum_sound) => match drum_sound {
            DrumSound::KICK => patch_drums_1_kick(t0_ms, index),
            DrumSound::HH => patch_drums_1_hh(t0_ms, index),
            DrumSound::SNARE => patch_drums_1_snare(t0_ms, index),
        },
        _ => {
            // Silence.
            0.0
        }
    }
}
fn patch_drums_1_frequency(drum_sound: &DrumSound) -> f32 {
    match drum_sound {
        DrumSound::KICK => 110.0,
        DrumSound::HH => 5500.0,
        DrumSound::SNARE => 1510.0,
    }
}
fn patch_drums_1_kick(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_saw());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(30, 300, 0.0);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}
fn patch_drums_1_hh(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    // let oscillator = WaveTableOscillator::new(create_wt_square());
    let oscillator = WaveTableOscillator::new(create_wt_noise());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(10, 80, 0.0);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}
fn patch_drums_1_snare(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_noise());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(10, 250, 0.0);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}
