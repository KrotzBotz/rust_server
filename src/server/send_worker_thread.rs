use std::{collections::HashMap, net::SocketAddr, sync::mpsc};
use tokio::net::tcp::OwnedWriteHalf;

pub enum SenderMessage {
    Send(SocketAddr, Vec<u8>),
    Add(SocketAddr, OwnedWriteHalf),
    Remove(SocketAddr),
}

pub fn start(send_rx: mpsc::Receiver<SenderMessage>) {
    let mut clients: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();

    loop {
        let message = send_rx.recv().unwrap();
        match message {
            SenderMessage::Send(addr, data) => {
                if let Some(client) = clients.get_mut(&addr) {
                    if let Err(err) = client.try_write(&data) {
                        println!("Error sending data to client: {}", err);
                    }
                }
            }
            SenderMessage::Add(addr, stream) => {
                clients.insert(addr, stream);
            }
            SenderMessage::Remove(addr) => {
                clients.remove(&addr);
            }
        }
    }
}
