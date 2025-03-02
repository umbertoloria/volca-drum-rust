use crate::instruments::instrument::Instrument;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct InstrumentBroadcastComm {
    pub tx_list: Vec<Sender<InstrumentCommCommand>>,
}
impl InstrumentBroadcastComm {
    pub fn play_song(&mut self, song_id: String, start_from_millis: u128) {
        for tx in &self.tx_list {
            tx.send(InstrumentCommCommand::PlaySong(
                song_id.clone(),
                start_from_millis,
            ))
            .unwrap();
        }
    }
    /*pub fn play_1_16th(&mut self, tempo_snapshot: &TempoSnapshot) {
        for tx in &self.tx_list {
            let cloned_tempo_snapshot = tempo_snapshot.clone();
            let instr_comm_command = InstrCommCommand::PlayHit(cloned_tempo_snapshot);
            tx.send(instr_comm_command).unwrap();
        }
    }*/
    pub fn shutdown(&self) {
        for tx in &self.tx_list {
            tx.send(InstrumentCommCommand::Shutdown).unwrap();
        }
    }
}

#[derive(Debug)]
pub enum InstrumentCommCommand {
    PlaySong(String, u128),
    // PlayHit(TempoSnapshot), // Deprecated.
    Shutdown,
}
pub fn create_instrument_comm() -> (
    Sender<InstrumentCommCommand>,
    Receiver<InstrumentCommCommand>,
) {
    mpsc::channel::<InstrumentCommCommand>()
}

pub fn start_listening_to_instrument_comm_commands(
    rx_instrument: Receiver<InstrumentCommCommand>,
    instrument: &mut impl Instrument,
) {
    for received in rx_instrument {
        match received {
            InstrumentCommCommand::PlaySong(song_id, start_from_millis) => {
                instrument.play_song(song_id, start_from_millis);
            }
            InstrumentCommCommand::Shutdown => {
                break;
            }
        }
    }
}
