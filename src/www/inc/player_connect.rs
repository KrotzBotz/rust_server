use crate::models::game_server_manager::GameServerManager;
use crate::www::packet::RunnablePacket;
use std::net::SocketAddr;

/*
-------------------- GameServerDisconnectPacket --------------------
*/
pub struct PlayerConnectPacket {
    pub ethereum_address: String,
    pub game_server_address: SocketAddr,
}

impl PlayerConnectPacket {
    pub fn new(buff: Vec<u8>, addres: SocketAddr) -> Result<PlayerConnectPacket, String> {
        let mut pos = 2;

        if buff.len() < 2 + 4 {
            return Err("Packet too short".to_string());
        }

        let string_size =
            i32::from_be_bytes([buff[pos], buff[pos + 1], buff[pos + 2], buff[pos + 3]]) as usize;

        pos += 4;

        if buff.len() < pos + string_size {
            return Err("Packet too short".to_string());
        }
        let ethereum_address = String::from_utf8(buff[pos..pos + string_size].to_vec())
            .map_err(|_| "Invalid string".to_string())?;

        println!("PlayerConnectPacket: {}", ethereum_address);

        Ok(PlayerConnectPacket {
            ethereum_address,
            game_server_address: addres,
        })
    }
}

impl RunnablePacket for PlayerConnectPacket {
    fn run(self: Self, game_server_manager: &mut GameServerManager) {
        let game_server = game_server_manager.get_game_server_mut(self.game_server_address);
        if let Some(game_server) = game_server {
            game_server.add_player(self.ethereum_address);
        }
    }
}
