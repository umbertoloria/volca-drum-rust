use crate::drummer::{Drummer, ParamSoundType};
use crate::midi_controller::init_midi_controller;
use std::f64::consts::PI;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod drummer;
mod midi_controller;

fn main() {
    // MIDI CONNECTION: START
    let midi_controller = init_midi_controller(Some(1)).expect("Unable to create midi controller");
    let mut conn = midi_controller.connect_and_get();

    // SONG INIT
    let bpm: f64 = 120.0;
    let mut drummer = Drummer {
        conn: &mut conn,
        bpm,
    };

    // SONG PLAY
    /*
    for _ in 0..(2 * 4) {
        drummer.hit(7, DRUM_HH, DUR_1_8);
    }
    drummer.hit(7, DRUM_KICK, DUR_1_8);
    drummer.hit(7, DRUM_HH, DUR_1_8);
    drummer.hit(7, DRUM_SNARE, DUR_1_8);
    drummer.hit(7, DRUM_HH, DUR_1_8);
    drummer.hit(7, DRUM_KICK, DUR_1_8);
    drummer.hit(7, DRUM_KICK, DUR_1_8);
    drummer.hit(7, DRUM_SNARE, DUR_1_8);
    drummer.hit(7, DRUM_HH, DUR_1_8);
    */

    drummer.set_param_level(0, ParamSoundType::Level, 127);

    let time_chunk = 1000 * 5;

    loop {
        let millis_this_very_second = get_now() % time_chunk;
        let from_0_to_1 = millis_this_very_second as f64 / time_chunk as f64;

        let sine_val = ((from_0_to_1 * 2.0 * PI).sin() + 1.0) / 2.0;
        let sine_val_u8 = (sine_val * 127.0) as u8;

        // Before, it was: "drummer.send_cc_message(0, 26, sine_val_u8);"
        drummer.set_param_level(0, ParamSoundType::Pitch, sine_val_u8);
        show_progress(sine_val);

        sleep(Duration::from_millis(149));
    }

    // MIDI CONNECTION: SHUT-DOWN
    conn.close();
}

fn get_now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

pub fn show_progress(val: f64) {
    let times = (val * 20.0) as usize;
    println!("{}", "#".repeat(times));
}
