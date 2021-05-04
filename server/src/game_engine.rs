use crate::{
  event_types::{CardID, ClientEvent, ClientEventCodes, ServerEvent, ServerEventCodes},
  Client, Clients, EffectCodes, GameState, PlayerInfo, Session, Sessions,
};
use nanoid::nanoid;
use serde_json::from_str;
use std::collections::{HashMap, HashSet};
use warp::ws::Message;

/// Handle the Client events from a given Session
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
    ClientEventCodes::DataRequest => {
      println!("DATA REQUEST");
      if let Some(session_id) = get_client_session_id(id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          println!("DATA SESSIONS");
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
      let mut new_client_ids = HashSet::new();
      new_client_ids.insert(String::from(id));
      let session = Session {
        client_ids: new_client_ids,
        session_id: get_rand_session_id(),
        game_state: None,
      };

      if let Some(client) = clients.read().await.get(id) {
        notify_client(
          &ServerEvent {
            event_code: ServerEventCodes::ClientJoined,
            session_id: Some(session.session_id.clone()),
            client_id: Some(String::from(id)),
            session_client_ids: Some(session.client_ids.clone().into_iter().collect()),
          },
          client,
        );
      }

      sessions
        .write()
        .await
        .insert(session.session_id.clone(), session);

      println!(
        "created session.\n\tsession count: {}",
        sessions.read().await.len()
      );
    }
    ClientEventCodes::JoinSession => {
      // identify session_id to join
      if let Some(session_id) = client_event.session_id {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // add the client to the session
          session.client_ids.insert(String::from(id));
          // notify all clients in the session that the client has joined
          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::ClientJoined,
              session_id: Some(session_id),
              client_id: Some(String::from(id)),
              session_client_ids: Some(session.client_ids.clone().into_iter().collect()),
            },
            &session,
            clients,
          )
          .await;
        } else {
          eprintln!("could not get session_id for id: {}", id);
        }
      }
    }
    ClientEventCodes::LeaveSession => {
      if let Some(session_id) = get_client_session_id(id, sessions).await {
        let mut session_empty: bool = false;
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // notify all clients in the sessions that the client will be leacing
          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::ClientLeft,
              session_id: None,
              client_id: Some(String::from(id)),
              session_client_ids: None,
            },
            &session,
            clients,
          )
          .await;
          // remove the client from the session
          session.client_ids.remove(&String::from(id));
          println!(
            "clients in this session after {} left: {}",
            id,
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
    ClientEventCodes::StartGame => {
      if let Some(session_id) = get_client_session_id(id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          // assign roles to players
          let turn_orders: Vec<PlayerInfo> = session
            .client_ids
            .clone()
            .into_iter()
            .map(|client_id| PlayerInfo {
              client_id: client_id,
              character: String::from("Sheriff"),
            })
            .collect();

          session.game_state = Some(GameState {
            turn_index: 0,
            turn_orders,
            player_blue_cards: HashMap::new(),
            player_green_cards: HashMap::new(),
            effect: EffectCodes::None,
          });

          notify_all_clients(
            &ServerEvent {
              event_code: ServerEventCodes::GameStarted,
              session_id: None,
              client_id: None,
              session_client_ids: None,
            },
            &session,
            clients,
          )
          .await;
        }
      }
    }
    ClientEventCodes::EndTurn => {
      if let Some(session_id) = get_client_session_id(id, sessions).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          if let Some(game_state) = session.game_state.as_mut() {
            game_state.turn_index += 1;

            notify_all_clients(
              &ServerEvent {
                event_code: ServerEventCodes::TurnStart,
                session_id: None,
                client_id: Some(
                  game_state.turn_orders[game_state.turn_index]
                    .client_id
                    .clone(),
                ),
                session_client_ids: None,
              },
              &session,
              clients,
            )
            .await;
          }
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
  return true; // TODO
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

/// Gets a random new session ID that is 5 characters long
/// This should almost ensure session uniqueness when dealing with a sizeable number of sessions
fn get_rand_session_id() -> String {
  let alphabet: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
  ];
  nanoid!(5, &alphabet)
}
