use crate::{
  data_types::{CardColor, CardData, CardDictionary, Client, Session},
  shared_types::{
    CardCode, CharacterCode, ClientEvent, ClientEventCode, EffectCode, GameState, PlayerInfo,
    ServerEvent, ServerEventCode, ServerEventData,
  },
  SafeCardDictionary, SafeClients, SafeSessions,
};
use nanoid::nanoid;
use nanorand::{WyRand, RNG};
use serde_json::from_str;
use std::collections::HashMap;
use warp::ws::Message;

// Helper constructors for different kinds of ServerEvents
impl ServerEvent {
  fn from_error(event_code: ServerEventCode, message: &str) -> ServerEvent {
    ServerEvent {
      event_code,
      message: Some(message.to_string()),
      data: None,
    }
  }
  fn from_update(event_code: ServerEventCode, data: ServerEventData) -> ServerEvent {
    ServerEvent {
      event_code,
      data: Some(data),
      message: None,
    }
  }
}

/// Handle the Client events from a given Session
pub async fn handle_event(
  client_id: &str,
  event: &str,
  clients: &SafeClients,
  sessions: &SafeSessions,
  card_dict: &SafeCardDictionary,
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
    ClientEventCode::DataRequest => {
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.read().await.get(&session_id) {
          println!("[event] relaying state data for client: {}", client_id);
          if let Some(client) = clients.read().await.get(client_id) {
            notify_client(
              &ServerEvent::from_update(
                ServerEventCode::DataResponse,
                ServerEventData {
                  session_id: Some(session.id.clone()),
                  client_id: None,
                  session_client_ids: Some(session.get_client_ids_vec()),
                  game_state: session.game_state.clone(),
                },
              ),
              client,
            );
          }
        }
      }
    }
    ClientEventCode::CreateSession => {
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
          &ServerEvent::from_update(
            ServerEventCode::ClientJoined,
            ServerEventData {
              session_id: Some(session.id.clone()),
              client_id: Some(client_id.to_string()),
              session_client_ids: Some(session.get_client_ids_vec()),
              game_state: None,
            },
          ),
          &client,
        );
      }

      println!(
        "[event] created session :: session count: {}",
        sessions.read().await.len()
      );
    }
    ClientEventCode::JoinSession => {
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
              &ServerEvent::from_update(
                ServerEventCode::InvalidSessionID,
                ServerEventData {
                  session_id: Some(session_id.clone()),
                  client_id: Some(client_id.to_string()),
                  session_client_ids: None,
                  game_state: None,
                },
              ),
              &client,
            );
          }
        }
      }
    }
    ClientEventCode::LeaveSession => {
      remove_client_from_current_session(client_id, clients, sessions).await;
    }
    ClientEventCode::StartGame => {
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          match get_player_character_mapping(&session.get_client_ids_vec()) {
            Ok(turn_orders) => {
              session.game_state = Some(GameState {
                turn_index: 0,
                turn_orders,
                player_hands: HashMap::new(),
                player_fields: HashMap::new(),
                effect: None,
              });

              notify_all_clients(
                ServerEvent::from_update(
                  ServerEventCode::GameStarted,
                  ServerEventData {
                    session_id: Some(session_id),
                    client_id: None,
                    session_client_ids: Some(session.get_client_ids_vec()),
                    game_state: session.game_state.clone(),
                  },
                ),
                &session,
                &clients,
              )
              .await;
            }
            Err(msg) => {
              eprintln!("[error] {}", msg);
              notify_all_clients(
                ServerEvent::from_error(ServerEventCode::LogicError, msg),
                &session,
                &clients,
              )
              .await;
            }
          }
        }
      }
    }
    ClientEventCode::EndTurn => {
      if let Some(session_id) = get_client_session_id(client_id, clients).await {
        if let Some(session) = sessions.write().await.get_mut(&session_id) {
          if let Some(game_state) = session.game_state.as_mut() {
            // incriment index and wrap around
            game_state.turn_index = (game_state.turn_index + 1) % game_state.turn_orders.len();

            notify_all_clients(
              ServerEvent::from_update(
                ServerEventCode::TurnStart,
                ServerEventData {
                  session_id: None,
                  client_id: Some(
                    game_state.turn_orders[game_state.turn_index]
                      .client_id
                      .clone(),
                  ),
                  session_client_ids: None,
                  game_state: None,
                },
              ),
              &session,
              clients,
            )
            .await;
          }
        }
      }
    }
    ClientEventCode::PlayCard => {
      // this is some crazy nesting (っ °Д °;)っ
      if let Some(card_code) = client_event.card_code {
        println!("[event] a card is being played!");
        if let Some(session_id) = get_client_session_id(client_id, clients).await {
          if let Some(session) = sessions.read().await.get(&session_id) {
            if let Some(game_state) = &session.game_state {
              if let Some(card_data) = card_dict.get(&card_code) {
                if let Some(card_targets) = client_event.target_ids {
                  notify_all_clients(
                    card_data.apply_effect(client_id, &card_targets, game_state),
                    session,
                    clients,
                  )
                  .await;
                }
              }
            }
          }
        }
      } else {
        eprintln!("[error] no card_id found for PlayCard Event!");
      }
    }
  }
}

