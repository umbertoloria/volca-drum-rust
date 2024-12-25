use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use crate::midi_controller::init_midi;

mod midi_controller;

fn main() {
    let controller = init_midi().expect("Unable to create midi controller");
    let out_port = &controller.midi_port;

    println!("\nOpening connection");
    let mut conn_out = controller.midi_output.connect(&out_port, "midir-test").unwrap();
    println!("Connection open. Listen!");
    {
        // Durations
        const DUR_1_4: Duration = Duration::from_millis(1000);
        const DUR_1_8: Duration = Duration::from_millis(500);
        const DUR_1_16: Duration = Duration::from_millis(250);
        const DUR_1_32: Duration = Duration::from_millis(125);

        // Drum parts
        const DRUM_KICK: u8 = 1;
        const DRUM_HH: u8 = 2;
        const DRUM_SNARE: u8 = 4;

        // BPM config
        const BPM_DEFAULT: f64 = 60.0;
        let bpm: f64 = 120.0;

        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, instr: u8, duration: Duration| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const PROGRAM_CHANGE: u8 = 0xC0;
            const VELOCITY: u8 = 0x70;
            // We're ignoring errors in here
            let _ = conn_out.send(&[PROGRAM_CHANGE, instr]);
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(duration.mul_f64(BPM_DEFAULT).div_f64(bpm));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        // First bar
        for _ in 0..(2 * 4) {
            play_note(7, DRUM_HH, DUR_1_8);
        }
        // Second bar
        play_note(7, DRUM_KICK, DUR_1_8);
        play_note(7, DRUM_HH, DUR_1_8);
        play_note(7, DRUM_SNARE, DUR_1_8);
        play_note(7, DRUM_HH, DUR_1_8);
        play_note(7, DRUM_KICK, DUR_1_8);
        play_note(7, DRUM_KICK, DUR_1_8);
        play_note(7, DRUM_SNARE, DUR_1_8);
        play_note(7, DRUM_HH, DUR_1_8);
    }
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
}
