use crate::{
  types::{
    CardID, CharacterCodes, ClientEvent, ClientEventCodes, EffectCodes, GameState, PlayerInfo,
    ServerEvent, ServerEventCodes, ServerEventData,
  },
  Client, SafeClients, SafeSessions, Session,
};
use nanoid::nanoid;
use nanorand::{WyRand, RNG};
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
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.read().await.get(&session_id) {
          println!("[event] relaying state data for client: {}", client_id);
          if let Some(client) = clients.read().await.get(client_id) {
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::DataResponse,
                data: Some(ServerEventData {
                  session_id: Some(session.id.clone()),
                  client_id: None,
                  session_client_ids: Some(session.get_client_ids_vec()),
                  game_state: session.game_state.clone(),
                }),
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

      if let Some(client) = clients.write().await.get_mut(client_id) {
        client.session_id = Some(session.id.clone());
      }

      if let Some(client) = clients.read().await.get(client_id) {
        notify_client(
          &ServerEvent {
            event_code: ServerEventCodes::ClientJoined,
            data: Some(ServerEventData {
              session_id: Some(session.id.clone()),
              client_id: Some(client_id.to_string()),
              session_client_ids: Some(session.get_client_ids_vec()),
              game_state: None,
            }),
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
          if session.game_state.is_some() {
            // do not allow clients to join an active game
            //=============================
            // TODO
            // temoporily allow joining
            //=============================
            // return;
          }
          insert_client_into_given_session(client_id, &clients, session).await;
        } else {
          eprintln!(
            "[warning] could not get session_id for client: {}",
            client_id
          );

          if let Some(client) = clients.read().await.get(client_id) {
            notify_client(
              &ServerEvent {
                event_code: ServerEventCodes::InvalidSessionID,
                data: Some(ServerEventData {
                  session_id: Some(session_id.clone()),
                  client_id: Some(client_id.to_string()),
                  session_client_ids: None,
                  game_state: None,
                }),
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
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          match get_player_character_mapping(&session.get_client_ids_vec()) {
            Ok(turn_orders) => {
              session.game_state = Some(GameState {
                turn_index: 0,
                turn_orders,
                player_blue_cards: HashMap::new(),
                player_green_cards: HashMap::new(),
                effect: None,
              });

              notify_all_clients(
                &ServerEvent {
                  event_code: ServerEventCodes::GameStarted,
                  data: Some(ServerEventData {
                    session_id: Some(session_id),
                    client_id: None,
                    session_client_ids: Some(session.get_client_ids_vec()),
                    game_state: session.game_state.clone(),
                  }),
                },
                &session,
                &clients,
              )
              .await;
            }
            Err(msg) => {
              eprintln!("[error] {}", msg);
              notify_all_clients(
                &ServerEvent {
                  event_code: ServerEventCodes::GameStarted,
                  data: None,
                },
                &session,
                &clients,
              )
              .await;
            }
          }
        }
      }
    }
    ClientEventCodes::EndTurn => {
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          if let Some(game_state) = session.game_state.as_mut() {
            // incriment index and wrap around
            game_state.turn_index = (game_state.turn_index + 1) % game_state.turn_orders.len();

            notify_all_clients(
              &ServerEvent {
                event_code: ServerEventCodes::TurnStart,
                data: Some(ServerEventData {
                  session_id: None,
                  client_id: Some(
                    game_state.turn_orders[game_state.turn_index]
                      .client_id
                      .clone(),
                  ),
                  session_client_ids: None,
                  game_state: None,
                }),
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
///
/// Uses a Read lock on clients
async fn notify_all_clients(game_update: &ServerEvent, session: &Session, clients: &SafeClients) {
  for (client_id, _) in &session.client_statuses {
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
      eprintln!(
        "[error] failed to send message to {} with err: {}",
        client.id, e,
      );
    }
  }
}

/// Removes a client from the session that they currently exist under
async fn remove_client_from_current_session(
  client_id: &str,
  clients: &SafeClients,
  sessions: &SafeSessions,
) {
  if let Some(session_id) = get_client_session_id(client_id, clients).await {
    let mut session_empty: bool = false;
    if let Some(session) = sessions.write().await.get_mut(&session_id) {
      // notify all clients in the sessions that the client will be leaving
      notify_all_clients(
        &ServerEvent {
          event_code: ServerEventCodes::ClientLeft,
          data: Some(ServerEventData {
            session_id: None,
            client_id: Some(client_id.to_string()),
            session_client_ids: None,
            game_state: None,
          }),
        },
        &session,
        &clients,
      )
      .await;
      // remove the client from the session
      session.remove_client(&client_id.to_string());
      // revoke the client's copy of the session_id
      if let Some(client) = clients.write().await.get_mut(client_id) {
        client.session_id = None;
      }
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
        set_new_session_owner(session, &clients, &session.get_client_ids_vec()[0]);
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
///
/// Uses a Read lock for Clients
async fn insert_client_into_given_session(
  client_id: &str,
  clients: &SafeClients,
  session: &mut Session,
) {
  // add client to session
  session.insert_client(client_id, true);
  // update session_id of client
  if let Some(client) = clients.write().await.get_mut(client_id) {
    client.session_id = Some(session.id.clone());
  }
  // notify all clients in the session that the client has joined
  notify_all_clients(
    &ServerEvent {
      event_code: ServerEventCodes::ClientJoined,
      data: Some(ServerEventData {
        session_id: Some(session.id.clone()),
        client_id: Some(client_id.to_string()),
        session_client_ids: Some(session.get_client_ids_vec()),
        game_state: None,
      }),
    },
    &session,
    &clients,
  )
  .await;
}

fn set_new_session_owner(session: &mut Session, clients: &SafeClients, client_id: &String) {
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

fn get_player_character_mapping(client_vec: &Vec<String>) -> Result<Vec<PlayerInfo>, &str> {
  let player_count = client_vec.len();
  if player_count < 4 {
    return Err("Cannot Play with less than 4 Players!");
  }

  let mut list: Vec<CharacterCodes> = Vec::with_capacity(player_count);
  list.extend(
    [
      CharacterCodes::Renegade,
      CharacterCodes::Outlaw,
      CharacterCodes::Outlaw,
    ]
    .iter()
    .cloned(),
  );
  // 4 players: Sheriff, 1 Renegade, 2 Outlaws
  // 5 players: Sheriff, 1 Renegade, 2 Outlaws, 1 Deputy
  // 6 players: Sheriff, 1 Renegade, 3 Outlaws, 1 Deputy
  // 7 players: Sheriff, 1 Renegade, 3 Outlaws, 2 Deputy
  if player_count >= 5 {
    list.push(CharacterCodes::Deputy);
  }
  if player_count >= 6 {
    list.push(CharacterCodes::Outlaw);
  }
  if player_count >= 7 {
    list.push(CharacterCodes::Deputy);
  }
  // shuffle
  WyRand::new().shuffle(&mut list);
  // set sheriff as first turn
  list.insert(0, CharacterCodes::Sheriff);

  let mut playerinfo_vec: Vec<PlayerInfo> = client_vec
    .iter()
    .map(|id| PlayerInfo {
      client_id: id.clone(),
      character_code: CharacterCodes::Sheriff,
    })
    .collect();
  // assign the corresponding positions to their characters
  for (i, character) in list.iter().clone().enumerate() {
    playerinfo_vec[i].character_code = character.clone();
  }

  return Ok(playerinfo_vec);
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

/// pull the session id off of a client
async fn get_client_session_id(client_id: &str, clients: &SafeClients) -> Option<String> {
  if let Some(client) = &clients.read().await.get(client_id) {
    client.session_id.clone()
  } else {
    None
  }
}
