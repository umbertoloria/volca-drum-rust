use crate::instruments::abs::instrument::Instrument;
use crate::song::song::Song;
use crate::thread_comm::thread_comm::{
    create_thread_comm_instances, ThreadCommReceiver, ThreadCommSender,
};

pub enum InstrumentCommCommand {
    PlaySong(Song, u128),
}
pub fn create_instrument_comm() -> (InstrumentCommSender, InstrumentCommReceiver) {
    let (sender, receiver) = create_thread_comm_instances::<InstrumentCommCommand>();
    let instrument_comm_sender = InstrumentCommSender::new(sender);
    (instrument_comm_sender, receiver)
}

pub struct InstrumentCommSender {
    sender: ThreadCommSender<InstrumentCommCommand>,
}
impl InstrumentCommSender {
    pub fn new(sender: ThreadCommSender<InstrumentCommCommand>) -> Self {
        Self { sender }
    }
    pub fn send_command_play_song(&self, song: Song, start_from_millis: u128) {
        let command = InstrumentCommCommand::PlaySong(song, start_from_millis);
        self.sender.send(command);
    }
    /*pub fn send_command_shutdown(&self) {
        let command = InstrumentCommCommand::Shutdown;
        self.sender.send(command);
    }*/
}
pub type InstrumentCommReceiver = ThreadCommReceiver<InstrumentCommCommand>;

// INSTRUMENT BROADCAST COMM
pub struct InstrumentBroadcastComm {
    pub instrument_comm_senders_list: Vec<InstrumentCommSender>,
}
impl InstrumentBroadcastComm {
    pub fn play_song(&mut self, song: Song, start_from_millis: u128) {
        for instrument_comm_sender in &self.instrument_comm_senders_list {
            instrument_comm_sender.send_command_play_song(song.clone(), start_from_millis);
        }
    }
    /*pub fn shutdown(&self) {
        for instrument_comm_sender in &self.instrument_comm_senders_list {
            instrument_comm_sender.send_command_shutdown();
        }
    }*/
}
pub fn start_listening_to_instrument_comm_commands<InstrumentPatternType>(
    instrument_comm_receiver: InstrumentCommReceiver,
    instrument: &mut impl Instrument<InstrumentPatternType>,
) {
    for command in instrument_comm_receiver.get_recv_iter() {
        match command {
            InstrumentCommCommand::PlaySong(song, start_from_millis) => {
                instrument.play_song(song, start_from_millis);
            }
        }
    }
}
