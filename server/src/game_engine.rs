use crate::{
    data_types, session_types,
    shared_types::{self, Card, ServerEvent, ServerEventCode, ServerEventData},
    ws::cleanup_session,
};
use nanoid::nanoid;
use nanorand::{WyRand, RNG};
use serde_json::from_str;
use std::collections::HashMap;
use warp::ws::Message;

pub mod cards;
pub mod characters;
pub mod deck;
pub mod types;

impl shared_types::PlayerData {
    pub fn card_iter(
        &self,
    ) -> std::iter::Chain<std::slice::Iter<'_, Card>, std::slice::Iter<'_, Card>> {
        self.hand.iter().chain(self.field.iter())
    }

    pub fn remove_cards(&mut self, cards: &Vec<Card>) {
        self.hand.retain(|card| !cards.contains(card));
        self.field.retain(|card| !cards.contains(card));
    }

    pub fn add_cards_to_hand(&mut self, cards: &Vec<Card>) {
        self.hand.extend(cards.into_iter().cloned());
    }
}

impl types::GameState {
    pub fn to_game_data(&self) -> shared_types::GameData {
        shared_types::GameData {
            round: self.round,
            turn_index: self.turn_index,
            discard: self.discard.clone(),
            player_order: self.player_order.clone(),
        }
    }
    pub fn is_player_turn(&self, player: &str) -> bool {
        self.player_order[self.turn_index] == player
    }

    pub fn player_owns_cards(&self, player: &str, cards: &Vec<shared_types::Card>) -> bool {
        match self.player_data.get(player) {
            Some(player_data) => cards.iter().all(|played_card| {
                player_data
                    .card_iter()
                    .any(|player_card| player_card == played_card)
            }),
            None => false,
        }
    }
}

