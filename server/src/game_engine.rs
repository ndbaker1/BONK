use crate::{
  event_types::{CardID, ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes},
  Client, Clients, EffectCodes, GameState, PlayerInfo, SafeClients, SafeSessions, Session,
  Sessions,
};
use nanoid::nanoid;
use serde_json::from_str;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use warp::ws::Message;

/// Handle the Client events from a given Session
pub async fn handle_event(id: &str, event: &str, clients: &SafeClients, sessions: &SafeSessions) {
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
    ClientEventCodes::DataRequest => {
      let r_session = sessions.read().await;
      let session_id_option = get_client_session_id(id, &r_session);
      drop(r_session);
      if let Some(session_id) = session_id_option {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          if let Some(client) = clients.read().await.get(id) {
            println!("relaying state data for client: {}", id);
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::DataResponse,
                session_id: Some(session.session_id.clone()),
                client_id: None,
                session_client_ids: Some(session.client_ids.clone().into_iter().collect()),
              },
              client,
            );
          }
        }
      }
    }
    ClientEventCodes::CreateSession => {
      let session = &mut Session {
        client_ids: HashSet::from_iter([id.to_string()].iter().cloned()),
        session_id: get_rand_session_id(),
        game_state: None,
      };

      insert_client_into_given_session(id, clients, session).await;

      sessions
        .write()
        .await
        .insert(session.session_id.clone(), session.clone());

      println!(
        "created session.\n\tsession count: {}",
        sessions.read().await.len()
      );
    }
    ClientEventCodes::JoinSession => {
      // identify session_id to join
      if let Some(session_id) = client_event.session_id {
        remove_client_from_current_session(id, clients, sessions).await;
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          insert_client_into_given_session(id, clients, session).await;
        } else {
          eprintln!("could not get session_id for client: {}", id);
          if let Some(client) = clients.read().await.get(id) {
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::InvalidSessionID,
                session_id: Some(session_id),
                client_id: Some(id.to_string()),
                session_client_ids: None,
              },
              &client,
            );
          }
        }
      }
    }
    ClientEventCodes::LeaveSession => {
      remove_client_from_current_session(id, clients, sessions).await;
    }
    ClientEventCodes::StartGame => {
      let r_session = sessions.read().await;
      let session_id_option = get_client_session_id(id, &r_session);
      drop(r_session);
      if let Some(session_id) = session_id_option {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // assign roles to players
          let turn_orders: Vec<PlayerInfo> = session
            .client_ids
            .clone()
            .into_iter()
            .map(|client_id| PlayerInfo {
              client_id: client_id,
              character: "Sheriff".to_string(),
            })
            .collect();

          session.game_state = Some(GameState {
            turn_index: 0,
            turn_orders,
            player_blue_cards: HashMap::new(),
            player_green_cards: HashMap::new(),
            effect: EffectCodes::None,
          });

          let r_clients = clients.read().await;
          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::GameStarted,
              session_id: None,
              client_id: None,
              session_client_ids: None,
            },
            &session,
            &r_clients,
          );
          drop(r_clients);
        }
      }
    }
    ClientEventCodes::EndTurn => {
      // if let r_session = sessions.read().await {
      //   if let Some(session_id) = get_client_session_id(id, &r_session) {
      //     if let Some(session) = sessions.write().await.get_mut(&session_id) {
      //       if let Some(game_state) = session.game_state.as_mut() {
      //         game_state.turn_index += 1;
      //         notify_all_clients(
      //           &ServerEvent {
      //             event_code: ServerEventCodes::TurnStart,
      //             session_id: None,
      //             client_id: Some(
      //               game_state.turn_orders[game_state.turn_index]
      //                 .client_id
      //                 .clone(),
      //             ),
      //             session_client_ids: None,
      //           },
      //           &session,
      //           clients,
      //         )
      //         .await;
      //       }
      //     }
      //   }
      // }
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
  return true; // TODO
}

/// Send an update to all clients in the session
fn notify_all_clients(game_update: &ServerEvent, session: &Session, clients: &Clients) {
  for client_id in &session.client_ids {
    if let Some(client) = clients.get(client_id) {
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

fn get_client_session_id(id: &str, sessions: &Sessions) -> Option<String> {
  for session in sessions.values() {
    if session.client_ids.contains(id) {
      return Some(session.session_id.clone());
    }
  }
  None
}

/// Removes a client from the session that they currently exist under
async fn remove_client_from_current_session(
  client_id: &str,
  clients: &SafeClients,
  sessions: &SafeSessions,
) {
  let r_session = sessions.read().await;
  let session_id_option = get_client_session_id(client_id, &r_session);
  drop(r_session);
  if let Some(session_id) = session_id_option {
    let mut session_empty: bool = false;
    if let Some(session) = sessions.write().await.get_mut(&session_id) {
      // notify all clients in the sessions that the client will be leacing
      let r_clients = clients.read().await;
      notify_all_clients(
        &ServerEvent {
          event_code: ServerEventCodes::ClientLeft,
          session_id: None,
          client_id: Some(client_id.to_string()),
          session_client_ids: None,
        },
        &session,
        &r_clients,
      );
      drop(r_clients);
      // remove the client from the session
      session.client_ids.remove(&client_id.to_string());
      println!(
        "clients in this session after {} left: {}",
        client_id,
        session.client_ids.len()
      );
      session_empty = session.client_ids.is_empty();
    }
    // clean up the session from the map if it is empty
    // * we cannot do this in the scope above because because we are already holding a mutable reference to a session within the map
    if session_empty {
      sessions.write().await.remove(&session_id);
      println!(
        "removed empty session.\n\tremaining session count: {}",
        sessions.read().await.len()
      );
    }
  }
}

/// Takes a mutable session reference in order to add a client to a given session
async fn insert_client_into_given_session(
  client_id: &str,
  clients: &SafeClients,
  session: &mut Session,
) {
  session.client_ids.insert(client_id.to_string());
  // notify all clients in the session that the client has joined
  let r_clients = clients.read().await;
  notify_all_clients(
    &ServerEvent {
      event_code: ServerEventCodes::ClientJoined,
      session_id: Some(session.session_id.clone()),
      client_id: Some(client_id.to_string()),
      session_client_ids: Some(session.client_ids.clone().into_iter().collect()),
    },
    &session,
    &r_clients,
  );
}

/// Gets a random new session ID that is 5 characters long
/// This should almost ensure session uniqueness when dealing with a sizeable number of sessions
fn get_rand_session_id() -> String {
  let alphabet: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];
  nanoid!(5, &alphabet)
}
