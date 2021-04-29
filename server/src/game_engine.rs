use crate::{Client, Clients, Session};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_repr::Deserialize_repr;
use warp::ws::Message;

#[derive(Serialize)]
struct GameUpdate {
  event: u8,
  message: String,
}

type CardID = u8;

#[derive(Deserialize)]
struct SessionEvent {
  event_code: SessionEventCode,
  target_ids: Option<Vec<String>>,
  card_id: Option<CardID>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
enum SessionEventCode {
  JoinRoom = 1,
  LeaveRoom = 2,
  CardPlay = 3,
}

/// Handle the events from a given sessions
pub async fn handle_event(id: &str, event: &str, clients: &Clients, session: &Session) {
  println!("> begin handling room events");
  //======================================================
  // Deserialize into Sesseion Event object
  //======================================================
  let session_event: SessionEvent = match from_str::<SessionEvent>(event) {
    Ok(obj) => obj,
    Err(e) => {
      println!("Error parsing SessionEvent struct from string: {}", event);
      return;
    }
  };
  //======================================================
  // Match Session Event Codes to execution
  //======================================================
  match session_event.event_code {
    SessionEventCode::JoinRoom => {
      notify_all_clients(
        &GameUpdate {
          event: 2,
          message: format!("{} is joining the room", id),
        },
        session,
        clients,
      )
      .await;
    }
    SessionEventCode::LeaveRoom => {
      notify_all_clients(
        &GameUpdate {
          event: 2,
          message: format!("{} is leaving the room", id),
        },
        session,
        clients,
      )
      .await;
    }
    SessionEventCode::CardPlay => {
      if let Some(card_id) = session_event.card_id {
        if card_has_targets(card_id) {
          if let Some(target_ids) = session_event.target_ids {
            // TODO
          } else {
            eprintln!("No targets given for a card that has targets!");
          }
        }
      } else {
        eprintln!("No card_id found for CardPlay Event!");
      }
    }
  }
}

fn card_has_targets(card_id: CardID) -> bool {
  // TODO
  return true;
}

/// Send an update to all clients in the session
async fn notify_all_clients(game_update: &GameUpdate, session: &Session, clients: &Clients) {
  for client_id in &session.client_ids {
    if let Some(client) = clients.read().await.get(client_id) {
      notify_client(game_update, client);
    }
  }
}

/// Send an update to single clients
fn notify_client(game_update: &GameUpdate, client: &Client) {
  if let Some(sender) = &client.sender {
    if let Err(e) = sender.send(Ok(Message::text(&game_update.message))) {
      println!("Error:{} sending message to {}", e, client.user_id);
    }
  }
}
