use crate::music::note::Note;

pub struct Chord {
    pub root_note: Note,
    pub chord_type: ChordType,
    pub inversion: i8,
}
impl Chord {
    pub fn new(root_note: Note, chord_type: ChordType, inversion: i8) -> Self {
        Self {
            root_note,
            chord_type,
            inversion,
        }
    }
    pub fn new_from_string(chord_name: String) -> Self {
        Self::new_from_string_inversion(chord_name, 0)
    }
    pub fn new_from_string_inversion(chord_name: String, inversion: i8) -> Self {
        // TODO: Notes with "octave=7" are not supported
        if chord_name.ends_with("maj7") {
            let note_str = &chord_name[..(chord_name.len() - 4)];
            Self {
                root_note: Note::new(note_str),
                chord_type: ChordType::MAJOR7,
                inversion,
            }
        } else if chord_name.ends_with("m7") {
            let note_str = &chord_name[..(chord_name.len() - 2)];
            Self {
                root_note: Note::new(note_str),
                chord_type: ChordType::MINOR7,
                inversion,
            }
        } else if chord_name.ends_with("7") {
            let note_str = &chord_name[..(chord_name.len() - 1)];
            Self {
                root_note: Note::new(note_str),
                chord_type: ChordType::DOMINANT7,
                inversion,
            }
        } else if chord_name.ends_with("m") {
            let note_str = &chord_name[..(chord_name.len() - 1)];
            Self {
                root_note: Note::new(note_str),
                chord_type: ChordType::MINOR,
                inversion,
            }
        } else {
            Self {
                root_note: Note::new(&chord_name),
                chord_type: ChordType::MAJOR,
                inversion,
            }
        }
    }
    pub fn get_notes(&self) -> Vec<Note> {
        // TODO: Here we clone Notes
        // "inversion=0" => Canonical Form       : "CEG  "
        // "inversion=1" => First Inversion Form : " EGC "
        // "inversion=2" => Second Inversion Form: "  GCE"
        let chord_notes = match self.chord_type {
            ChordType::MAJOR => {
                vec![
                    self.root_note.clone(),
                    self.root_note.get_major_third(),
                    self.root_note.get_fifth(),
                ]
            }
            ChordType::MINOR => {
                vec![
                    self.root_note.clone(),
                    self.root_note.get_minor_third(),
                    self.root_note.get_fifth(),
                ]
            }
            ChordType::MINOR7 => {
                vec![
                    self.root_note.clone(),
                    self.root_note.get_minor_third(),
                    self.root_note.get_fifth(),
                    self.root_note.get_minor_seventh(),
                ]
            }
            ChordType::MAJOR7 => {
                vec![
                    self.root_note.clone(),
                    self.root_note.get_major_third(),
                    self.root_note.get_fifth(),
                    self.root_note.get_major_seventh(),
                ]
            }
            ChordType::DOMINANT7 => {
                vec![
                    self.root_note.clone(),
                    self.root_note.get_major_third(),
                    self.root_note.get_fifth(),
                    self.root_note.get_minor_seventh(),
                ]
            }
        };
        if self.inversion > 0 {
            let mut result = Vec::new();
            let notes_to_move_up_from_left = self.inversion as usize;
            // Keep these Notes.
            for i in notes_to_move_up_from_left..chord_notes.len() {
                let chord_note = chord_notes[i].clone();
                result.push(chord_note);
            }
            // Use these Notes but higher.
            for i in 0..notes_to_move_up_from_left {
                let chord_note = chord_notes[i].get_octave_higher();
                result.push(chord_note);
            }
            result
        } else if self.inversion < 0 {
            let mut result = Vec::new();
            let notes_to_move_down_from_right = -self.inversion as usize;
            // Use these Notes but lower.
            for i in chord_notes.len() - notes_to_move_down_from_right..chord_notes.len() {
                let chord_note = chord_notes[i].get_octave_lower();
                result.push(chord_note);
            }
            // Keep these Notes.
            for i in 0..chord_notes.len() - notes_to_move_down_from_right {
                let chord_note = chord_notes[i].clone();
                result.push(chord_note);
            }
            result
        } else {
            // No Inversion.
            chord_notes
        }
    }
}
pub enum ChordType {
    // On C Major: C3maj7, D3m7, E3m7, F3maj7, G37, A3m7, B3m7(b5) (last one not supported...)
    MAJOR,     // C:     CEG
    MINOR,     // Am:    ACE
    MINOR7,    // Em7:   EGBD
    MAJOR7,    // Fmaj7: FACE
    DOMINANT7, // G7:    GBDF
}
