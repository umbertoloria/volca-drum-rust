use crate::instruments::instrument::Instrument;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
enum InstrumentCommCommand {
    PlaySong(String, u128),
    // PlayHit(TempoSnapshot), // Deprecated.
    Shutdown,
}
pub fn create_instrument_comm() -> (InstrumentCommSender, InstrumentCommReceiver) {
    let (tx, rx) = mpsc::channel::<InstrumentCommCommand>();
    let instrument_comm_sender = InstrumentCommSender::new(tx);
    let instrument_comm_receiver = InstrumentCommReceiver::new(rx);
    (instrument_comm_sender, instrument_comm_receiver)
}

// INSTRUMENT COMM SENDER
pub struct InstrumentCommSender {
    tx: Sender<InstrumentCommCommand>,
}
impl InstrumentCommSender {
    pub fn new(tx: Sender<InstrumentCommCommand>) -> Self {
        Self { tx }
    }
    pub fn send_command_play_song(&self, song_id: String, start_from_millis: u128) {
        let command = InstrumentCommCommand::PlaySong(song_id, start_from_millis);
        self.tx.send(command).unwrap();
    }
    pub fn send_command_shutdown(&self) {
        let command = InstrumentCommCommand::Shutdown;
        self.tx.send(command).unwrap();
    }
}

// INSTRUMENT COMM RECEIVER
pub struct InstrumentCommReceiver {
    rx: Receiver<InstrumentCommCommand>,
}
impl InstrumentCommReceiver {
    pub fn new(rx: Receiver<InstrumentCommCommand>) -> Self {
        Self { rx }
    }
    pub fn loop_requests(self) -> Receiver<InstrumentCommCommand> {
        self.rx
    }
}

// INSTRUMENT BROADCAST COMM
pub struct InstrumentBroadcastComm {
    pub instrument_comm_senders_list: Vec<InstrumentCommSender>,
}
impl InstrumentBroadcastComm {
    pub fn play_song(&mut self, song_id: String, start_from_millis: u128) {
        for instrument_comm_sender in &self.instrument_comm_senders_list {
            instrument_comm_sender.send_command_play_song(song_id.clone(), start_from_millis);
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
        for instrument_comm_sender in &self.instrument_comm_senders_list {
            instrument_comm_sender.send_command_shutdown();
        }
    }
}
pub fn start_listening_to_instrument_comm_commands(
    instrument_comm_receiver: InstrumentCommReceiver,
    instrument: &mut impl Instrument,
) {
    for instrument_comm_command in instrument_comm_receiver.loop_requests() {
        match instrument_comm_command {
            InstrumentCommCommand::PlaySong(song_id, start_from_millis) => {
                instrument.play_song(song_id, start_from_millis);
            }
            InstrumentCommCommand::Shutdown => {
                break;
            }
        }
    }
}
