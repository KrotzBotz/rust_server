use std::env;
use web3::signing::{keccak256, recover};
/*
-------------------- getting TCP ip/port --------------------
*/

pub enum ConnectionType {
    TCP,
    UDP,
    HTTP,
}

const HOST: &str = ":";
const LOCAL_HOST: &str = "127.0.0.1:";

#[allow(unused)]
const BUFFER_SIZE: usize = 1024;

pub fn get_host(con_type: ConnectionType) -> String {
    let args: Vec<String> = env::args().collect();
    let mut host = String::new();
    if args.len() > 1 {
        if args[1] == "-remote" {
            host.push_str(HOST);
        }
    } else {
        host.push_str(LOCAL_HOST);
    }
    match con_type {
        ConnectionType::TCP => {
            host.push_str("8080");
        }
        ConnectionType::UDP => {
            host.push_str("8081");
        }
        ConnectionType::HTTP => {
            host.push_str("8082");
        }
    };
    return host;
}

/*
-------------------- Recover Eth Address --------------------

*/

pub fn eth_message(message: &String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

pub fn recover_address(message: &String, sig: &String) -> Result<String, String> {
    let message = eth_message(message);
    let sig = &sig[2..];
    let signature = hex::decode(sig);
    let signature = match signature {
        Ok(sig) => sig,
        Err(_) => {
            return Err("Invalid signature".to_string());
        }
    };

    let pubkey = recover(&message, &signature[..64], 0);
    let pubkey = match pubkey {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Err("Invalid signature".to_string());
        }
    };
    let pubkey = format!("{:02X?}", pubkey);
    Ok(pubkey)
}
