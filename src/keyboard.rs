use crate::instrument::Instrument;
use crate::player::TempoSnapshot;
use crate::song::{KeyboardPattern, Song};
use crate::volca_keys::VolcaKeys;
use std::collections::{HashMap, HashSet};
use std::process::exit;

pub struct Keyboard {
    // Song
    song: Song,

    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    stop_notes_queue: HashMap<usize, HashSet<String>>,
    volca_keys: VolcaKeys,
}
impl Keyboard {
    pub fn new(song: Song, volca_keys: VolcaKeys) -> Self {
        Self {
            song,
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            stop_notes_queue: HashMap::new(),
            volca_keys,
        }
    }
    fn update_pattern_from_song_section(&mut self) {
        if self.curr_section_index < self.song.sections.len() {
            let current_song_section = &self.song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.keyboard_pattern_key {
                Some(keyboard_pattern_key) => {
                    let keyboard_pattern = self
                        .song
                        .get_keyboard_pattern_from_key(keyboard_pattern_key.into())
                        .expect("Unable to find right Keyboard Pattern")
                        // TODO: Avoid cloning pattern
                        .clone();
                    Some(keyboard_pattern)
                }
                None => None,
            }
        } else {
            self.pattern = None;
        }
    }
    fn play_notes_start(&mut self, notes: &Vec<String>) {
        // println!("play_notes_start {:?}", notes);

        for note in notes {
            // TODO: Avoid cloning note
            let note_clone = note.clone();

            self.volca_keys.note_play_start(note_clone);
        }
    }
    fn play_notes_stop(&mut self, notes: &Vec<String>) {
        // println!("play_notes_stop {:?}", notes);

        for note in notes {
            // TODO: Avoid cloning note
            let note_clone = note.clone();

            self.volca_keys.note_play_stop(note_clone);
        }
    }
    fn add_notes_to_stop_notes_queue(&mut self, notes: &Vec<String>, index_1_16th: usize) {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.get_mut(&index_1_16th) {
            for note in notes {
                // TODO: Avoid cloning note
                let note_clone = note.clone();
                queued_notes_to_stop.insert(note_clone);
            }
        } else {
            let mut queued_notes_to_stop = HashSet::new();
            for note in notes {
                // TODO: Avoid cloning note
                let note_clone = note.clone();
                queued_notes_to_stop.insert(note_clone);
            }
            self.stop_notes_queue
                .insert(index_1_16th, queued_notes_to_stop);
        }
    }
    fn dequeue_notes_at_this_1_16th_from_stop_notes_queue(&mut self, index_1_16th: usize) {
        // Param "index_1_16th" starts from 1.
        if let Some(queued_notes_to_stop) = self.stop_notes_queue.remove(&index_1_16th) {
            let mut notes_to_stop = Vec::new();
            for note in queued_notes_to_stop {
                notes_to_stop.push(note);
            }
            self.play_notes_stop(&notes_to_stop);
        }
    }
}
impl Instrument for Keyboard {
    fn get_instrument_name(&self) -> String {
        "Keyboard".into()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            if self.chord_index < pattern.chords.len() {
                let mut chord = &pattern.chords[self.chord_index];
                return format!("part \"{}\" / {} chord", pattern.key, chord.chord_name);
            }
        }
        "".to_string()
    }
    fn teach_song(&mut self, song_id: String) {
        if self.song.id != song_id {
            println!("Keyboard doesn't know the song");
            exit(0x0100);
        }
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section();
    }
    fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.dequeue_notes_at_this_1_16th_from_stop_notes_queue(index_1_16th);

        if let Some(pattern) = &self.pattern {
            let bars_covered_by_pattern = pattern.get_ceil_num_bars_coverage();
            // Adjusting because we may have 4 bars patter onto 8 bars section.
            let index_1_16th_for_pattern = (index_1_16th - 1) % (bars_covered_by_pattern * 16) + 1;

            // TODO: This is slow
            let mut i = 0;
            for chord in &pattern.chords {
                if chord.from_1_16th_incl <= index_1_16th_for_pattern
                    && index_1_16th_for_pattern <= chord.to_1_16th_incl
                {
                    self.chord_index = i;
                    break;
                }
                i += 1;
            }

            if 0 <= self.chord_index && self.chord_index < pattern.chords.len() {
                // TODO: Avoid cloning pattern
                let pattern = self.pattern.clone().unwrap();
                let chord = &pattern.chords[self.chord_index];

                /*
                println!("play_1_16th:");
                println!(" > index_1_16th_for_pattern={index_1_16th_for_pattern}");
                println!(
                    " > 1/16ths [{}, {}]",
                    chord.from_1_16th_incl, chord.to_1_16th_incl
                );
                println!(" > chord notes={:?}", chord.notes);
                */

                if index_1_16th_for_pattern == chord.from_1_16th_incl {
                    self.play_notes_start(&chord.notes);
                } else if index_1_16th_for_pattern == chord.to_1_16th_incl {
                    // Here we check if this Chord's Notes should end on the *next* of this 1/16th
                    // (so after current 1/16th) using variable "index_1_16th_for_pattern".
                    // Queueing Notes to be stopped using "index_1_16th" since Stop Notes Queue uses
                    // absolute 1/16ths Indexes.
                    self.add_notes_to_stop_notes_queue(&chord.notes.clone(), index_1_16th + 1);
                } else {
                    // Notes are still playing.
                }
            }
        }

        // Preparing the next hit!
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&self.song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section();
        }
    }
}
