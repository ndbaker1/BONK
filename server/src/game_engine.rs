use crate::{
  game_events::{CardID, ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes},
  Client, Clients, Session, Sessions,
};
use nanoid::nanoid;
use serde_json::from_str;
use std::collections::HashSet;
use warp::ws::Message;

/// Handle the events from a given sessions
pub async fn handle_event(id: &str, event: &str, clients: &Clients, sessions: &Sessions) {
  //======================================================
  // Deserialize into Session Event object
  //======================================================
  let client_event: ClientEvent = match from_str::<ClientEvent>(event) {
    Ok(obj) => obj,
    Err(_) => {
      println!("Error parsing ClientEvent struct from string: {}", event);
      return;
    }
  };

  match client_event.event_code {
    ClientEventCodes::CreateSession => {
      let mut new_client_ids = HashSet::new();
      new_client_ids.insert(String::from(id));
      let session = Session {
        client_ids: new_client_ids,
        session_id: get_rand_session_id(),
      };

      if let Some(client) = clients.read().await.get(id) {
        notify_client(
          &ServerEvent {
            event_code: ServerEventCodes::SessionCreated,
            session_id: Some(String::from(session.session_id.as_str())),
            client_id: None,
          },
          client,
        );
      }

      sessions
        .write()
        .await
        .insert(String::from(session.session_id.as_str()), session);
    }
    ClientEventCodes::JoinSession => {
      // identify session_id to join
      if let Some(session_id) = client_event.session_id {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // notify all clients in the session that the client will be joining
          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::ClientJoined,
              session_id: None,
              client_id: Some(String::from(id)),
            },
            &session,
            clients,
          )
          .await;
          // add the client to the session
          session.client_ids.insert(String::from(id));
        } else {
          eprintln!("could not get session_id for ")
        }
      }
    }
    ClientEventCodes::LeaveSession => {
      if let Some(session_id) = get_client_session_id(id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // notify all clients in the sessions that the client will be leacing
          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::ClientLeft,
              session_id: None,
              client_id: Some(String::from(id)),
            },
            &session,
            clients,
          )
          .await;
          // remove the client from the session
          session.client_ids.remove(&String::from(id));
        }
      }
    }
    ClientEventCodes::PlayCard => {
      if let Some(card_id) = client_event.card_id {
        if card_has_targets(card_id) {
          if let None = client_event.target_ids {
            eprintln!("No targets given for a card that has targets!");
            return;
          }
        }
        println!("A Card is being played!");
      } else {
        eprintln!("No card_id found for PlayCard Event!");
      }
    }
  }
}

fn card_has_targets(card_id: CardID) -> bool {
  // TODO
  return true;
}

/// Send an update to all clients in the session
async fn notify_all_clients(game_update: &ServerEvent, session: &Session, clients: &Clients) {
  for client_id in &session.client_ids {
    if let Some(client) = clients.read().await.get(client_id) {
      notify_client(game_update, client);
    }
  }
}

/// Send an update to single clients
fn notify_client(game_update: &ServerEvent, client: &Client) {
  if let Some(sender) = &client.sender {
    if let Err(e) = sender.send(Ok(Message::text(
      serde_json::to_string(game_update).unwrap(),
    ))) {
      println!("Error:{} sending message to {}", e, client.user_id);
    }
  }
}

async fn get_client_session_id(id: &str, sessions: &Sessions) -> Option<String> {
  for session in sessions.read().await.values() {
    if session.client_ids.contains(id) {
      return Some(session.session_id.clone());
    }
  }
  None
}

fn get_rand_session_id() -> String {
  let alphabet: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
  ];
  nanoid!(5, &alphabet)
}
