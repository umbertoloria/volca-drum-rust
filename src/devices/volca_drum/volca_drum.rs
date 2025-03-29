use crate::devices::volca_drum::volca_drum_patch::VolcaDrumPatch;
use crate::devices::volca_drum::volca_drum_patch_manager::VolcaDrumPatchManager;
use crate::devices::volca_drum::volca_drum_patches::get_volca_drum_patch_1;
use crate::midi::midi_device::MidiDevice;

pub const DRUM_CH_KICK: u8 = 0;
pub const DRUM_CH_HH: u8 = 1;
pub const DRUM_CH_SNARE: u8 = 2;
// TODO: Understand what's the "right" default note value.
const DEFAULT_NOTE_VALUE: u8 = 7;

pub struct VolcaDrum {
    pub device: Box<dyn MidiDevice>,
    pub patch_manager: VolcaDrumPatchManager,
}
impl VolcaDrum {
    pub fn new(mut device: Box<dyn MidiDevice>) -> Self {
        let default_volca_drum_patch = get_volca_drum_patch_1();
        let patch_manager = VolcaDrumPatchManager::new(&mut device, default_volca_drum_patch);
        Self {
            device,
            patch_manager,
        }
    }

    // HIGH LEVEL
    pub fn hit_hh(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_HH);
    }
    pub fn hit_kick(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_KICK);
    }
    pub fn hit_snare(&mut self) {
        self.hit(DEFAULT_NOTE_VALUE, DRUM_CH_SNARE);
    }
    fn hit(&mut self, note: u8, instr: u8) {
        // TODO: Improve if possible
        const PROGRAM_CHANGE: u8 = 0xC0;
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x70;

        self.send_plain_message(PROGRAM_CHANGE, instr, 0);
        // let _ = volca_drum.send(&[PROGRAM_CHANGE, instr]);

        self.send_plain_message(NOTE_ON_MSG, note, VELOCITY);

        // Are we sure that no wait is fine?
        // sleep(Duration::from_millis(50));
        self.send_plain_message(NOTE_OFF_MSG, note, VELOCITY);
    }

    // SOUNDS (Layout 1 only)
    pub fn apply_sound(&mut self, volca_drum_patch: VolcaDrumPatch) {
        self.patch_manager
            .apply_sound(&mut self.device, volca_drum_patch);
    }

    // LOW LEVEL
    fn send_plain_message(&mut self, a: u8, b: u8, c: u8) {
        let _ = self.device.send(a, b, c);
    }
}
