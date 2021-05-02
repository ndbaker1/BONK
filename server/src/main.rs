use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod event_types;
mod game_engine;
mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type SafeResource<T> = Arc<RwLock<T>>;
type Clients = SafeResource<HashMap<String, Client>>;
type Sessions = SafeResource<HashMap<String, Session>>;

// Data Stored for a Single User
#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    // pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// Data Stored for a Game Sessions
#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: String,
    pub client_ids: HashSet<String>,
    pub game_state: Option<GameState>,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub turn_index: usize,
    pub turn_orders: Vec<PlayerInfo>,
    pub player_blue_cards: HashMap<String, HashSet<BlueCards>>,
    pub player_green_cards: HashMap<String, HashSet<GreenCards>>,
    pub effect: EffectCodes,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub client_id: String,
    pub character: String,
}

#[derive(Debug, Clone)]
pub enum EffectCodes {
    GeneralStore = 1,
    None,
}

#[derive(Debug, Clone)]
pub enum BlueCards {
    Barrel = 1,
    Dynamite,
}

#[derive(Debug, Clone)]
pub enum GreenCards {}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        // Closures which pass the client and sessions maps to our handler
        .and(warp::any().map(move || clients.clone()))
        .and(warp::any().map(move || sessions.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route.or(ws_route).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["Content-Type"])
            .allow_methods(vec!["GET", "POST", "DELETE"]),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
