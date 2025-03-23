use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{Song, SongSection};

pub trait Instrument<InstrumentPatternType> {
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let (i_section, tempo_snapshot) = realtime_player.want_and_get_next_tempo_snapshot();

            let curr_song_section = &song.sections[i_section];
            let instrument_pattern = self.get_instrument_pattern(&song, &curr_song_section);
            self.play_1_16th(tempo_snapshot, instrument_pattern);

            realtime_player.prepare_next_1_16th();
        }
        self.clean_up();
    }
    fn get_instrument_pattern<'a>(
        &mut self,
        song: &'a Song,
        song_section: &SongSection,
    ) -> Option<&'a InstrumentPatternType>;
    fn play_1_16th(
        &mut self,
        tempo_snapshot: &TempoSnapshot,
        instrument_pattern: Option<&InstrumentPatternType>,
    );
    fn clean_up(&mut self);
}
