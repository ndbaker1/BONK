use crate::{ws, Result, SafeCardDictionary, SafeClients, SafeSessions};
use warp::{http::StatusCode, Reply};

/// An Rejection Class for new clients trying to use currently online ID's
#[derive(Debug)]
struct IDAlreadyTaken;
impl warp::reject::Reject for IDAlreadyTaken {}

/// Will handle a Client attempting to connect a websocket with the server
/// A User Requesting to be connected to an already connected ID will be rejected
pub async fn ws_handler(
    ws: warp::ws::Ws,
    id: String,
    clients: SafeClients,
    sessions: SafeSessions,
    card_dict: SafeCardDictionary,
) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(_) => {
            println!("[event] duplicate connection request for id: {}", id);
            Err(warp::reject::custom(IDAlreadyTaken))
        }
        None => Ok(ws.on_upgrade(move |socket| {
            ws::client_connection(socket, id, clients, sessions, card_dict)
        })),
    }
}

/// Health Check Endpoint used to verify the service is live
pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
