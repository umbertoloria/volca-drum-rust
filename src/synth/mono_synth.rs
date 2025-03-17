use crate::synth::oscillator::wave_tables::WAVE_TABLE_SIZE;
use crate::synth::sound::synth_chain_injector::DynAbstractSynthChain;

const SAMPLE_RATE: u32 = 48000;
pub struct MonoSynth {
    synth_chain: DynAbstractSynthChain,
    index: f32,
    index_increment: f32,
}
impl MonoSynth {
    pub fn new(synth_chain: DynAbstractSynthChain) -> Self {
        Self {
            synth_chain,
            index: 0.0,
            index_increment: 0.0,
        }
    }
    pub fn get_sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    pub fn get_sample_and_prepare_next(&mut self) -> f32 {
        let index = self.get_index_and_prepare_next();
        self.synth_chain.get_sample(index)
    }
    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * WAVE_TABLE_SIZE as f32 / self.get_sample_rate() as f32;
    }
    pub fn get_index_and_prepare_next(&mut self) -> f32 {
        let result = self.index;
        self.index += self.index_increment;
        self.index %= WAVE_TABLE_SIZE as f32;
        result
    }
}
