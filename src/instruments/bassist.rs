use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentMono;
use crate::instruments::lib::instrument_with_queued_note::InstrumentWithQueuedNote;
use crate::players::realtime_player::TempoSnapshot;
use crate::song::song::{BassPattern, Song, SongSection};

pub struct Bassist {
    // Outputs
    queued_instrument: InstrumentWithQueuedNote,
}
impl Bassist {
    pub fn new(instrument: Box<dyn AbstractInstrumentMono>, log: bool) -> Self {
        Self {
            queued_instrument: InstrumentWithQueuedNote::new(instrument, log),
        }
    }
}
impl Instrument<BassPattern> for Bassist {
    fn get_instrument_pattern<'a>(
        &mut self,
        song: &'a Song,
        song_section: &SongSection,
    ) -> Option<&'a BassPattern> {
        match &song_section.bass_pattern_key {
            Some(bass_pattern_key) => Some(
                song.get_bass_pattern_from_key(bass_pattern_key)
                    .expect("Unable to find right Bass Pattern"),
            ),
            None => None,
        }
    }
    fn play_1_16th(
        &mut self,
        tempo_snapshot: &TempoSnapshot,
        instrument_pattern: Option<&BassPattern>,
    ) {
        let index_1_16th_sec = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.queued_instrument
            .stop_notes_queued_on_this_1_16th(index_1_16th_sec);

        if let Some(pattern) = instrument_pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern =
                (index_1_16th_sec - 1) % (bars_covered_by_pattern * 16) + 1;

            let bass_chords = pattern.get_chords();

            // FIXME: This is slow
            let mut chord = None;
            let mut i = 0;
            for _chord in &bass_chords {
                if _chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= _chord.to_1_16th_incl
                {
                    chord = Some(_chord);
                    break;
                }
                i += 1;
            }

            if let Some(chord) = chord {
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
                    self.queued_instrument.attack_note(bass_note);
                }
                if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Telling the Queued Instrument to stop playing this Chord at the very start of
                    // the *next* 1/16th.
                    let stop_on_start_of_index_1_16th_sec =
                        (index_1_16th_sec + 1) % tempo_snapshot.get_tot_1_16ths_in_section();
                    self.queued_instrument
                        .notify_release_note_at(bass_note, stop_on_start_of_index_1_16th_sec);
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
