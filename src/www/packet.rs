use crate::models::game_server_manager::GameServerManager;
use crate::models::player::Player;
use crate::www::inc::gameserver_connect::GameServerConnectPacket;
use crate::www::inc::gameserver_disconnect::GameServerDisconnectPacket;
use crate::www::inc::heart_beat::HeartBeatPacket;
use crate::www::inc::player_connect::PlayerConnectPacket;
use crate::www::inc::player_disconnect::PlayerDisconnectPacket;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;

/*
-------------------- Packet --------------------
*/
pub enum Packet {
    PlayerConnect(PlayerConnectPacket),
    PlayerDisconnect(PlayerDisconnectPacket),
    GameServerConnect(GameServerConnectPacket),
    GameServerDisconnect(GameServerDisconnectPacket),
    HeartBeat(HeartBeatPacket),
    IsPlayerLoggedIn(String, Sender<bool>),
    PlayerToken(String, String, i32),
    Fail(String),
}

impl Packet {
    pub fn new(buff: Vec<u8>, socket: SocketAddr) -> Packet {
        if buff.len() < 2 {
            return Packet::Fail("Packet too small".to_string());
        }
        match i16::from_be_bytes([buff[0], buff[1]]) {
            2 => match PlayerConnectPacket::new(buff, socket) {
                Ok(p) => Packet::PlayerConnect(p),
                Err(s) => Packet::Fail(s),
            },
            3 => match PlayerDisconnectPacket::new(buff, socket) {
                Ok(p) => Packet::PlayerDisconnect(p),
                Err(s) => Packet::Fail(s),
            },
            _ => Packet::Fail("Unknown packet".to_string()),
        }
    }
}

impl RunnablePacket for Packet {
    fn run(self: Self, game_server_manager: &mut GameServerManager) {
        match self {
            Packet::PlayerConnect(packet) => packet.run(game_server_manager),
            Packet::PlayerDisconnect(packet) => packet.run(game_server_manager),
            Packet::HeartBeat(packet) => packet.run(game_server_manager),
            Packet::PlayerToken(address, token, world_id) => {}
            _ => {}
        }
    }
}

/*
-------------------- Traits --------------------
*/
pub trait RunnablePacket {
    fn run(self: Self, server_manager: &mut GameServerManager);
}
