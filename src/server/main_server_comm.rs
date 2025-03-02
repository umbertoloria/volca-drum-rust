use crate::server::main_server::WSResponse;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

pub type BroadcastSenderToServerThread = Sender<WSResponse>;
pub type BroadcastReceiverToServerThread = Receiver<WSResponse>;
pub fn create_channel_for_server_thread() -> (
    BroadcastSenderToServerThread,
    BroadcastReceiverToServerThread,
) {
    // TODO: Adjust BUFFER_SIZE
    const BUFFER_SIZE: usize = 32;
    let (tx_to_web_server, rx_to_web_server) = broadcast::channel::<WSResponse>(BUFFER_SIZE);
    (tx_to_web_server, rx_to_web_server)
}
