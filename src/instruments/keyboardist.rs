use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_keys_based_instrument::AbstractKeysBasedInstrument;
use crate::instruments::lib::keys_with_queue::KeysWithQueue;
use crate::instruments::lib::stop_notes_queue::StopNotesQueue;
use crate::music::note::get_notes_from_note_str_list;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{KeyboardPattern, Song};

pub struct Keyboardist {
    inner_instrument_name_16_chars: String,

    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    keys_with_queue: KeysWithQueue,
}
impl Keyboardist {
    pub fn new(
        inner_instrument_name_16_chars: String,
        keys_based_instrument: Box<dyn AbstractKeysBasedInstrument>,
    ) -> Self {
        Self {
            inner_instrument_name_16_chars,
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            keys_with_queue: KeysWithQueue::new(
                //
                StopNotesQueue::new(),
                keys_based_instrument,
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
            .stop_notes_queued_on_this_1_16th(index_1_16th);

        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern = (index_1_16th - 1) % (bars_covered_by_pattern * 16) + 1;

            let keys_chords = &pattern.chords;

            // FIXME: This is slow
            let mut i = 0;
            for chord in keys_chords {
                if chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= chord.to_1_16th_incl
                {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }

            if 0 <= self.chord_index && self.chord_index < keys_chords.len() {
                let chord = &keys_chords[self.chord_index];
                let chord_notes = get_notes_from_note_str_list(&chord.notes);

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
                    self.keys_with_queue.attack_notes(&chord_notes);
                }
                if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Here we check if this Chord's Notes should end on the *next* of this 1/16th
                    // (so after current 1/16th) using variable "index_1_16th_for_pattern".
                    // Queueing Notes to be stopped using "index_1_16th" since Stop Notes Queue uses
                    // absolute 1/16ths Indexes.
                    self.keys_with_queue
                        .notify_release_notes_at(&chord_notes, index_1_16th + 1);
                }
            }
        }

        // Preparing the next hit!
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section(&song);
        }
    }
}
impl Instrument for Keyboardist {
    fn get_instrument_name_16_chars(&self) -> String {
        // TODO: Avoid cloning Instrument Name
        self.inner_instrument_name_16_chars.clone()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            let keys_chords = &pattern.chords;
            if self.chord_index < keys_chords.len() {
                let chord = &keys_chords[self.chord_index];
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
