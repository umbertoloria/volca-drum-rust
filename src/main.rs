use crate::player::Player;
use crate::yaml_reader::read_from_yaml;
use std::io::Write;

mod drummer;
mod input;
mod midi_controller;
mod player;
mod sound_panel;
mod yaml_reader;

fn main() {
    // READ YAML FILE
    let song1 = read_from_yaml("files/songs/harry-styles-sign-of-the-times.yaml");

    // PLAY SONG
    let player = Player::new(song1.tempo_1_4);
    println!("Play song \"{}\" by \"{}\"", song1.title, song1.author);
    for section in &song1.sections {
        let mut notes = "";
        if let Some(x) = &section.notes {
            notes = x;
        }
        println!("New section: type {:6} -> {}", section.kind, notes);
        player.play_num_bars(section.bars);
    }
    println!("Song end.");

    /*
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
    */
}
