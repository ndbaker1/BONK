use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod event_types;
mod game_engine;
mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type Sessions = Arc<RwLock<HashMap<String, Session>>>;

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
}

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and(with_sessions(sessions.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route.or(ws_route).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["Content-Type"])
            .allow_methods(vec!["GET", "POST", "DELETE"]),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_sessions(
    sessions: Sessions,
) -> impl Filter<Extract = (Sessions,), Error = Infallible> + Clone {
    warp::any().map(move || sessions.clone())
}
