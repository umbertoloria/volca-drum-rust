use crate::synth::sound::synth_chain_injector::{
    AbstractSynthChainInjector, DynAbstractSynthChain, DynSynthChainInjector,
};

pub struct SynthGenerator {
    synth_chain_injector: DynSynthChainInjector,
    volume: f32,
}
impl SynthGenerator {
    pub fn new(synth_chain_injector: DynSynthChainInjector, volume: f32) -> Self {
        Self {
            synth_chain_injector,
            volume,
        }
    }
    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn generate(&self, t0_ms: u128) -> DynAbstractSynthChain {
        self.synth_chain_injector.get_synth_chain(t0_ms)
    }
}
