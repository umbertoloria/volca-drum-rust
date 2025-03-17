use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::bass_with_queue::BassWithQueue;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{BassPattern, Song};

// FIXME: Too much similar to Keyboardist
// FIXME: Too few similar to Drummer
pub struct Bassist {
    inner_instrument_name_16_chars: String,

    // Charts
    curr_section_index: usize,
    pattern: Option<BassPattern>,
    chord_index: usize,

    // Outputs
    bass_with_queue: BassWithQueue,
}
impl Bassist {
    pub fn new(
        inner_instrument_name_16_chars: String,
        bass_based_instrument: Box<dyn AbstractBassBasedInstrument>,
    ) -> Self {
        Self {
            inner_instrument_name_16_chars,
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            bass_with_queue: BassWithQueue::new(bass_based_instrument),
        }
    }
    fn update_pattern_from_song_section(&mut self, song: &Song) {
        if self.curr_section_index < song.sections.len() {
            let current_song_section = &song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.keyboard_pattern_key {
                Some(bass_pattern_key) => {
                    let keyboard_pattern = song
                        .get_bass_pattern_from_key(bass_pattern_key.into())
                        .expect("Unable to find right Synth Pattern")
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
        self.bass_with_queue
            .stop_notes_queued_on_this_1_16th(index_1_16th);

        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern = (index_1_16th - 1) % (bars_covered_by_pattern * 16) + 1;

            let bass_chords = pattern.get_chords();

            // FIXME: This is slow
            let mut i = 0;
            for chord in &bass_chords {
                if chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= chord.to_1_16th_incl
                {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }

            if 0 <= self.chord_index && self.chord_index < bass_chords.len() {
                let chord = &bass_chords[self.chord_index];
                let bass_note = &chord.note;

                /*
                println!("play_1_16th:");
                println!(" > index_1_16th_for_pattern={index_1_16th_for_pattern}");
                println!(
                    " > 1/16ths [{}, {}]",
                    chord.from_1_16th_incl, chord.to_1_16th_incl
                );
                println!(" > chord notes={:?}", chord.note);
                */

                if index_1_16th_for_pattern == chord.from_1_16th_incl {
                    self.bass_with_queue.attack_note(bass_note);
                }
                if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Here we check if this Chord's Notes should end on the *next* of this 1/16th
                    // (so after current 1/16th) using variable "index_1_16th_for_pattern".
                    // Queueing Notes to be stopped using "index_1_16th" since Stop Notes Queue uses
                    // absolute 1/16ths Indexes.
                    self.bass_with_queue
                        .notify_release_note_at(bass_note, index_1_16th + 1);
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
impl Instrument for Bassist {
    fn get_instrument_name_16_chars(&self) -> String {
        // TODO: Avoid cloning Instrument Name
        self.inner_instrument_name_16_chars.clone()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            let bass_chords = pattern.get_chords();
            if self.chord_index < bass_chords.len() {
                let chord = &bass_chords[self.chord_index];
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
