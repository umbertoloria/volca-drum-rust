use crate::player::TempoSnapshot;

pub trait Instrument {
    fn get_instrument_name(&self) -> String;
    fn get_short_info(&self) -> String;
    fn teach_song(&mut self, song_id: String);
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot);
}
