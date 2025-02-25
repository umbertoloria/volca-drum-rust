use crate::players::player::TempoSnapshot;

pub trait Instrument {
    fn get_instrument_name(&self) -> String;
    fn get_short_info(&self) -> String;
    fn teach_song(&mut self, song_id: String);
    fn play_song(&mut self, song_id: String, start_from_millis: u128);
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot);
}
