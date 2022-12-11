use crate::www::packet::Packet;
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

/*
-------------------- DecodeWorkerThread --------------------
*/
pub async fn start(decode_rx: Receiver<(Vec<u8>, SocketAddr)>, runnable_tx: Sender<Packet>) {
    loop {
        let (buffer, address) = decode_rx.recv().unwrap();
        let packet = Packet::new(buffer, address);
        if let Packet::Fail(err) = packet {
            print!("Error: {}", err);
            continue;
        }
        runnable_tx.send(packet).await;
    }
}
