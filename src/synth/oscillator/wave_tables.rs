pub const WAVE_TABLE_SIZE: usize = 64;

pub fn create_wt_sine() -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
    for n in 0..WAVE_TABLE_SIZE {
        let delta = n as f32 / WAVE_TABLE_SIZE as f32;
        let value = (std::f32::consts::TAU * delta).sin();
        wave_table.push(value);
    }
    wave_table
}

pub fn create_wt_square() -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
    let first_half = WAVE_TABLE_SIZE / 2;
    let mut i = 0;
    while i < first_half {
        wave_table.push(1.0);
        i += 1;
    }
    while i < WAVE_TABLE_SIZE {
        wave_table.push(-1.0);
        i += 1;
    }
    wave_table
}

pub fn create_wt_saw() -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
    let first_quarter = WAVE_TABLE_SIZE / 4;
    let third_quarter = 3 * WAVE_TABLE_SIZE / 4;
    let mut i = 0;
    // From 0.0 to 1.0.
    while i < first_quarter {
        let delta = i as f32 / first_quarter as f32;
        wave_table.push(delta);
        i += 1;
    }
    // From 1.0 down to -1.0.
    while i < third_quarter {
        let delta = 1.0 - 2.0 * (i - first_quarter) as f32 / (third_quarter - first_quarter) as f32;
        wave_table.push(delta);
        i += 1;
    }
    // From -1.0 to 0.0.
    while i < WAVE_TABLE_SIZE {
        let delta = (i - third_quarter) as f32 / (WAVE_TABLE_SIZE - third_quarter) as f32 - 1.0;
        wave_table.push(delta);
        i += 1;
    }
    wave_table
}
