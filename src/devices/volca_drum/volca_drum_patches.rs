use crate::devices::volca_drum::volca_drum_patch::{
    VolcaDrumPatch, VolcaDrumPatchLayout, VolcaDrumPatchLayoutAmpEg, VolcaDrumPatchLayoutModType,
    VolcaDrumPatchLayoutSoundSrcType,
};

fn get_volca_drum_patch_kick_acoustic_1() -> VolcaDrumPatchLayout {
    VolcaDrumPatchLayout {
        sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveSine,
        mod_type: VolcaDrumPatchLayoutModType::ModExp,
        amp_eg: VolcaDrumPatchLayoutAmpEg::EnvExp,
        level: 127,
        pitch: 20,
        eg_attack: 0,
        eg_release: 115,
        mod_amount: 42,
        mod_rate: 188,
    }
}
fn get_volca_drum_patch_kick_funny() -> VolcaDrumPatchLayout {
    VolcaDrumPatchLayout {
        sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveNoiseLPF,
        mod_type: VolcaDrumPatchLayoutModType::ModExp,
        amp_eg: VolcaDrumPatchLayoutAmpEg::EnvExp,
        level: 255,
        pitch: 59,
        eg_attack: 47,
        eg_release: 139,
        mod_amount: 255,
        mod_rate: 0,
    }
}
fn get_volca_drum_patch_hh_acoustic_1() -> VolcaDrumPatchLayout {
    VolcaDrumPatchLayout {
        sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveNoiseHPF,
        mod_type: VolcaDrumPatchLayoutModType::ModTri,
        amp_eg: VolcaDrumPatchLayoutAmpEg::EnvMul,
        level: 127,
        pitch: 48,
        eg_attack: 13,
        eg_release: 32,
        mod_amount: 74,
        mod_rate: 31,
    }
}

pub fn get_volca_drum_patch_1() -> VolcaDrumPatch {
    VolcaDrumPatch {
        kick: get_volca_drum_patch_kick_acoustic_1(),
        hh: get_volca_drum_patch_hh_acoustic_1(),
        snare: VolcaDrumPatchLayout {
            sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveSaw,
            mod_type: VolcaDrumPatchLayoutModType::ModExp,
            amp_eg: VolcaDrumPatchLayoutAmpEg::EnvAd,
            level: 127,
            pitch: 40,
            eg_attack: 5,
            eg_release: 50,
            mod_amount: 120,
            mod_rate: 120,
        },
        sound4: VolcaDrumPatchLayout {
            sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveSine,
            mod_type: VolcaDrumPatchLayoutModType::ModExp,
            amp_eg: VolcaDrumPatchLayoutAmpEg::EnvAd,
            level: 127,
            pitch: 37,
            eg_attack: 13,
            eg_release: 9,
            mod_amount: 9,
            mod_rate: 20,
        },
        sound5: VolcaDrumPatchLayout {
            sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveSine,
            mod_type: VolcaDrumPatchLayoutModType::ModExp,
            amp_eg: VolcaDrumPatchLayoutAmpEg::EnvAd,
            level: 127,
            pitch: 37,
            eg_attack: 13,
            eg_release: 9,
            mod_amount: 9,
            mod_rate: 20,
        },
        sound6: VolcaDrumPatchLayout {
            sound_src_type: VolcaDrumPatchLayoutSoundSrcType::WaveSine,
            mod_type: VolcaDrumPatchLayoutModType::ModExp,
            amp_eg: VolcaDrumPatchLayoutAmpEg::EnvAd,
            level: 127,
            pitch: 37,
            eg_attack: 13,
            eg_release: 9,
            mod_amount: 9,
            mod_rate: 20,
        },
    }
}
