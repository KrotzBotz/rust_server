use crate::models::player::Player;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

/*
-------------------- GameServer --------------------
*/
pub struct GameServer {
    pub ip_address: SocketAddr,
    pub address: SocketAddr,
    pub max_players: i32,
    pub players: HashMap<String, Player>,
    pub last_heartbeat: i32,
    pub world_id: i32,
}

impl GameServer {
    pub fn new(
        ip_address: SocketAddr,
        address: SocketAddr,
        max_players: i32,
        world_id: i32,
    ) -> GameServer {
        GameServer {
            ip_address: ip_address,
            address: address,
            max_players: max_players,
            players: HashMap::new(),
            last_heartbeat: 0,
            world_id: world_id,
        }
    }

    pub fn add_player(&mut self, ethereum_address: String) {
        if self.players.len() < self.max_players as usize {
            self.players.insert(
                ethereum_address.clone(),
                Player::new(ethereum_address, self.world_id),
            );
        }
    }

    pub fn remove_player(&mut self, ethereum_address: String) {
        if self.players.contains_key(&ethereum_address) {
            self.players.remove(&ethereum_address);
        }
    }
}
/*
-------------------- GameServerManager --------------------
*/
pub struct GameServerManager {
    pub game_servers: HashMap<SocketAddr, GameServer>,
    pub login_tokens: HashMap<String, Player>,
    pub world_ids: HashMap<i32, SocketAddr>,
}

impl GameServerManager {
    pub fn new() -> GameServerManager {
        GameServerManager {
            game_servers: HashMap::new(),
            login_tokens: HashMap::new(),
            world_ids: HashMap::new(),
        }
    }

    pub fn add_game_server(&mut self, game_server: GameServer) {
        self.world_ids
            .insert(game_server.world_id, game_server.ip_address);
        self.game_servers
            .insert(game_server.ip_address, game_server);
    }

    pub fn remove_game_server(&mut self, game_ip: SocketAddr) {
        self.world_ids.remove(&self.game_servers[&game_ip].world_id);
        self.game_servers.remove(&game_ip);
    }

    pub fn get_game_server(&self, game_ip: SocketAddr) -> Option<&GameServer> {
        self.game_servers.get(&game_ip)
    }

    pub fn get_game_server_by_id(&self, world_id: i32) -> Option<&GameServer> {
        if let Some(game_ip) = self.world_ids.get(&world_id) {
            self.game_servers.get(game_ip)
        } else {
            None
        }
    }

    pub fn get_game_server_mut(&mut self, game_ip: SocketAddr) -> Option<&mut GameServer> {
        self.game_servers.get_mut(&game_ip)
    }

    pub fn add_login_token(
        &mut self,
        login_token: String,
        ethereum_address: &String,
        game_id: i32,
    ) {
        if self.is_logged_in(&ethereum_address) {
            return;
        }
        self.login_tokens.insert(
            ethereum_address.clone(),
            Player::new(ethereum_address.clone(), login_token, game_id),
        );
    }

    pub fn get_game_server_count(&self) -> i32 {
        self.game_servers.len() as i32
    }

    pub fn get_player_count(&self) -> i32 {
        let mut player_count = 0;
        for (_, game_server) in self.game_servers.iter() {
            player_count += game_server.players.len() as i32;
        }
        player_count
    }

    pub fn get_player_count_in_game(&self, game_ip: SocketAddr) -> i32 {
        let server = self.game_servers.get(&game_ip);
        match server {
            Some(game_server) => {
                return game_server.players.len() as i32;
            }
            None => return 0,
        }
    }

    pub fn game_server_from_player(&self, ethereum_address: String) -> Option<SocketAddr> {
        for (_, game_server) in self.game_servers.iter() {
            if game_server.players.contains_key(&ethereum_address) {
                return Some(game_server.ip_address.clone());
            }
        }
        None
    }

    pub fn is_logged_in(&self, ethereum_address: &String) -> bool {
        for (_, game_server) in self.game_servers.iter() {
            if game_server.players.contains_key(ethereum_address) {
                return true;
            }
        }
        false
    }

    //TODO: Implement server heartbeats
    pub fn servers_heartbeat(&mut self) {}
}
