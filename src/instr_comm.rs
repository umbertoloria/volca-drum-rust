use crate::instrument::Instrument;
use crate::player::TempoSnapshot;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct InstrComm {
    pub tx_list: Vec<Sender<InstrCommCommand>>,
}
impl InstrComm {
    pub fn teach_songs(&mut self, song_id: String) {
        for tx in &self.tx_list {
            tx.send(InstrCommCommand::TeachSong(song_id.clone()))
                .unwrap();
        }
    }
    pub fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        for tx in &self.tx_list {
            let cloned_tempo_snapshot = tempo_snapshot.clone();
            tx.send(InstrCommCommand::PlayHit(cloned_tempo_snapshot))
                .unwrap();
        }
    }
    pub fn shutdown(&self) {
        for tx in &self.tx_list {
            tx.send(InstrCommCommand::Shutdown).unwrap();
        }
    }
}

#[derive(Debug)]
pub enum InstrCommCommand {
    TeachSong(String),
    PlayHit(TempoSnapshot),
    Shutdown,
}
pub fn create_instr_comm() -> (Sender<InstrCommCommand>, Receiver<InstrCommCommand>) {
    mpsc::channel::<InstrCommCommand>()
}
pub fn start_listening_to_instr_comm_commands(
    rx_keyboard: Receiver<InstrCommCommand>,
    instrument: &mut impl Instrument,
) {
    for received in rx_keyboard {
        match received {
            InstrCommCommand::TeachSong(song_id) => {
                instrument.teach_song(song_id);
            }
            InstrCommCommand::PlayHit(tempo_signature) => {
                instrument.play_1_16th(&tempo_signature);
            }
            InstrCommCommand::Shutdown => {
                break;
            }
        }
    }
}
