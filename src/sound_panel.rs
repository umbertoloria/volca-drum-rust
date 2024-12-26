use midir::MidiOutputConnection;

pub struct SoundPanel<'a> {
    pub conn: &'a mut MidiOutputConnection,
}
impl SoundPanel<'_> {
    pub fn set_sound_source_type(&mut self, channel: u8, sound_source_type: SoundSourceType) {
        match sound_source_type {
            SoundSourceType::WaveSine => self.send_cc_message(channel, 14, 24),
            SoundSourceType::WaveSaw => self.send_cc_message(channel, 14, 50),
            SoundSourceType::WaveNoiseHPF => self.send_cc_message(channel, 14, 76),
            SoundSourceType::WaveNoiseLPF => self.send_cc_message(channel, 14, 101),
            SoundSourceType::WaveNoiseBPF => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_modulation_type(&mut self, channel: u8, modulation_type: ModulationType) {
        match modulation_type {
            ModulationType::ModExp => self.send_cc_message(channel, 14, 109),
            ModulationType::ModTri => self.send_cc_message(channel, 14, 118),
            ModulationType::ModRand => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_amp_eg(&mut self, channel: u8, amp_eg: AmpEg) {
        match amp_eg {
            AmpEg::EnvAd => self.send_cc_message(channel, 14, 121),
            AmpEg::EnvExp => self.send_cc_message(channel, 14, 124),
            AmpEg::EnvMul => self.send_cc_message(channel, 14, 127),
        }
    }
    pub fn set_param_level(&mut self, channel: u8, param: ParamSoundType, value: u8) {
        match param {
            ParamSoundType::Level => self.send_cc_message(channel, 17, value),
            ParamSoundType::Pitch => self.send_cc_message(channel, 26, value),
            ParamSoundType::EgAttack => self.send_cc_message(channel, 20, value),
            ParamSoundType::EgRelease => self.send_cc_message(channel, 23, value),
            ParamSoundType::ModAmount => self.send_cc_message(channel, 29, value),
            ParamSoundType::ModRate => self.send_cc_message(channel, 46, value),
        }
    }

    fn send_cc_message(&mut self, channel: u8, cc_number: u8, value: u8) {
        let message: [u8; 3] = [
            // 1
            0xb0 | (channel & 0x0f),
            // 2
            cc_number & 0x7f,
            // 3
            value & 0x7f,
        ];
        let _ = self.conn.send(&message);
        // println!("Send msg -> {:?}", message);
    }
}

enum SoundSourceType {
    WaveSine,
    WaveSaw,
    WaveNoiseHPF,
    WaveNoiseLPF,
    WaveNoiseBPF,
}
enum ModulationType {
    ModExp,
    ModTri,
    ModRand,
}
enum AmpEg {
    EnvAd,
    EnvExp,
    EnvMul,
}
pub enum ParamSoundType {
    Level,
    Pitch,
    EgAttack,
    EgRelease,
    ModAmount,
    ModRate,
}
