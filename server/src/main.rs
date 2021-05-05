use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod event_types;
mod game_engine;
mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type SafeResource<T> = Arc<RwLock<T>>;

type Clients = HashMap<String, Client>;
type Sessions = HashMap<String, Session>;

type SafeClients = SafeResource<Clients>;
type SafeSessions = SafeResource<Sessions>;

// Data Stored for a Single User
#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub session_id: Option<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// Data Stored for a Game Sessions
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub game_state: Option<GameState>,
    pub owner: String,
    pub client_statuses: HashMap<String, bool>,
}
impl Session {
    fn get_client_count(&self) -> usize {
        self.client_statuses.len()
    }
    fn contains_client(&self, id: &str) -> bool {
        self.client_statuses.contains_key(id)
    }
    fn get_client_ids_vec(&self) -> Vec<String> {
        self.client_statuses
            .clone()
            .into_iter()
            .map(|(id, _)| id)
            .collect::<Vec<String>>()
    }
    fn remove_client(&mut self, id: &str) {
        self.client_statuses.remove(id);
    }
    fn insert_client(&mut self, id: &str, active_status: bool) {
        self.client_statuses.insert(id.to_string(), active_status);
    }
    fn get_clients_with_active_status(&self, active_status: bool) -> Vec<String> {
        self.client_statuses
            .clone()
            .into_iter()
            .filter(|(_, status)| status == &active_status)
            .map(|(id, _)| id)
            .collect::<Vec<String>>()
    }
    fn set_client_active(&mut self, id: &str) {
        if let Some(_) = self.client_statuses.get(id) {
            self.client_statuses.insert(id.to_string(), true);
        } else {
            println!(
                "[warning] tried to set active_status of client: {} but id was not found in session",
                id
            );
        }
    }
    fn set_client_inactive(&mut self, id: &str) {
        if let Some(_) = self.client_statuses.get(id) {
            self.client_statuses.insert(id.to_string(), false);
        } else {
            println!(
                "[warning] tried to set active_status of client: {} but id was not found in session",
                id
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientStatus {
    pub client_id: String,
    pub active: bool,
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
    let clients: SafeClients = Arc::new(RwLock::new(HashMap::new()));
    let sessions: SafeSessions = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        // pass copies of our references for the client and sessions maps to our handler
        .and(warp::any().map(move || clients.clone()))
        .and(warp::any().map(move || sessions.clone()))
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
