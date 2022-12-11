use crate::server::controllers::auth::request_token;
use crate::www::packet::Packet;
use crate::{models::token_store::Store, server::controllers};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use warp::{http, Filter};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenRequest {
    pub address: String,
    pub message: String,
    pub signature: String,
    pub world: i32,
}

fn post_json() -> impl Filter<Extract = (TokenRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn init_http_server(run_tx: Sender<Packet>) {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let run_tx_filter = warp::any().map(move || run_tx.clone());
    let update_item = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("auth"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and(run_tx_filter)
        .and_then(request_token);

    //use .or() to add more routes
    let routes = update_item;

    println!("HTTP server started on port 8080");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
