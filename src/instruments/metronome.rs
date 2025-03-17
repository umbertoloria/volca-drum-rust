use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_bass_based_instrument::AbstractBassBasedInstrument;
use crate::instruments::lib::bass_with_queue::BassWithQueue;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{Song, SongMetronomeDataClickOn};

pub struct Metronome {
    inner_instrument_name_16_chars: String,

    // Outputs
    bass_with_queue: BassWithQueue,
}
impl Metronome {
    pub fn new(
        inner_instrument_name_16_chars: String,
        bass_based_instrument: Box<dyn AbstractBassBasedInstrument>,
    ) -> Self {
        Self {
            inner_instrument_name_16_chars,
            bass_with_queue: BassWithQueue::new(bass_based_instrument),
        }
    }
    fn play_1_16th(&mut self, song: &Song, tempo_snapshot: &TempoSnapshot) {
        let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.bass_with_queue
            .stop_notes_queued_on_this_1_16th(index_1_16th);

        // Assuming every 1/4th has 4 1/16ths.

        let one_click_every_n_1_16ths = match &song.metronome_data.click_on {
            SongMetronomeDataClickOn::OnEvery1_4ths => 4,
            SongMetronomeDataClickOn::OnEvery1_8ths => 2,
        };
        let offset_1_16th = (index_1_16th - 1) % one_click_every_n_1_16ths;
        if offset_1_16th == 0 {
            // Hit the metronome!
            let note = &song.metronome_data.note;
            self.bass_with_queue.attack_note(note);
            self.bass_with_queue
                .notify_release_note_at(note, index_1_16th + one_click_every_n_1_16ths);
        }
    }
}
impl Instrument for Metronome {
    fn get_instrument_name_16_chars(&self) -> String {
        // TODO: Avoid cloning Instrument Name
        self.inner_instrument_name_16_chars.clone()
    }
    fn get_short_info(&self) -> String {
        "Metronome".to_string()
    }
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th(&song, tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }
    }
}
