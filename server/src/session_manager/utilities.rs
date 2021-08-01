use super::types::{Client, SafeSessions};
use crate::game_engine::types::SafeGameStates;

/// If a client exists in a session, then set their status to inactive.
/// If setting inactive status would leave no other active member, remove the session
pub async fn disconnect_client(
    client: &Client,
    sessions: &SafeSessions,
    game_states: &SafeGameStates,
) {
    println!("[event] {} disconnected", client.id);
    if let Some(session_id) = &client.session_id {
        let mut session_empty = false;
        // remove the client from the session and check if the session become empty
        if let Some(session) = sessions.write().await.get_mut(session_id) {
            session.set_client_active_status(&client.id, false);
            session_empty = session.get_clients_with_active_status(true).is_empty();
        }
        // remove the session if empty
        if session_empty {
            sessions.write().await.remove(session_id);
            println!(
                "[event] removed empty session :: remaining session count: {}",
                sessions.read().await.len()
            );
        }
    }
}

/// If a client exists in a session, then set their status to active
pub async fn handle_client_connect(client: &Client, sessions: &SafeSessions) {
    println!("[event] {} connected", client.id);
    if let Some(session_id) = &client.session_id {
        if let Some(session) = sessions.write().await.get_mut(session_id) {
            session.set_client_active_status(&client.id, true);
        }
    }
}

/// Gets the SessionID of a client if it exists
pub async fn get_client_session_id(client_id: &str, sessions: &SafeSessions) -> Option<String> {
    for session in sessions.read().await.values() {
        if session.contains_client(client_id) {
            return Some(session.id.clone());
        }
    }
    return None;
}

/// Remove a sessions and the possible game state that accompanies it
pub async fn cleanup_session(
    session_id: &str,
    sessions: &SafeSessions,
    game_states: &SafeGameStates,
) {
    // remove session
    // remove possible game state
    game_states.write().await.remove(session_id);
    // log
    println!(
        "[event] removed empty session :: remaining session count: {}",
        sessions.read().await.len()
    );
}
