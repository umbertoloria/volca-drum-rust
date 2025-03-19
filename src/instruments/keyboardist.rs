use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentPoly;
use crate::instruments::lib::instrument_with_queued_notes::InstrumentWithQueuedNotes;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{KeyboardPattern, Song};

pub struct Keyboardist {
    inner_instrument_name_16_chars: String,

    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    queued_instrument: InstrumentWithQueuedNotes,
}
impl Keyboardist {
    pub fn new(
        inner_instrument_name_16_chars: String,
        instrument: Box<dyn AbstractInstrumentPoly>,
        log: bool,
    ) -> Self {
        Self {
            inner_instrument_name_16_chars,
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            queued_instrument: InstrumentWithQueuedNotes::new(instrument, log),
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
        let index_1_16th_sec = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.queued_instrument
            .stop_notes_queued_on_this_1_16th(index_1_16th_sec);

        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern =
                (index_1_16th_sec - 1) % (bars_covered_by_pattern * 16) + 1;

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
                let chord_notes = &chord.notes;

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
                    self.queued_instrument.attack_notes(chord_notes);
                }
                if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Telling the Queued Instrument to stop playing this Chord at the very start of
                    // the *next* 1/16th.
                    let stop_on_start_of_index_1_16th_sec =
                        (index_1_16th_sec + 1) % tempo_snapshot.get_tot_1_16ths_in_section();
                    self.queued_instrument
                        .notify_release_notes_at(chord_notes, stop_on_start_of_index_1_16th_sec);
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

        // Stopping the Queued Instrument.
        // TODO: Empty the Queue instead of "guessing" on the "1"...
        self.queued_instrument.stop_notes_queued_on_this_1_16th(1);
    }
}
