use crate::{
    models::game_server_manager::{GameServer, GameServerManager},
    www::packet::RunnablePacket,
};
use std::net::SocketAddr;

/*
-------------------- GameServerConnectPacket --------------------
*/
pub struct GameServerConnectPacket {
    pub game_server_id: SocketAddr,
    pub password: String,
    buffer: Vec<u8>,
}

impl GameServerConnectPacket {
    pub fn new(buffer: Vec<u8>, address: SocketAddr) -> Option<GameServerConnectPacket> {
        let mut buffer = buffer;
        let game_server_id = address;
        let password = "pw".to_string();

        //TODO: add password check
        if password == "password" {
            Some(GameServerConnectPacket {
                game_server_id: game_server_id,
                password: password,
                buffer: buffer,
            })
        } else {
            None
        }
    }
}

//TODO: update GameServer::new() to get all arguments
// impl RunnablePacket for GameServerConnectPacket {
//     fn run(&self, mut server_manager: &mut GameServerManager) {
//         // let game_server = GameServer::new(self.game_server_id);
//         // server_manager.add_game_server(game_server);
//     }
// }
