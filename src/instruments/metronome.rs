use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentMono;
use crate::instruments::lib::instrument_with_queued_note::InstrumentWithQueuedNote;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{Song, SongMetronomeData, SongMetronomeDataClickOn, SongSection};

pub struct Metronome {
    // Outputs
    queued_instrument: InstrumentWithQueuedNote,
}
impl Metronome {
    pub fn new(instrument: Box<dyn AbstractInstrumentMono>, log: bool) -> Self {
        Self {
            queued_instrument: InstrumentWithQueuedNote::new(instrument, log),
        }
    }
    fn play_1_16th(
        &mut self,
        tempo_snapshot: &TempoSnapshot,
        instrument_pattern: Option<&SongMetronomeData>,
    ) {
        let index_1_16th_sec = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.queued_instrument
            .stop_notes_queued_on_this_1_16th(index_1_16th_sec);

        // Assuming every 1/4th has 4 1/16ths.

        if let Some(metronome_data) = instrument_pattern {
            let one_click_every_n_1_16ths = match &metronome_data.click_on {
                SongMetronomeDataClickOn::OnEvery1_4ths => 4,
                SongMetronomeDataClickOn::OnEvery1_8ths => 2,
            };
            let offset_1_16th = (index_1_16th_sec - 1) % one_click_every_n_1_16ths;
            if offset_1_16th == 0 {
                // Hit the metronome!
                let note = &metronome_data.note;
                self.queued_instrument.attack_note(note);
                // Telling the Queued Instrument to stop playing this Chord at the very start of
                // the *next* 1/16th.
                let stop_on_start_of_index_1_16th_sec =
                    index_1_16th_sec + one_click_every_n_1_16ths;
                self.queued_instrument
                    .notify_release_note_at(note, stop_on_start_of_index_1_16th_sec);
            }
        }
    }
}
impl Instrument<SongMetronomeData> for Metronome {
    fn get_instrument_pattern<'a>(
        &mut self,
        song: &'a Song,
        song_section: &SongSection,
    ) -> Option<&'a SongMetronomeData> {
        match &song.metronome_data {
            Some(metronome_data) => Some(metronome_data),
            None => None,
        }
    }
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)
        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let (i_section, tempo_snapshot) = realtime_player.want_and_get_next_tempo_snapshot();

            let curr_song_section = &song.sections[i_section];
            let instrument_pattern = self.get_instrument_pattern(&song, &curr_song_section);
            self.play_1_16th(tempo_snapshot, instrument_pattern);

            realtime_player.prepare_next_1_16th();
        }

        // Stopping the Queued Instrument.
        // TODO: Empty the Queue instead of "guessing" on the "1"...
        self.queued_instrument.stop_notes_queued_on_this_1_16th(1);
    }
}
