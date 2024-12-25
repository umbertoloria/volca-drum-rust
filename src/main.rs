use crate::drummer::{Drummer, DRUM_HH, DRUM_KICK, DRUM_SNARE, DUR_1_8};
use crate::midi_controller::init_midi_controller;
use std::io::Write;

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

    // MIDI CONNECTION: SHUT-DOWN
    conn.close();
}
