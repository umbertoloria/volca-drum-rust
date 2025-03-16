use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::sound::synth_chain::BasicSynthChain;

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
    pub fn generate(&self, now_millis: u128) -> BasicSynthChain {
        // FIXME: Avoid cloning Wave Table
        let wave_table = self.wave_table.clone();
        let sine_wt_oscillator = WaveTableOscillator::new(wave_table);

        // FIXME: Avoid cloning LFO
        let lfo = self.lfo.clone();

        BasicSynthChain::new(
            //
            sine_wt_oscillator,
            lfo,
            now_millis,
        )
    }
}
