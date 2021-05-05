use crate::{
  event_types::{CardID, ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes},
  Client, Clients, EffectCodes, GameState, PlayerInfo, SafeClients, SafeSessions, Session,
};
use nanoid::nanoid;
use serde_json::from_str;
use std::collections::HashMap;
use warp::ws::Message;

/// Handle the Client events from a given Session
pub async fn handle_event(
  client_id: &str,
  event: &str,
  clients: &SafeClients,
  sessions: &SafeSessions,
) {
  //======================================================
  // Deserialize into Session Event object
  //======================================================
  let client_event: ClientEvent = match from_str::<ClientEvent>(event) {
    Ok(obj) => obj,
    Err(_) => {
      eprintln!(
        "[error] failed to parse ClientEvent struct from string: {}",
        event
      );
      return;
    }
  };

  match client_event.event_code {
    ClientEventCodes::DataRequest => {
      if let Some(session_id) = get_client_session_id(client_id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          if let Some(client) = clients.read().await.get(client_id) {
            println!("[event] relaying state data for client: {}", client_id);
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::DataResponse,
                session_id: Some(session.id.clone()),
                client_id: None,
                session_client_ids: Some(session.get_client_ids_vec()),
              },
              client,
            );
          }
        }
      }
    }
    ClientEventCodes::CreateSession => {
      let session = &mut Session {
        client_statuses: HashMap::new(),
        owner: client_id.to_string(),
        id: get_rand_session_id(),
        game_state: None,
      };
      session.insert_client(&client_id.to_string(), true);

      sessions
        .write()
        .await
        .insert(session.id.clone(), session.clone());

      if let Some(client) = clients.read().await.get(client_id) {
        notify_client(
          &ServerEvent {
            event_code: ServerEventCodes::ClientJoined,
            session_id: Some(session.id.clone()),
            client_id: Some(client_id.to_string()),
            session_client_ids: Some(session.get_client_ids_vec()),
          },
          &client,
        );
      }

      println!(
        "[event] created session :: session count: {}",
        sessions.read().await.len()
      );
    }
    ClientEventCodes::JoinSession => {
      // identify session_id to join
      if let Some(session_id) = client_event.session_id {
        remove_client_from_current_session(client_id, clients, sessions).await;
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          let r_clients = clients.read().await;
          insert_client_into_given_session(client_id, &r_clients, session);
          drop(r_clients);
        } else {
          eprintln!(
            "[warning] could not get session_id for client: {}",
            client_id
          );
          if let Some(client) = clients.read().await.get(client_id) {
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::InvalidSessionID,
                session_id: Some(session_id),
                client_id: Some(client_id.to_string()),
                session_client_ids: None,
              },
              &client,
            );
          }
        }
      }
    }
    ClientEventCodes::LeaveSession => {
      remove_client_from_current_session(client_id, clients, sessions).await;
    }
    ClientEventCodes::StartGame => {
      if let Some(session_id) = get_client_session_id(client_id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // assign roles to players
          let turn_orders: Vec<PlayerInfo> = session
            .get_client_ids_vec()
            .iter()
            .map(|client_id| PlayerInfo {
              client_id: client_id.clone(),
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

          {
            // create scope for read lock
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
          }
        }
      }
    }
    ClientEventCodes::EndTurn => {
      // if let r_session = sessions.read().await {
      //   if let Some(session_id) = get_client_session_id(client_id, &r_session) {
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
  for (client_id, _) in &session.client_statuses {
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
      eprintln!(
        "[error] failed to send message to {} with err: {}",
        client.id, e,
      );
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

/// Removes a client from the session that they currently exist under
async fn remove_client_from_current_session(
  client_id: &str,
  clients: &SafeClients,
  sessions: &SafeSessions,
) {
  if let Some(session_id) = get_client_session_id(client_id, sessions).await {
    let mut session_empty: bool = false;
    if let Some(session) = sessions.write().await.get_mut(&session_id) {
      // notify all clients in the sessions that the client will be leaving
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
      session.remove_client(&client_id.to_string());
      println!(
        "[event] clients remainings in session: {} after {} left: {}",
        session_id,
        client_id,
        session.get_client_count()
      );
      // checks the statuses to see if any users are still active
      session_empty = session.get_clients_with_active_status(true).is_empty();
      // if the session is not empty, make someone else the owner
      if !session_empty {
        let r_clients = clients.read().await;
        set_new_session_owner(session, &r_clients, &session.get_client_ids_vec()[0]);
        drop(r_clients);
      }
    }
    // clean up the session from the map if it is empty
    // * we cannot do this in the scope above because because we are already holding a mutable reference to a session within the map
    if session_empty {
      sessions.write().await.remove(&session_id);
      println!(
        "[event] removed empty session :: remaining session count: {}",
        sessions.read().await.len()
      );
    }
  }
}

/// Takes a mutable session reference in order to add a client to a given session
fn insert_client_into_given_session(client_id: &str, clients: &Clients, session: &mut Session) {
  session.insert_client(client_id, true);
  // notify all clients in the session that the client has joined
  notify_all_clients(
    &ServerEvent {
      event_code: ServerEventCodes::ClientJoined,
      session_id: Some(session.id.clone()),
      client_id: Some(client_id.to_string()),
      session_client_ids: Some(session.get_client_ids_vec()),
    },
    &session,
    &clients,
  );
}

fn set_new_session_owner(session: &mut Session, clients: &Clients, client_id: &String) {
  session.owner = client_id.clone();
  // notify_all_clients(
  //   &ServerEvent {
  //     event_code: ServerEventCodes::SessionOwnerChange,
  //     session_id: Some(session.id.clone()),
  //     client_id: Some(client_id.clone()),
  //     session_client_ids: None,
  //   },
  //   &session,
  //   &clients,
  // );
}

/// Gets a random new session 1 that is 5 characters long
/// This should almost ensure session uniqueness when dealing with a sizeable number of sessions
fn get_rand_session_id() -> String {
  let alphabet: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];
  nanoid!(5, &alphabet)
}
