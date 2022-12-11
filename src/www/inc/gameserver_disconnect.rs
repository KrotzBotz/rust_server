use crate::models::game_server_manager::GameServerManager;
use crate::www::packet::RunnablePacket;
use std::net::SocketAddr;

/*
-------------------- GameServerDisconnectPacket --------------------
*/
pub struct GameServerDisconnectPacket {
    ip_address: SocketAddr,
}

impl RunnablePacket for GameServerDisconnectPacket {
    fn run(self: Self, game_server_manager: &mut GameServerManager) {
        game_server_manager.remove_game_server(self.ip_address);
    }
}
