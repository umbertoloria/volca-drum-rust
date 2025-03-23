use crate::song::song::Song;

pub trait Instrument {
    fn play_song(&mut self, song: Song, start_from_millis: u128);
}
