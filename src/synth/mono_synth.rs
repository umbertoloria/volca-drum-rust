use crate::instruments::lib::drum_sounds::DrumSound;
use crate::music::note::Note;
use crate::synth::oscillator::wave_tables::WAVE_TABLE_SIZE;
use crate::synth::sound::patches::SynthPatch;

const SAMPLE_RATE: u32 = 48000;
pub struct MonoSynth {
    synth_patch: SynthPatch,
    synth_sound_type: SynthSoundType,
    t0_ms: u128,
    index: f32,
    index_increment: f32,
}
impl MonoSynth {
    pub fn new(
        //
        synth_patch: SynthPatch,
        synth_sound_type: SynthSoundType,
        t0_ms: u128,
    ) -> Self {
        let frequency = synth_patch.get_frequency(&synth_sound_type);
        let mut new = Self {
            synth_patch,
            synth_sound_type,
            t0_ms,
            index: 0.0,
            index_increment: 0.0,
        };
        new.set_frequency(frequency);
        new
    }
    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * WAVE_TABLE_SIZE as f32 / self.get_sample_rate() as f32;
    }
    pub fn get_sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    pub fn get_sample_and_prepare_next(&mut self) -> f32 {
        // Don't change method signature.
        let index = self.get_index_and_prepare_next();
        // TODO: Avoid cloning Synth Sound Type
        self.synth_patch
            .get_sample(self.t0_ms, index, self.synth_sound_type.clone())
    }
    pub fn get_index_and_prepare_next(&mut self) -> f32 {
        let result = self.index;
        self.index += self.index_increment;
        self.index %= WAVE_TABLE_SIZE as f32;
        result
    }
}

#[derive(Clone)]
pub enum SynthSoundType {
    Note(Note),
    DrumSound(DrumSound),
}
