use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Store {
    token_list: Arc<RwLock<HashMap<String, Token>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            token_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
    address: String,
    token: String,
    world: i32,
}

pub enum LoginToken {
    Pending(Token),
    Confirmed(i32),
}
