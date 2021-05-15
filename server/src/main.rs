use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

mod data_types;
mod game_data;
mod game_engine;
mod game_types;
mod handler;
mod session_types;
mod shared_types;
mod ws;

#[tokio::main]
async fn main() {
    let clients: data_types::SafeClients = Arc::new(RwLock::new(HashMap::new()));
    let sessions: data_types::SafeSessions = Arc::new(RwLock::new(HashMap::new()));
    let game_states: data_types::SafeGameStates = Arc::new(RwLock::new(HashMap::new()));
    let game_dict: data_types::SafeGameDictionary = Arc::new(game_types::GameDictionary {
        card_dict: game_data::get_card_dictionary(),
        character_dict: game_data::get_character_dictionary(),
    });

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        // pass copies of our references for the client and sessions maps to our handler
        .and(warp::any().map(move || clients.clone()))
        .and(warp::any().map(move || sessions.clone()))
        .and(warp::any().map(move || game_states.clone()))
        .and(warp::any().map(move || game_dict.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route.or(ws_route).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["Content-Type"])
            .allow_methods(vec!["GET", "POST", "DELETE"]),
    );

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .expect("PORT must be a number");

    println!("[boot] server listening on port::{}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
