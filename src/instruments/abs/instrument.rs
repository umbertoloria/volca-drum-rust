use crate::song::song::{Song, SongSection};

pub trait Instrument<InstrumentPatternType> {
    fn get_instrument_pattern<'a>(
        &mut self,
        song: &'a Song,
        song_section: &SongSection,
    ) -> Option<&'a InstrumentPatternType>;
    fn play_song(&mut self, song: Song, start_from_millis: u128);
}