/// Handle the Client events from a given Session
pub async fn handle_event(
    client_id: &str,
    event: &str,
    clients: &data_types::SafeClients,
    sessions: &data_types::SafeSessions,
    game_states: &data_types::SafeGameStates,
) {
    //======================================================
    // Deserialize into Session Event object
    //======================================================
    let client_event: shared_types::ClientEvent = match from_str::<shared_types::ClientEvent>(event)
    {
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
        shared_types::ClientEventCode::DataRequest => {
            let session_id = match get_client_session_id(client_id, clients).await {
                Some(s_id) => s_id,
                None => return, // no session is ok
            };

            let mut server_event: shared_types::ServerEvent =
                shared_types::ServerEvent::builder(shared_types::ServerEventCode::DataResponse)
                    .data(ServerEventData::builder().session_id(&session_id).build())
                    .build();

            if let Some(client) = clients.read().await.get(client_id) {
                if let Some(data) = server_event.data.as_mut() {
                    if let Some(session) = sessions.read().await.get(&session_id) {
                        data.session_client_ids = Some(session.get_client_ids());
                    }
                    if let Some(game_state) = game_states.read().await.get(&session_id) {
                        data.game_data = Some(game_state.to_game_data());
                        data.player_data = match game_state.player_data.get(&client.id) {
                            Some(pd) => Some(pd.clone()),
                            None => None,
                        };
                    }
                }
                notify_client(&server_event, client);
            }
        }
        shared_types::ClientEventCode::CreateSession => {
            let session = &mut session_types::Session {
                client_statuses: HashMap::new(),
                owner: client_id.to_string(),
                id: get_rand_session_id(),
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
                    &shared_types::ServerEvent::builder(
                        shared_types::ServerEventCode::ClientJoined,
                    )
                    .data(
                        ServerEventData::builder()
                            .client_id(client_id)
                            .session_id(&session.id)
                            .session_client_ids(&session.get_client_ids())
                            .build(),
                    )
                    .build(),
                    &client,
                );
            }

            println!(
                "[event] created session :: session count: {}",
                sessions.read().await.len()
            );
        }
        shared_types::ClientEventCode::JoinSession => {
            let session_id = match client_event.session_id {
                Some(s_id) => s_id,
                None => return, // no session was found on a session join request? ¯\(°_o)/¯
            };

            remove_client_from_current_session(client_id, clients, sessions, game_states).await;

            if let Some(session) = sessions.write().await.get_mut(&session_id) {
                if let Some(_) = game_states.read().await.get(&session_id) {
                    return; // do not allow clients to join an active game
                }
                insert_client_into_given_session(client_id, &clients, session).await;
            } else {
                if let Some(client) = clients.read().await.get(client_id) {
                    notify_client(
                        &shared_types::ServerEvent::builder(
                            shared_types::ServerEventCode::LogicError,
                        )
                        .message(&format!("Invalid SessionID: {}", session_id))
                        .build(),
                        &client,
                    );
                }
            }
        }
        shared_types::ClientEventCode::LeaveSession => {
            remove_client_from_current_session(client_id, clients, sessions, game_states).await;
        }
        shared_types::ClientEventCode::StartGame => {
            let session_id = match get_client_session_id(client_id, clients).await {
                Some(s_id) => s_id,
                None => return,
            };

            if let Some(session) = sessions.read().await.get(&session_id) {
                match initialize_game_data(&session.get_client_ids()) {
                    Ok((player_order, player_data, deck)) => {
                        let game_state = types::GameState {
                            round: 1, // temp to avoid grace period
                            turn_index: 0,
                            player_order,
                            player_data,
                            deck,
                            discard: Vec::new(),
                            event_state_stack: Vec::new(),
                        };

                        game_states
                            .write()
                            .await
                            .insert(session_id.clone(), game_state.clone());

                        // give each player the initial state of the game
                        for (player, player_data) in game_state.player_data.iter() {
                            if let Some(client) = clients.read().await.get(player) {
                                notify_client(
                                    &shared_types::ServerEvent::builder(
                                        shared_types::ServerEventCode::GameStarted,
                                    )
                                    .data(
                                        ServerEventData::builder()
                                            .session_client_ids(&session.get_client_ids())
                                            .game_data(&game_state.to_game_data())
                                            .player_data(&player_data.clone())
                                            .build(),
                                    )
                                    .build(),
                                    &client,
                                )
                            }
                        }
                        // signal the turn start
                        notify_session(
                            &shared_types::ServerEvent::builder(
                                shared_types::ServerEventCode::TurnStart,
                            )
                            .build(),
                            session,
                            clients,
                        )
                        .await;
                    }
                    Err(msg) => {
                        eprintln!("[error] {}", msg);
                        notify_session(
                            &shared_types::ServerEvent::builder(
                                shared_types::ServerEventCode::LogicError,
                            )
                            .message(msg)
                            .build(),
                            &session,
                            &clients,
                        )
                        .await;
                    }
                }
            }
        }
        shared_types::ClientEventCode::EndTurn => {
            let session_id: String = match get_client_session_id(client_id, clients).await {
                Some(s_id) => s_id,
                None => return,
            };

            if let Some(game_state) = game_states.write().await.get_mut(&session_id) {
                // incriment index and wrap around
                game_state.turn_index = (game_state.turn_index + 1) % game_state.player_order.len();

                if let Some(session) = sessions.read().await.get(&session_id) {
                    notify_session(
                        &shared_types::ServerEvent::builder(
                            shared_types::ServerEventCode::TurnStart,
                        )
                        .data(
                            ServerEventData::builder()
                                .client_id(&game_state.player_order[game_state.turn_index])
                                .build(),
                        )
                        .build(),
                        &session,
                        clients,
                    )
                    .await;
                }
            }
        }
        shared_types::ClientEventCode::PlayerAction => {
            let cards = match client_event.cards {
                Some(c) => c,
                None => Vec::new(), // no card list for a play-card event?
            };

            // later incorporate events that are not only cards
            // the actiontype will tell if the action is a card play or a character ability
            let action_type = client_event.action_type;

            let session_id = match get_client_session_id(client_id, clients).await {
                Some(s_id) => s_id,
                None => return, // this card was not played in an active session?
            };

            if let Some(game_state) = game_states.write().await.get_mut(&session_id) {
                // check the event state stack to find out if the play was an initiation or response
                match game_state.event_state_stack.clone().last() {
                    // this play must be a response to another, or it is invalid
                    Some((card_name, event_players, _)) => {
                        if event_players.contains(&String::from(client_id)) {
                            // based on what event is currenty being processed, decide on the behavior
                            let messages = (cards::get_card_data(card_name).update)(
                                client_id,
                                &cards,
                                &Vec::new(),
                                game_state,
                            );

                            // relay any updates or errors from the cards being played to those in the lobby.
                            for (client_id, message) in messages.iter() {
                                if let Some(client) = clients.read().await.get(client_id) {
                                    notify_client(message, client);
                                }
                            }
                        }
                    }
                    // this player is initiating a sequence, so it must be their turn to continue
                    None => {
                        if game_state.is_player_turn(client_id) {
                            // verify player has cards
                            if game_state.player_owns_cards(client_id, &cards) {
                                let card_data = cards::get_card_data(&cards[0].name);
                                let (precheck, effect): (
                                    &types::CardConditions,
                                    &types::CardEffect,
                                ) = (&card_data.requirements, &card_data.initiate);

                                // default to an empty vector for cards whose effects do not concern targets
                                let targets = match client_event.target_ids {
                                    Some(card_targets) => card_targets,
                                    None => Vec::new(),
                                };

                                //=========================================================
                                // execute the preconditions check
                                // if it passes then execute the effect of the card/cards
                                //=========================================================
                                let messages =
                                    match precheck(client_id, &cards, &targets, game_state) {
                                        Ok(_) => effect(client_id, &cards, &targets, game_state),
                                        Err(e) => {
                                            if let Some(client) =
                                                clients.read().await.get(client_id)
                                            {
                                                notify_client(
                                                    &shared_types::ServerEvent::builder(
                                                        ServerEventCode::LogicError,
                                                    )
                                                    .message(&e)
                                                    .build(),
                                                    client,
                                                );
                                            }
                                            return;
                                        }
                                    };

                                // relay any updates or errors from the cards being played to those in the lobby.
                                for (client_id, message) in messages.iter() {
                                    if let Some(client) = clients.read().await.get(client_id) {
                                        notify_client(message, client);
                                    }
                                }
                            } else if let Some(client) = clients.read().await.get(client_id) {
                                notify_client(
                                    &shared_types::ServerEvent::builder(
                                        shared_types::ServerEventCode::LogicError,
                                    )
                                    .message("Lack the cards to play.")
                                    .build(),
                                    client,
                                );
                            }
                        } else if let Some(client) = clients.read().await.get(client_id) {
                            notify_client(
                                &shared_types::ServerEvent::builder(
                                    shared_types::ServerEventCode::LogicError,
                                )
                                .message("Cannot initiate play when it is not your turn.")
                                .build(),
                                client,
                            );
                        }
                    }
                }
            } else {
                eprintln!("[error] session was not found with id: {}", session_id);
            }
        }
    }
}

