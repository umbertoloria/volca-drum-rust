use crate::volca_drum::VolcaDrum;

pub struct Keyboard {
    //
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            //
        }
    }

    pub fn set_pattern(&mut self) {
        //
    }

    pub fn get_short_info(&self) -> String {
        "Fmaj7".to_string()
    }

    pub fn play_1_16th(&self, cur_1_4: usize, cur_1_16: usize, volca_drum: &mut VolcaDrum) {
        //
    }

    fn hit(&self, note: u8, instr: u8, volca_drum: &mut VolcaDrum) {
        //
    }
}
