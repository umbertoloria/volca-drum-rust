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

// Generic Synth Chain Injector
pub struct GenericSynthChainInjector {
    //
    sound_synth_patch: SoundSynthPatch,
}
impl GenericSynthChainInjector {
    pub fn new_box(sound_synth_patch: SoundSynthPatch) -> DynSynthChainInjector {
        Box::new(
            //
            Self {
                //
                sound_synth_patch,
            },
        )
    }
}
impl AbstractSynthChainInjector for GenericSynthChainInjector {
    fn get_synth_chain(&self, t0_ms: u128) -> DynAbstractSynthChain {
        Box::new(
            //
            GenericSynthChain::new(t0_ms, self.sound_synth_patch.clone()),
        )
    }
}
pub struct GenericSynthChain {
    t0_ms: u128,
    sound_synth_patch: SoundSynthPatch,
}
impl GenericSynthChain {
    pub fn new(
        //
        t0_ms: u128,
        sound_synth_patch: SoundSynthPatch,
    ) -> Self {
        Self {
            //
            t0_ms,
            sound_synth_patch,
        }
    }
}
impl AbstractSynthChain for GenericSynthChain {
    fn get_sample(&self, index: f32) -> f32 {
        self.sound_synth_patch.get_sample(self.t0_ms, index)
    }
}

// Sound Synth Patch
#[derive(Clone)]
pub enum SoundSynthPatch {
    SQUARE,
    SAW,
}
impl SoundSynthPatch {
    fn get_sample(&self, t0_ms: u128, index: f32) -> f32 {
        match self {
            SoundSynthPatch::SQUARE => {
                // 1. Oscillator
                let oscillator = WaveTableOscillator::new(create_wt_square());
                let oscillator_value = oscillator.lerp(index);

                // 2. LFO
                let ms = get_now_millis() - t0_ms;
                let lfo = LFO::new(17, 300, 0.3);
                let lfo_value = lfo.get_value(ms);

                oscillator_value * lfo_value
            }
            SoundSynthPatch::SAW => {
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
        }
    }
}