/// Send an update to all clients in the session
///
/// Uses a Read lock on clients
async fn notify_session(
    game_update: &shared_types::ServerEvent,
    session: &session_types::Session,
    clients: &data_types::SafeClients,
) {
    for (client_id, _) in &session.client_statuses {
        if let Some(client) = clients.read().await.get(client_id) {
            notify_client(game_update, client);
        }
    }
}

/// Send and update to a set of clients
async fn notify_clients(
    game_update: &shared_types::ServerEvent,
    client_ids: &Vec<String>,
    clients: &data_types::SafeClients,
) {
    for client_id in client_ids {
        if let Some(client) = clients.read().await.get(client_id) {
            notify_client(game_update, client);
        }
    }
}

/// Send an update to single clients
fn notify_client(game_update: &shared_types::ServerEvent, client: &session_types::Client) {
    // println!("[emit] {:?} to {:?}", game_update, client);
    let sender = match &client.sender {
        Some(s) => s,
        None => return eprintln!("[error] sender was lost for client: {}", client.id),
    };
    if let Err(e) = sender.send(Ok(Message::text(
        serde_json::to_string(game_update).unwrap(),
    ))) {
        eprintln!(
            "[error] failed to send message to {} with err: {}",
            client.id, e,
        );
    }
}

/// Removes a client from the session that they currently exist under
async fn remove_client_from_current_session(
    client_id: &str,
    clients: &data_types::SafeClients,
    sessions: &data_types::SafeSessions,
    game_states: &data_types::SafeGameStates,
) {
    let session_id: String = match get_client_session_id(client_id, clients).await {
        Some(s_id) => s_id,
        None => return, // client did not exist in any session
    };

    let mut session_empty = false;
    if let Some(session) = sessions.write().await.get_mut(&session_id) {
        // notify all clients in the sessions that the client will be leaving
        notify_session(
            &ServerEvent::builder(ServerEventCode::ClientLeft)
                .data(ServerEventData::builder().client_id(client_id).build())
                .build(),
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
        // checks the statuses to see if any users are still active
        session_empty = session.get_clients_with_active_status(true).is_empty();
        // if the session is not empty, make someone else the owner
        if !session_empty {
            set_new_session_owner(session, &clients, &session.get_client_ids()[0]);
        }
    }
    // clean up the session from the map if it is empty
    // * we cannot do this in the scope above because because we are already holding a mutable reference to a session within the map
    if session_empty {
        cleanup_session(&session_id, sessions, game_states).await;
    }
}

/// Takes a mutable session reference in order to add a client to a given session
///
/// Uses a Read lock for Clients
async fn insert_client_into_given_session(
    client_id: &str,
    clients: &data_types::SafeClients,
    session: &mut session_types::Session,
) {
    // add client to session
    session.insert_client(client_id, true);
    // update session_id of client
    if let Some(client) = clients.write().await.get_mut(client_id) {
        client.session_id = Some(session.id.clone());
    }
    // notify all clients in the session that the client has joined
    notify_session(
        &ServerEvent::builder(ServerEventCode::ClientJoined)
            .data(
                ServerEventData::builder()
                    .session_id(&session.id)
                    .client_id(client_id)
                    .session_client_ids(&session.get_client_ids())
                    .build(),
            )
            .build(),
        &session,
        &clients,
    )
    .await;
}

fn set_new_session_owner(
    session: &mut session_types::Session,
    _clients: &data_types::SafeClients,
    client_id: &String,
) {
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

fn initialize_game_data(
    client_vec: &Vec<String>,
) -> Result<
    (
        Vec<String>,
        HashMap<String, shared_types::PlayerData>,
        Vec<shared_types::Card>,
    ),
    &str,
> {
    let player_count = client_vec.len();
    if player_count < 4 || player_count > 7 {
        return Err("Cannot Play with less than 4 Players or More than 7!");
    }

    let mut role_vec = Vec::with_capacity(player_count);
    role_vec.extend(
        [
            shared_types::Role::Renegade,
            shared_types::Role::Outlaw,
            shared_types::Role::Outlaw,
        ]
        .iter()
        .cloned(),
    );
    // 4 players: Sheriff, 1 Renegade, 2 Outlaws
    // 5 players: Sheriff, 1 Renegade, 2 Outlaws, 1 Deputy
    // 6 players: Sheriff, 1 Renegade, 3 Outlaws, 1 Deputy
    // 7 players: Sheriff, 1 Renegade, 3 Outlaws, 2 Deputy
    if player_count >= 5 {
        role_vec.push(shared_types::Role::Deputy);
    }
    if player_count >= 6 {
        role_vec.push(shared_types::Role::Outlaw);
    }
    if player_count >= 7 {
        role_vec.push(shared_types::Role::Deputy);
    }
    // random gen for shuffling
    let mut rand = WyRand::new();
    // shuffle the order characters
    rand.shuffle(&mut role_vec);
    // set sheriff as first turn
    role_vec.insert(0, shared_types::Role::Sheriff);
    // create client_vec copy that we will shuffle
    let mut playerinfo_vec = client_vec.clone();
    rand.shuffle(&mut playerinfo_vec);
    // map the random client_vec to the
    let mut player_data = playerinfo_vec
        .iter()
        .enumerate()
        .map(|(i, id)| {
            (
                id.clone(),
                shared_types::PlayerData {
                    max_health: 5,
                    health: 5, // TODO with character dictionary
                    field: Vec::new(),
                    hand: Vec::new(),
                    character: shared_types::Character::BillyTheKid, // TODO
                    role: role_vec[i].clone(),
                    alive: true,
                },
            )
        })
        .collect::<HashMap<String, shared_types::PlayerData>>();

    let mut deck = deck::generate_deck();

    for (_, data) in player_data.iter_mut() {
        for _ in 0..data.health {
            if let Some(card) = deck.pop() {
                data.hand.push(card);
            }
        }
    }

    return Ok((playerinfo_vec, player_data, deck));
}

/// Gets a random new session 1 that is 5 characters long
/// This should almost ensure session uniqueness when dealing with a sizeable number of sessions
fn get_rand_session_id() -> String {
    let alphabet: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    nanoid!(5, &alphabet)
}

/// pull the session id off of a client
async fn get_client_session_id(
    client_id: &str,
    clients: &data_types::SafeClients,
) -> Option<String> {
    if let Some(client) = &clients.read().await.get(client_id) {
        client.session_id.clone()
    } else {
        None
    }
}
