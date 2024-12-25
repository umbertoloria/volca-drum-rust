use crate::input::get_console_int_input;
use crate::midi_controller::init_midi_controller;
use crate::sound_panel::{ParamSoundType, SoundPanel};
use std::io::Write;

mod drummer;
mod input;
mod midi_controller;
mod sound_panel;

fn main() {
    // MIDI CONNECTION: START
    let midi_controller = init_midi_controller(Some(1)).expect("Unable to create midi controller");
    let mut conn = midi_controller.connect_and_get();

    // SONG INIT
    let mut sound_panel = SoundPanel { conn: &mut conn };

    // SOUND SETTING
    sound_panel.set_param_level(0, ParamSoundType::Level, 127);
    loop {
        let value = get_console_int_input("Give me pitch value: ", 0, 127);
        if value == 0 {
            break;
        }

        // Before, it was: "drummer.send_cc_message(0, 26, sine_val_u8);"
        sound_panel.set_param_level(0, ParamSoundType::Pitch, value);
    }

    // MIDI CONNECTION: SHUT-DOWN
    conn.close();
}
