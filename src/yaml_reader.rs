use crate::player::Player;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlTypeSong {
    pub author: String,
    pub title: String,
    pub tempo_1_4: usize,
    pub default_drum_beat: String,
    pub sections: Vec<YamlTypeSection>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlTypeSection {
    pub kind: String,
    pub bars: usize,
    pub notes: Option<String>,
}

pub fn read_from_yaml(filepath: &str) -> YamlTypeSong {
    let contents = fs::read_to_string(filepath).expect("Unable to read file");

    let data_song: YamlTypeSong =
        serde_yaml::from_str(&contents).expect("Unable to parse YAML file");

    data_song
}
