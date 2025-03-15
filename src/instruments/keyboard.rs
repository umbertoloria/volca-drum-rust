use crate::devices::volca_keys::VolcaKeys;
use crate::instruments::instrument::Instrument;
use crate::instruments::stop_notes_queue::{
    AbstractKeysBasedInstrument, KeysWithQueue, StopNotesQueue,
};
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{KeyboardPattern, Song};

pub struct Keyboard {
    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    keys_with_queue: KeysWithQueue,
}
impl Keyboard {
    pub fn new(volca_keys: VolcaKeys) -> Self {
        Self {
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            keys_with_queue: KeysWithQueue::new(
                StopNotesQueue::new(),
                Box::new(VolcaKeysBasedInstrument::new(volca_keys)),
            ),
        }
    }
    fn update_pattern_from_song_section(&mut self, song: &Song) {
        if self.curr_section_index < song.sections.len() {
            let current_song_section = &song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.keyboard_pattern_key {
                Some(keyboard_pattern_key) => {
                    let keyboard_pattern = song
                        .get_keyboard_pattern_from_key(keyboard_pattern_key.into())
                        .expect("Unable to find right Keyboard Pattern")
                        // TODO: Avoid cloning pattern
                        .clone();
                    Some(keyboard_pattern)
                }
                None => None,
            }
        } else {
            self.pattern = None;
        }
    }
    fn play_1_16th(&mut self, song: &Song, tempo_snapshot: &TempoSnapshot) {
        let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.keys_with_queue
            .playing_hit_dequeue_and_stop_notes_at_this_1_16th(index_1_16th);

        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern = (index_1_16th - 1) % (bars_covered_by_pattern * 16) + 1;

            // FIXME: This is slow
            let mut i = 0;
            for chord in &pattern.chords {
                if chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= chord.to_1_16th_incl
                {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }

            if 0 <= self.chord_index && self.chord_index < pattern.chords.len() {
                // TODO: Avoid cloning pattern
                let pattern = self.pattern.clone().unwrap();
                let chord = &pattern.chords[self.chord_index];

                /*
                println!("play_1_16th:");
                println!(" > index_1_16th_for_pattern={index_1_16th_for_pattern}");
                println!(
                    " > 1/16ths [{}, {}]",
                    chord.from_1_16th_incl, chord.to_1_16th_incl
                );
                println!(" > chord notes={:?}", chord.notes);
                */

                if index_1_16th_for_pattern == chord.from_1_16th_incl {
                    self.keys_with_queue.playing_hit_chord_start(&chord.notes);
                } else if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Here we check if this Chord's Notes should end on the *next* of this 1/16th
                    // (so after current 1/16th) using variable "index_1_16th_for_pattern".
                    // Queueing Notes to be stopped using "index_1_16th" since Stop Notes Queue uses
                    // absolute 1/16ths Indexes.
                    self.keys_with_queue
                        .playing_hit_last_before_chord_stop(&chord.notes, index_1_16th + 1);
                } else {
                    // Notes are still playing.
                }
            }
        }

        // Preparing the next hit!
        // TODO: Avoid cloning Song
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section(&song);
        }
    }
}
impl Instrument for Keyboard {
    fn get_instrument_name_16_chars(&self) -> String {
        "Keyboard        ".into()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            if self.chord_index < pattern.chords.len() {
                let chord = &pattern.chords[self.chord_index];
                return format!("part \"{}\" / {} chord", pattern.key, chord.chord_name);
            }
        }
        "".to_string()
    }
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section(&song);

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th(&song, tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }
    }
}

struct VolcaKeysBasedInstrument {
    volca_keys: VolcaKeys,
}
impl VolcaKeysBasedInstrument {
    pub fn new(volca_keys: VolcaKeys) -> Self {
        Self { volca_keys }
    }
}
impl AbstractKeysBasedInstrument for VolcaKeysBasedInstrument {
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_start: {:?}", notes_str_list);

        for note_str in notes_str_list {
            // TODO: Avoid cloning note
            let note_str_clone = note_str.clone();

            self.volca_keys.note_play_start(note_str_clone);
        }
    }
    fn play_notes_stop(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_stop: {:?}", notes_str_list);

        for note_str in notes_str_list {
            // TODO: Avoid cloning note
            let note_str_clone = note_str.clone();

            self.volca_keys.note_play_stop(note_str_clone);
        }
    }
}
