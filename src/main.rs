mod models;
mod server;
mod services;
mod www;

use crate::server::decode_worker_thread;
use crate::server::http::init_http_server;
use crate::server::init_tcp_server;
use crate::server::runnable_worker_thread;
use crate::server::send_worker_thread;
use crate::server::send_worker_thread::SenderMessage;
use crate::www::packet::Packet;

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tokio::sync::broadcast;

type ClientMap = Arc<Mutex<HashMap<SocketAddr, tokio::sync::mpsc::Sender<Vec<u8>>>>>;
type ByteCh = (
    mpsc::Sender<(Vec<u8>, SocketAddr)>,
    mpsc::Receiver<(Vec<u8>, SocketAddr)>,
);
type PacketCh = (
    tokio::sync::mpsc::Sender<Packet>,
    tokio::sync::mpsc::Receiver<Packet>,
);
type SendCh = (mpsc::Sender<SenderMessage>, mpsc::Receiver<SenderMessage>);

#[tokio::main]
async fn main() -> io::Result<()> {
    let client_map = ClientMap::default();

    /* create channels */
    #[allow(unused)]
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<Vec<u8>>(100);
    let (decode_tx, decode_rx): ByteCh = mpsc::channel();
    let (run_tx, run_rx): PacketCh = tokio::sync::mpsc::channel(100);
    let (send_tx, send_rx): SendCh = mpsc::channel();
    let run_clone = run_tx.clone();

    thread::spawn(move || async {
        decode_worker_thread::start(decode_rx, run_clone).await;
    });
    let run_clone = run_tx.clone();
    let send_clone = send_tx.clone();
    thread::spawn(move || async {
        runnable_worker_thread::start(run_rx, send_clone).await;
    });
    thread::spawn(move || {
        send_worker_thread::start(send_rx);
    });

    let http = init_http_server(run_clone); /* start worker threads */
    let tcp = init_tcp_server(decode_tx.clone(), send_tx.clone());
    tokio::join!(http, tcp);

    Ok(())
}
