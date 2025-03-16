use crate::song::song::Song;

pub trait Instrument {
    fn get_instrument_name_16_chars(&self) -> String;
    fn get_short_info(&self) -> String;
    fn play_song(&mut self, song: Song, start_from_millis: u128);
}
