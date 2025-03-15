pub trait AbstractKeysBasedInstrument {
    fn play_notes_start(&mut self, notes_str_list: &Vec<String>);
    fn play_notes_stop(&mut self, notes_str_list: &Vec<String>);
}
