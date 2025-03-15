use crate::instruments::instrument::Instrument;
use crate::instruments::synth_thread::{create_synth_thread_comm, synth_thread, SynthCommand};
use crate::music::note::{get_notes_from_note_str_list, Note};
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{KeyboardPattern, Song};
use crate::thread_comm::thread_comm::ThreadCommSender;
use std::collections::{HashMap, HashSet};
use std::thread::JoinHandle;

// TODO: Avoid duplicate code between this and Keyboard
pub struct Synth {
    // Charts
    curr_section_index: usize,
    pattern: Option<KeyboardPattern>,
    chord_index: usize,

    // Outputs
    stop_notes_queue: HashMap<usize, HashSet<String>>,
    inner_synth: InnerSynth,
}
impl Synth {
    pub fn new() -> Self {
        Self {
            curr_section_index: 0,
            pattern: None,
            chord_index: 0,
            stop_notes_queue: HashMap::new(),
            inner_synth: InnerSynth::new(),
        }
    }
    fn update_pattern_from_song_section(&mut self, song: &Song) {
        if self.curr_section_index < song.sections.len() {
            let current_song_section = &song.sections[self.curr_section_index];
            self.pattern = match &current_song_section.keyboard_pattern_key {
                Some(keyboard_pattern_key) => {
                    let keyboard_pattern = song
                        .get_keyboard_pattern_from_key(keyboard_pattern_key.into())
                        .expect("Unable to find right Synth Pattern")
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
    fn play_1_16th(&mut self, song: &Song, tempo_snapshot: &TempoSnapshot) {
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
        // TODO: Avoid cloning Song
        if tempo_snapshot.is_this_the_last_1_16th_of_this_section(&song) {
            self.curr_section_index += 1;
            self.update_pattern_from_song_section(&song);
        }
    }
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>) {
        // println!("play_notes_start {:?}", notes);

        let notes = get_notes_from_note_str_list(notes_str_list);
        self.inner_synth.note_play_start(notes);
    }
    fn play_notes_stop(&mut self, notes: &Vec<String>) {
        // println!("play_notes_stop {:?}", notes);

        // TODO: Try to use "notes"
        self.inner_synth.note_play_stop();
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
impl Instrument for Synth {
    fn get_instrument_name_16_chars(&self) -> String {
        "Synth           ".into()
    }
    fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            if self.chord_index < pattern.chords.len() {
                let chord = &pattern.chords[self.chord_index];
                return format!("part \"{}\" / {} chord", pattern.key, chord.chord_name);
            }
        }
        "".to_string()
    }
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)
        // Start from beginning.
        self.curr_section_index = 0;
        self.update_pattern_from_song_section(&song);

        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let tempo_snapshot = realtime_player.want_and_get_next_tempo_snapshot();

            self.play_1_16th(&song, tempo_snapshot);

            realtime_player.prepare_next_1_16th();
        }
    }
}

struct InnerSynth {
    synth_thread_handle: JoinHandle<()>,
    synth_command_sender: ThreadCommSender<SynthCommand>,
}
impl InnerSynth {
    pub fn new() -> Self {
        let (synth_command_sender, synth_command_receiver) = create_synth_thread_comm();
        let synth_thread_handle = synth_thread(synth_command_receiver);
        Self {
            synth_thread_handle,
            synth_command_sender,
        }
    }
    pub fn note_play_start(&self, notes: Vec<Note>) {
        // println!("NOTES: {:?}", notes);
        self.synth_command_sender
            .send(SynthCommand::StartNotes(notes));
    }
    pub fn note_play_stop(&self) {
        self.synth_command_sender.send(SynthCommand::StopNote);
    }
}
