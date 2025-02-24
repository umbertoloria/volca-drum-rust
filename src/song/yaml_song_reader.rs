use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlSong {
    pub author: String,
    pub title: String,
    pub tempo_1_4: usize,
    pub default_drum_beat: String,
    pub sections: Vec<YamlSongSection>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlSongSection {
    pub kind: String,
    pub bars: usize,
    pub notes: Option<String>,
}

pub fn read_song_from_yaml(filepath: &str) -> YamlSong {
    let contents = fs::read_to_string(filepath).expect("Unable to read song file");

    let data_song: YamlSong =
        serde_yaml::from_str(&contents).expect("Unable to parse song YAML file");

    data_song
}
