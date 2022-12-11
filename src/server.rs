mod byte_process;
pub mod controllers;
pub mod decode_worker_thread;
pub mod http;
pub mod runnable_worker_thread;
pub mod send_worker_thread;

use crate::server::byte_process::IncomingBytes;
use parent_server::{get_host, ConnectionType};
use std::sync::{mpsc, Arc, Mutex};
use std::{collections::HashMap, net::SocketAddr};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use self::send_worker_thread::SenderMessage;

/*
-------------------- Server --------------------
*/
pub async fn init_tcp_server(
    decode_tx: mpsc::Sender<(Vec<u8>, SocketAddr)>,
    send_tx: mpsc::Sender<SenderMessage>,
) {
    /* client map */
    let client_map = Arc::new(Mutex::new(HashMap::new()));

    /* start server */
    let connection_ip = get_host(ConnectionType::TCP);
    println!("TCP server started on {}", connection_ip);
    let server = tokio::net::TcpListener::bind(&connection_ip).await.unwrap();

    /* listening for connections */
    loop {
        let (socket, address) = server.accept().await.unwrap();
        let decode_sender = decode_tx.clone();
        let send_tx = send_tx.clone();

        let client_map = Arc::clone(&client_map);

        tokio::spawn(async move {
            handle_connection(socket, address, decode_sender, send_tx, client_map).await;
        });
    }
}

/* sending buffered packets to threads */
async fn handle_connection(
    socket: tokio::net::TcpStream,
    address: SocketAddr,
    decode_sender: mpsc::Sender<(Vec<u8>, SocketAddr)>,
    send_tx: mpsc::Sender<SenderMessage>,
    client_map: Arc<Mutex<HashMap<SocketAddr, tokio::sync::mpsc::Sender<Vec<u8>>>>>,
) {
    /* callback to clean client disconnections */
    let clean_disconnect = || {
        println!("{} disconnected", address);
        client_map.lock().unwrap().remove(&address);
        // send_tx.send(SenderMessage::Remove(address)).unwrap();
    };

    let mut incoming_bytes = IncomingBytes::new();
    let (mut reader, writer) = TcpStream::into_split(socket);

    println!("Handshake complete {}", address);

    // TODO: make sure to read from buffer first

    if authenticate_connection(&incoming_bytes.bytes) {
        println!("{} authenticated", address);
        incoming_bytes.bytes = [0; 1028];
        // send_tx
        //     .send(SenderMessage::Add(address.clone(), writer))
        //     .unwrap();
    } else {
        println!("{} failed to authenticate", address);
        clean_disconnect();
        return;
    }

    /* read and write data to client */
    loop {
        tokio::select! {
            result = reader.read(&mut incoming_bytes.bytes) => {
                match result {
                    Ok(result) => {
                        if result == 0 {
                            clean_disconnect();
                            return;
                        }
                        let packets = incoming_bytes.process(result);
                        if packets.len() > 0 {
                            for packet in packets {
                                decode_sender.send((packet, address)).unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        clean_disconnect();
                        return;
                    }
                }
            },
        }
    }
}

fn authenticate_connection(buffer: &[u8; 1028]) -> bool {
    return true;
}
