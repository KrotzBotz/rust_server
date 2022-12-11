use std::collections::HashMap;

use tokio::sync::mpsc::Sender;

use crate::models::token_store::{LoginToken, Store, Token};
use crate::server::http::TokenRequest;
use crate::www::packet::Packet;
use rand::{distributions::Alphanumeric, Rng};
use reqwest;
//import lib
use parent_server::recover_address;
use warp::http;

const URL: &str = "";

pub async fn request_token(
    req: TokenRequest,
    store: Store,
    run_tx: Sender<Packet>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", req);
    let (tmp_tx, tmp_rx) = std::sync::mpsc::channel();

    if let Err(e) = run_tx
        .send(Packet::IsPlayerLoggedIn(req.address.clone(), tmp_tx))
        .await
    {
        println!("Error: {}", e);
        return Ok(warp::reply::with_status(
            "Internal Server Error",
            http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let logged_in = tmp_rx.recv().unwrap();

    if logged_in {
        return Ok(warp::reply::with_status(
            "Already logged in",
            http::StatusCode::BAD_REQUEST,
        ));
    }

    if let Ok(s) = recover_address(&req.message, &req.signature) {
        if s.to_lowercase() != req.address.to_lowercase() {
            return Ok(warp::reply::with_status(
                "Invalid signature",
                http::StatusCode::BAD_REQUEST,
            ));
        }
    }

    let nonce = req.message.split("\n").last().unwrap().to_string();

    let mut body = HashMap::new();
    body.insert("address", req.address.to_lowercase());

    let client = reqwest::Client::new();
    match client
        .post(URL.to_string() + "getNonce")
        .json(&body)
        .send()
        .await
    {
        Ok(result) => {
            let res = result.json().await;
            let res: HashMap<String, String> = match res {
                Ok(res) => res,
                Err(e) => {
                    println!("Error: {}", e);
                    return Ok(warp::reply::with_status(
                        "Internal Server Error",
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                }
            };
            let nonce2 = res.get("nonce");
            if let None = nonce2 {
                return Ok(warp::reply::with_status(
                    "Invalid address",
                    http::StatusCode::BAD_REQUEST,
                ));
            }
            let nonce2 = nonce2.unwrap();

            if nonce != *nonce2 {
                return Ok(warp::reply::with_status(
                    "Invalid nonce",
                    http::StatusCode::BAD_REQUEST,
                ));
            }
        }
        Err(_) => {
            return Ok(warp::reply::with_status(
                "Internal Server Error",
                http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
    let LoginToken: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    Ok(warp::reply::with_status("Nice", http::StatusCode::OK))
    // Ok(warp::reply::json(&result))
}

pub async fn get_token_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    // let mut result = HashMap::new();
    // let r = store.token_list.read();

    // for (key, value) in r.iter() {
    //     result.insert(key, value);
    // }

    // Ok(warp::reply::json(&result))
    Ok(warp::reply::with_status("Nice", http::StatusCode::OK))
}
