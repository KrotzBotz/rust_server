use crate::server::send_worker_thread::SenderMessage;
use crate::www::packet::Packet;
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;

/*
-------------------- RunnableWorkerThread --------------------
*/
pub async fn start(mut runnable_rx: Receiver<Packet>, send_tx: Sender<SenderMessage>) {
    loop {
        let packet = runnable_rx.recv().await;
    }
}
