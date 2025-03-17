use crate::synth::lfo::lfo::LFO;
use crate::synth::oscillator::wave_table_oscillator::WaveTableOscillator;
use crate::synth::oscillator::wave_tables::{create_wt_saw, create_wt_square};
use crate::utils::timing::get_now_millis;

// Abstract Synth Chain Injector
pub trait AbstractSynthChain {
    fn get_sample(&self, index: f32) -> f32;
}
pub type DynAbstractSynthChain = Box<dyn AbstractSynthChain + Send>;
pub trait AbstractSynthChainInjector {
    fn get_synth_chain(&self, t0_ms: u128) -> DynAbstractSynthChain;
}
pub type DynSynthChainInjector = Box<dyn AbstractSynthChainInjector + Send>;

// Square Synth
pub struct SquareSynthChainInjector {
    //
}
impl SquareSynthChainInjector {
    pub fn new_box() -> DynSynthChainInjector {
        Box::new(
            //
            Self {},
        )
    }
}
impl AbstractSynthChainInjector for SquareSynthChainInjector {
    fn get_synth_chain(&self, t0_ms: u128) -> DynAbstractSynthChain {
        Box::new(
            //
            SquareSynthChain::new(t0_ms),
        )
    }
}
pub struct SquareSynthChain {
    t0_ms: u128,
}
impl SquareSynthChain {
    pub fn new(t0_ms: u128) -> Self {
        Self { t0_ms }
    }
}
impl AbstractSynthChain for SquareSynthChain {
    fn get_sample(&self, index: f32) -> f32 {
        // 1. Oscillator
        let oscillator = WaveTableOscillator::new(create_wt_square());
        let oscillator_value = oscillator.lerp(index);

        // 2. LFO
        let ms = get_now_millis() - self.t0_ms;
        let lfo = LFO::new(17, 300, 0.3);
        let lfo_value = lfo.get_value(ms);

        oscillator_value * lfo_value
    }
}

// Saw Synth
pub struct SawSynthChainInjector {
    //
}
impl SawSynthChainInjector {
    pub fn new_box() -> DynSynthChainInjector {
        Box::new(
            //
            Self {},
        )
    }
}
impl AbstractSynthChainInjector for SawSynthChainInjector {
    fn get_synth_chain(&self, t0_ms: u128) -> DynAbstractSynthChain {
        Box::new(
            //
            SawSynthChain::new(t0_ms),
        )
    }
}
pub struct SawSynthChain {
    t0_ms: u128,
}
impl SawSynthChain {
    pub fn new(t0_ms: u128) -> Self {
        Self { t0_ms }
    }
}
impl AbstractSynthChain for SawSynthChain {
    fn get_sample(&self, index: f32) -> f32 {
        // 1. Oscillator
        // let oscillator = WaveTableOscillator::new(create_wt_sine());
        let oscillator = WaveTableOscillator::new(create_wt_saw());
        let oscillator_value = oscillator.lerp(index);

        // 2. LFO
        let ms = get_now_millis() - self.t0_ms;
        let lfo = LFO::new(30, 800, 0.7);
        let lfo_value = lfo.get_value(ms);

        oscillator_value * lfo_value
    }
}
