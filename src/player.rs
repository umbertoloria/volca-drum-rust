use crate::drummer::Drummer;
use crate::yaml_song_reader::YamlSong;
use midir::MidiOutputConnection;
use std::thread::sleep;
use std::time::Duration;

// Durations
pub const DUR_1_4: Duration = Duration::from_millis(1000);
pub const DUR_1_8: Duration = Duration::from_millis(500);
pub const DUR_1_16: Duration = Duration::from_millis(250);
pub const DUR_1_32: Duration = Duration::from_millis(125);
pub const BPM_DEFAULT: f64 = 60.0;

pub fn play_song(song: YamlSong, volca_drum: &mut MidiOutputConnection) {
    println!("Play song \"{}\" by \"{}\"", song.title, song.author);

    let mut player = Player::new(song.tempo_1_4);

    for section in &song.sections {
        // Print section info
        let mut section_notes = "";
        if let Some(x) = &section.notes {
            section_notes = x;
        }
        println!("New section: type {:6} -> {}", section.kind, section_notes);

        if section.bars < 1 {
            continue;
        }

        // Assuming bars or 4/4. Assuming 1/4 is two 1/8s.
        for _ in 0..(section.bars * 8) {
            player.play_1_8_now(volca_drum);
            player.next_1_8();
        }
    }
    println!("Song end.");
}

pub struct Player {
    // Compose-time.
    bpm: usize,
    // Assuming all 4/4 bars.
    // Assuming bpm ticks to 1/4.

    // Play-time.
    cur_bar: usize,
    cur_quarter: usize,
    cur_1_8: usize,

    // Musicians
    drummer: Drummer,
}
impl Player {
    pub fn new(bpm: usize) -> Self {
        Self {
            bpm,
            cur_bar: 1,
            cur_quarter: 1,
            cur_1_8: 1,
            drummer: Drummer::new(),
        }
    }
    pub fn play_1_8_now(&self, volca_drum: &mut MidiOutputConnection) {
        // Play music
        self.drummer
            .play_1_8(self.cur_quarter, self.cur_1_8, volca_drum);
        println!(
            "Bar={}, Quarter={}.{}",
            self.cur_bar, self.cur_quarter, self.cur_1_8
        );

        // Wait time
        let millis_1_8 = DUR_1_8.mul_f64(BPM_DEFAULT).div_f64(self.bpm as f64); // One eighth.
        sleep(millis_1_8);
    }
    pub fn next_1_8(&mut self) {
        self.cur_1_8 += 1;
        if self.cur_1_8 > 2 {
            self.cur_1_8 = 1;
            self.cur_quarter += 1;
        }
        if self.cur_quarter > 4 {
            self.cur_quarter = 1;
            self.cur_bar += 1;
        }
    }
}
