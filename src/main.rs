use crate::drummer::{Drummer, ParamSoundType};
use crate::input::get_console_int_input;
use crate::midi_controller::init_midi_controller;
use std::io::Write;

mod drummer;
mod input;
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

    // SOUND SETTING
    drummer.set_param_level(0, ParamSoundType::Level, 127);
    loop {
        let value = get_console_int_input("Give me pitch value: ", 0, 127);
        if value == 0 {
            break;
        }

        // Before, it was: "drummer.send_cc_message(0, 26, sine_val_u8);"
        drummer.set_param_level(0, ParamSoundType::Pitch, value);
    }

    // MIDI CONNECTION: SHUT-DOWN
    conn.close();
}
