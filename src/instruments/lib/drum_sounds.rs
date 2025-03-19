#[derive(Clone, Debug)]
pub enum DrumSound {
    KICK,
    HH,
    SNARE,
}
impl DrumSound {
    pub fn to_string(&self) -> String {
        match self {
            DrumSound::KICK => DRUMS_SOUND_KICK.into(),
            DrumSound::HH => DRUMS_SOUND_HH.into(),
            DrumSound::SNARE => DRUMS_SOUND_SNARE.into(),
        }
    }
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            DRUMS_SOUND_KICK => Some(DrumSound::KICK),
            DRUMS_SOUND_HH => Some(DrumSound::HH),
            DRUMS_SOUND_SNARE => Some(DrumSound::SNARE),
            &_ => None,
        }
    }
}

const DRUMS_SOUND_KICK: &'static str = "KICK";
const DRUMS_SOUND_HH: &'static str = "HH";
const DRUMS_SOUND_SNARE: &'static str = "SNARE";
