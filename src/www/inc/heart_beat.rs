use crate::models::game_server_manager::GameServerManager;
use crate::www::packet::RunnablePacket;
use std::net::SocketAddr;

/*
-------------------- HeartBeatPacket --------------------
*/
pub struct HeartBeatPacket {
    ip_address: SocketAddr,
}

//TODO: add heartbeats or look into tokio to see if it already implements
impl RunnablePacket for HeartBeatPacket {
    fn run(self: Self, game_server_manager: &mut GameServerManager) {
        // game_server_manager.heart_beat(self.ip_address);
    }
}
