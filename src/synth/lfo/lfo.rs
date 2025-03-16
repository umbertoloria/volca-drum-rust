#[derive(Clone)]
pub struct LFO {
    pub attack__ms: u128,
    pub decay___ms: u128,
    pub sustain_vl: f32,
}
impl LFO {
    pub fn new(attack__ms: u128, decay___ms: u128, sustain_vl: f32) -> Self {
        Self {
            attack__ms,
            decay___ms,
            sustain_vl,
        }
    }
    pub fn get_value(&self, ms: u128) -> f32 {
        if ms <= self.attack__ms {
            // Going up to Attack.
            (ms as f32) / self.attack__ms as f32
        } else if ms <= self.decay___ms {
            // Going from Attack to Sustain.
            let from_0_to_1 =
                (ms - self.attack__ms) as f32 / (self.decay___ms - self.attack__ms) as f32;
            let volume_diff = 1.0 - self.sustain_vl;
            1.0 - from_0_to_1 * volume_diff
        } else {
            // Staying in Sustain forever...
            self.sustain_vl
        }
    }
}
