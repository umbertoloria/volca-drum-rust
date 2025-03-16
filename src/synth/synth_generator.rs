use crate::music::note::Note;
use crate::synth::digital_mono_synth_sample_source::{
    DigitalMonoSynthSampleSource, DigitalMonoSynthSamplesConverter,
};
use crate::synth::lfo::lfo::LFO;
use crate::synth::mono_synth::MonoSynth;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use rodio::Source;

pub struct SynthGenerator {
    wave_table: Vec<f32>,
    lfo: LFO,
    volume: f32,
}
impl SynthGenerator {
    pub fn new(wave_table: Vec<f32>, lfo: LFO, volume: f32) -> Self {
        Self {
            wave_table,
            lfo,
            volume,
        }
    }
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn generate(&self, note: &Note, now_millis: u128) -> DigitalMonoSynthSamplesConverter {
        // FIXME: Avoid cloning Wave Table
        let wave_table = self.wave_table.clone();
        let sine_wt_oscillator = WaveTableOscillator::new(wave_table);

        // FIXME: Avoid cloning LFO
        let lfo = self.lfo.clone();

        let mut mono_synth = MonoSynth::new(sine_wt_oscillator, lfo, now_millis);
        mono_synth.set_frequency(note.get_frequency());

        let sample_source = DigitalMonoSynthSampleSource::new(mono_synth);
        sample_source.convert_samples()
    }
}
