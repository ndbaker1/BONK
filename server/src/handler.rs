use crate::{data_types, ws};
use warp::Rejection;
use warp::{http::StatusCode, Reply};

pub type Result<T> = std::result::Result<T, Rejection>;
/// An Rejection Class for new clients trying to use currently online ID's
#[derive(Debug)]
struct IDAlreadyTaken;
impl warp::reject::Reject for IDAlreadyTaken {}

/// Will handle a Client attempting to connect a websocket with the server
/// A User Requesting to be connected to an already connected ID will be rejected
pub async fn ws_handler(
    ws: warp::ws::Ws,
    id: String,
    clients: data_types::SafeClients,
    sessions: data_types::SafeSessions,
    game_states: data_types::SafeGameStates,
    game_dict: data_types::SafeGameDictionary,
) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(_) => {
            println!("[event] duplicate connection request for id: {}", id);
            Err(warp::reject::custom(IDAlreadyTaken))
        }
        None => Ok(ws.on_upgrade(move |socket| {
            ws::client_connection(socket, id, clients, sessions, game_states, game_dict)
        })),
    }
}

/// Health Check Endpoint used to verify the service is live
pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