fn card_has_targets(card_code: &CardCode) -> bool {
  return false; // TODO
}

/// Send an update to all clients in the session
///
/// Uses a Read lock on clients
async fn notify_all_clients(game_update: ServerEvent, session: &Session, clients: &SafeClients) {
  for (client_id, _) in &session.client_statuses {
    if let Some(client) = clients.read().await.get(client_id) {
      notify_client(&game_update, client);
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
        ServerEvent::from_update(
          ServerEventCode::ClientLeft,
          ServerEventData {
            session_id: None,
            client_id: Some(client_id.to_string()),
            session_client_ids: None,
            game_state: None,
          },
        ),
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
    ServerEvent::from_update(
      ServerEventCode::ClientJoined,
      ServerEventData {
        session_id: Some(session.id.clone()),
        client_id: Some(client_id.to_string()),
        session_client_ids: Some(session.get_client_ids_vec()),
        game_state: None,
      },
    ),
    &session,
    &clients,
  )
  .await;
}

fn set_new_session_owner(session: &mut Session, clients: &SafeClients, client_id: &String) {
  session.owner = client_id.clone();
  // notify_all_clients(
  //   &ServerEvent {
  //     event_code: ServerEventCode::SessionOwnerChange,
  //     session_id: Some(session.id.clone()),
  //     client_id: Some(client_id.clone()),
  //     session_client_ids: None,
  //   },
  //   &session,
  //   &clients,
  // );
}

fn get_player_character_mapping<'a>(client_vec: &Vec<String>) -> Result<Vec<PlayerInfo>, &str> {
  let player_count = client_vec.len();
  if player_count < 4 || player_count > 7 {
    return Err("Cannot Play with less than 4 Players or More than 7!");
  }

  let mut character_vec: Vec<CharacterCode> = Vec::with_capacity(player_count);
  character_vec.extend(
    [
      CharacterCode::Renegade,
      CharacterCode::Outlaw,
      CharacterCode::Outlaw,
    ]
    .iter()
    .cloned(),
  );
  // 4 players: Sheriff, 1 Renegade, 2 Outlaws
  // 5 players: Sheriff, 1 Renegade, 2 Outlaws, 1 Deputy
  // 6 players: Sheriff, 1 Renegade, 3 Outlaws, 1 Deputy
  // 7 players: Sheriff, 1 Renegade, 3 Outlaws, 2 Deputy
  if player_count >= 5 {
    character_vec.push(CharacterCode::Deputy);
  }
  if player_count >= 6 {
    character_vec.push(CharacterCode::Outlaw);
  }
  if player_count >= 7 {
    character_vec.push(CharacterCode::Deputy);
  }
  // random gen for shuffling
  let mut rand = WyRand::new();
  // shuffle the order characters
  rand.shuffle(&mut character_vec);
  // set sheriff as first turn
  character_vec.insert(0, CharacterCode::Sheriff);
  // create client_vec copy that we will shuffle
  let mut playerinfo_vec: Vec<String> = client_vec.clone();
  rand.shuffle(&mut playerinfo_vec);
  // map the random client_vec to the
  return Ok(
    playerinfo_vec
      .iter()
      .enumerate()
      .map(|(i, id)| PlayerInfo {
        client_id: id.clone(),
        character_code: character_vec[i].clone(),
      })
      .collect::<Vec<PlayerInfo>>(),
  );
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

/// A dictionary of cards to their Color and Function
///
/// Should always remain read-only in concept,
/// so a lock is not needed.
pub fn get_card_dictionary() -> CardDictionary {
  let mut card_dict = HashMap::new();

  card_dict.insert(
    CardCode::Bang,
    CardData {
      color: CardColor::Brown,
      effect: |user_id, targets, game_state| {
        let new_game_state = game_state.clone();
        if targets.len() != 1 {
          println!("Wrong number of Targets");
          return ServerEvent::from_error(ServerEventCode::LogicError, "");
        }
        // card logic that would transform the game_state
        // ...
        println!("Bang!");
        return ServerEvent::from_error(
          ServerEventCode::InvalidSessionID, /* this needs to be updated */
          "",
        );
      },
    },
  );

  return card_dict;
}
