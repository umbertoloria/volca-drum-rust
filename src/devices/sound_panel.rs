use crate::devices::volca_drum::volca_drum::{VolcaDrum, DRUM_CH_HH, DRUM_CH_KICK, DRUM_CH_SNARE};
use crate::devices::volca_drum::volca_drum_patch::{VolcaDrumPatch, VolcaDrumPatchLayout};

pub struct SoundPanel<'a> {
    pub volca_drum: &'a mut VolcaDrum,
}
impl<'a> SoundPanel<'a> {
    pub fn new(volca_drum: &'a mut VolcaDrum) -> SoundPanel<'a> {
        Self { volca_drum }
    }
    pub fn set_from_patch(&mut self, patch: VolcaDrumPatch) {
        self.config_patch_onto_channel_and_layout1(DRUM_CH_KICK, patch.kick);
        self.volca_drum.mute_layer_2(DRUM_CH_KICK);

        self.config_patch_onto_channel_and_layout1(DRUM_CH_HH, patch.hh);
        self.volca_drum.mute_layer_2(DRUM_CH_HH);

        self.config_patch_onto_channel_and_layout1(DRUM_CH_SNARE, patch.snare);
        self.volca_drum.mute_layer_2(DRUM_CH_SNARE);

        // TODO: Use the other 3 sounds
    }
    pub fn config_patch_onto_channel_and_layout1(
        &mut self,
        channel: u8,
        patch_layout: VolcaDrumPatchLayout,
    ) {
        // println!("Applying patch: {:?}", patch_layout);
        self.volca_drum.set_patch_sound(
            channel,
            patch_layout.sound_src_type,
            patch_layout.mod_type,
            patch_layout.amp_eg,
        );
        self.volca_drum
            .set_patch_level(channel, patch_layout.level as u8);
        self.volca_drum
            .set_patch_pitch(channel, patch_layout.pitch as u8);
        self.volca_drum
            .set_patch_eg_att(channel, patch_layout.eg_attack as u8);
        self.volca_drum
            .set_patch_eg_rel(channel, patch_layout.eg_release as u8);
        self.volca_drum
            .set_patch_mod_amount(channel, patch_layout.mod_amount as u8);
        self.volca_drum
            .set_patch_mod_rate(channel, patch_layout.mod_rate as u8);
    }
}
