pub struct WaveTableOscillator {
    wave_table: Vec<f32>,
}
impl WaveTableOscillator {
    pub fn new(wave_table: Vec<f32>) -> Self {
        Self { wave_table }
    }
    pub fn lerp(&self, index: f32) -> f32 {
        let truncated_index = index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}
