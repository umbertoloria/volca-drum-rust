use crate::devices::volca_drum::VolcaDrum;
use crate::instruments::abs::instrument::Instrument;
use crate::instruments::lib::abstract_instruments::AbstractInstrumentPolyDrumSounds;
use crate::instruments::lib::drum_sounds::DrumSound;
use crate::instruments::lib::instrument_with_queued_sounds::InstrumentWithQueuedDrumSounds;
use crate::players::realtime_player::{create_realtime_player, TempoSnapshot};
use crate::song::song::{DrumPattern, Song, SongSection};

pub struct Drummer {
    inner_instrument_name_16_chars: String,

    // Outputs
    volca_drum: VolcaDrum,
    queued_instrument: InstrumentWithQueuedDrumSounds,
}
impl Drummer {
    pub fn new(
        inner_instrument_name_16_chars: String,
        volca_drum: VolcaDrum,
        instrument: Box<dyn AbstractInstrumentPolyDrumSounds>,
        log: bool,
    ) -> Self {
        Self {
            inner_instrument_name_16_chars,
            volca_drum,
            queued_instrument: InstrumentWithQueuedDrumSounds::new(instrument, log),
        }
    }
    fn get_instrument_pattern(
        &mut self,
        song: &Song,
        song_section: &SongSection,
    ) -> Option<DrumPattern> {
        match &song_section.drum_pattern_key {
            Some(drum_pattern_key) => {
                let drum_pattern = song
                    .get_drum_pattern_from_key(drum_pattern_key.into())
                    .expect("Unable to find right Drum Pattern")
                    // TODO: Avoid cloning pattern
                    .clone();
                Some(drum_pattern)
            }
            None => None,
        }
    }
    fn play_1_16th(
        &mut self,
        tempo_snapshot: &TempoSnapshot,
        instrument_pattern: Option<DrumPattern>,
    ) {
        let index_1_16th_sec = tempo_snapshot.get_cur_1_16ths_in_section_from_1();
        self.queued_instrument
            .stop_notes_queued_on_this_1_16th(index_1_16th_sec);

        if let Some(pattern) = instrument_pattern {
            let index_1_16th = tempo_snapshot.get_cur_1_16ths_in_bar_from_1() - 1;

            let hh_symbol = pattern.hh.get(index_1_16th..=index_1_16th).unwrap();
            let sn_symbol = pattern.sn.get(index_1_16th..=index_1_16th).unwrap();
            let kk_symbol = pattern.kk.get(index_1_16th..=index_1_16th).unwrap();

            // TODO: Let Volca Drum commands happen via something abstract
            let mut attack_notes = Vec::new();
            if hh_symbol != " " {
                self.volca_drum.hit_hh();
                attack_notes.push(DrumSound::HH);
            }
            if kk_symbol != " " {
                self.volca_drum.hit_kick();
                attack_notes.push(DrumSound::KICK);
            }
            if sn_symbol != " " {
                self.volca_drum.hit_snare();
                attack_notes.push(DrumSound::SNARE);
            }
            if !attack_notes.is_empty() {
                self.queued_instrument.attack_notes(&attack_notes);
                // Telling the Queued Instrument to stop playing this Chord at the very start of
                // the *next* 1/16th.
                let stop_on_start_of_index_1_16th_sec =
                    (index_1_16th_sec + 4) % tempo_snapshot.get_tot_1_16ths_in_section();
                self.queued_instrument
                    .notify_release_notes_at(&attack_notes, stop_on_start_of_index_1_16th_sec)
            }
        }
    }
}
impl Instrument for Drummer {
    fn get_instrument_name_16_chars(&self) -> String {
        // TODO: Avoid cloning Instrument Name
        self.inner_instrument_name_16_chars.clone()
    }
    /*fn get_short_info(&self) -> String {
        if let Some(pattern) = &self.pattern {
            format!("part \"{}\"", pattern.key)
        } else {
            "no drums".to_string()
        }
    }*/
    fn play_song(&mut self, song: Song, start_from_millis: u128) {
        // TODO: Duplicated code (*hjk)
        let mut realtime_player = create_realtime_player(&song, start_from_millis);
        while realtime_player.has_next_song_instant() {
            let (i_section, tempo_snapshot) = realtime_player.want_and_get_next_tempo_snapshot();

            let curr_song_section = &song.sections[i_section];
            let instrument_pattern = self.get_instrument_pattern(&song, &curr_song_section);
            self.play_1_16th(tempo_snapshot, instrument_pattern);

            realtime_player.prepare_next_1_16th();
        }

        // Stopping the Queued Instrument.
        // TODO: Empty the Queue instead of "guessing" on the "1"...
        self.queued_instrument.stop_notes_queued_on_this_1_16th(1);
    }
}
