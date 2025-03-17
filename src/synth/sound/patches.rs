use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::oscillator::wave_tables::{create_wt_saw, create_wt_square};
use crate::synth::sound::synth_patch_injector::SynthPatchInjector;
use crate::utils::timing::get_now_millis;

#[derive(Clone)]
pub enum SynthPatch {
    Bass1,
    Keys1,
    MetronomeClick,
}
impl SynthPatch {
    pub fn get_sample(&self, t0_ms: u128, index: f32) -> f32 {
        match self {
            SynthPatch::Bass1 => patch_keys_1(t0_ms, index),
            SynthPatch::Keys1 => patch_bass_1(t0_ms, index),
            SynthPatch::MetronomeClick => patch_metronome_click(t0_ms, index),
        }
    }
}

// PATCH KEYS 1
pub fn make_patch_keys_1() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::Keys1, 0.3)
}
pub fn patch_keys_1(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_square());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(17, 300, 0.3);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}

// PATCH BASS 1
pub fn make_patch_bass_1() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::Bass1, 0.4)
}
pub fn patch_bass_1(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    // let oscillator = WaveTableOscillator::new(create_wt_sine());
    let oscillator = WaveTableOscillator::new(create_wt_saw());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(30, 800, 0.7);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}

// PATCH METRONOME CLICK
pub fn make_patch_metronome_click() -> SynthPatchInjector {
    SynthPatchInjector::new(SynthPatch::MetronomeClick, 0.8)
}
pub fn patch_metronome_click(t0_ms: u128, index: f32) -> f32 {
    // 1. Oscillator
    let oscillator = WaveTableOscillator::new(create_wt_square());
    let oscillator_value = oscillator.lerp(index);

    // 2. LFO
    let ms = get_now_millis() - t0_ms;
    let lfo = LFO::new(10, 300, 0.0);
    let lfo_value = lfo.get_value(ms);

    oscillator_value * lfo_value
}
