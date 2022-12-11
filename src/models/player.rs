/*
-------------------- Player --------------------
*/
pub struct Player {
    pub ethereum_address: String,
    pub world_id: Option<i32>,
    pub login_token: Option<(String, i32)>,
}

impl Player {
    pub fn new(ethereum_address: String, login_token: String, world_id: i32) -> Player {
        Player {
            ethereum_address: ethereum_address,
            world_id: None,
            login_token: Some((login_token, world_id)),
        }
    }
}
