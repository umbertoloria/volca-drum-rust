use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentPoly;
use crate::instruments::lib::instrument_with_queued_notes::InstrumentWithQueuedNotes;
use crate::players::realtime_player::TempoSnapshot;
use crate::song::song::{KeyboardPattern, Song, SongSection};

pub struct Keyboardist {
    // Outputs
    queued_instrument: InstrumentWithQueuedNotes,
}
impl Keyboardist {
    pub fn new(instrument: Box<dyn AbstractInstrumentPoly>, log: bool) -> Self {
        Self {
            queued_instrument: InstrumentWithQueuedNotes::new(instrument, log),
        }
    }
}
impl Instrument<KeyboardPattern> for Keyboardist {
    fn get_instrument_pattern<'a>(
        &mut self,
        song: &'a Song,
        song_section: &SongSection,
    ) -> Option<&'a KeyboardPattern> {
        match &song_section.keyboard_pattern_key {
            Some(keyboard_pattern_key) => Some(
                song.get_keyboard_pattern_from_key(keyboard_pattern_key)
                    .expect("Unable to find right Keyboard Pattern"),
            ),
            None => None,
        }
    }
    fn play_1_16th(
        &mut self,
        tempo_snapshot: &TempoSnapshot,
        instrument_pattern: Option<&KeyboardPattern>,
    ) {
        let index_1_16th_sec = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.queued_instrument
            .stop_notes_queued_on_this_1_16th(index_1_16th_sec);

        if let Some(pattern) = instrument_pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern =
                (index_1_16th_sec - 1) % (bars_covered_by_pattern * 16) + 1;

            let keys_chords = &pattern.chords;

            // FIXME: This is slow
            let mut chord = None;
            let mut i = 0;
            for _chord in keys_chords {
                if _chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= _chord.to_1_16th_incl
                {
                    chord = Some(_chord);
                    break;
                }
                i += 1;
            }

            if let Some(chord) = chord {
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
    }
    fn clean_up(&mut self) {
        // Stopping the Queued Instrument.
        // TODO: Empty the Queue instead of "guessing" on the "1"...
        self.queued_instrument.stop_notes_queued_on_this_1_16th(1);
    }
}
