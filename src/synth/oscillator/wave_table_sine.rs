pub const WAVE_TABLE_SIZE: usize = 64;

pub fn create_sine_wave_table() -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
    for n in 0..WAVE_TABLE_SIZE {
        let delta = n as f32 / WAVE_TABLE_SIZE as f32;
        let value = (std::f32::consts::TAU * delta).sin();
        wave_table.push(value);
    }
    wave_table
}
