use crate::{
    game_engine,
    game_engine::characters,
    shared_types::{self, CardName, ServerEvent, ServerEventData},
};
use std::{cmp::min, collections::HashMap};

pub fn get_card_data(card_name: &shared_types::CardName) -> &'static game_engine::types::CardData {
    match card_name {
        shared_types::CardName::Bang => &BANG_CARD_DATA,
        shared_types::CardName::Missed => &MISSED_CARD_DATA,
        shared_types::CardName::Indians => &INDIANS_CARD_DATA,
        shared_types::CardName::Duel => &DUEL_CARD_DATA,
        shared_types::CardName::GeneralStore => &GENERAL_STORE_CARD_DATA,
        _ => &BANG_CARD_DATA,
    }
}

const GRACE_PERIOD_MSG: &str = "Cannot damage other players during the first round.";

impl game_engine::types::GameState {
    fn is_grace_period(&self) -> bool {
        self.round == 0
    }
    /// calculate the distance from one player to another.
    /// Dead players do not contribute to the distance.
    fn player_distance_from_to(&self, from_player: &str, to_player: &str) -> Result<i8, ()> {
        let alive_player_order = self
            .player_order
            .iter()
            .filter(|player| match self.player_data.get(*player) {
                Some(player_data) => player_data.alive,
                None => false,
            })
            .collect::<Vec<&String>>();

        let order_length = alive_player_order.len() as i8;
        let from_player_pos = match alive_player_order
            .iter()
            .position(|player| *player == from_player)
        {
            Some(index) => index as i8,
            None => return Err(()),
        };
        let to_player_pos = match alive_player_order
            .iter()
            .position(|player| *player == to_player)
        {
            Some(index) => index as i8,
            None => return Err(()),
        };

        return Ok(min(
            (to_player_pos - from_player_pos).rem_euclid(order_length),
            (from_player_pos - to_player_pos).rem_euclid(order_length),
        ));
    }
}

impl shared_types::PlayerData {
    fn range(&self) -> i8 {
        return 1;
    }
}

impl game_engine::types::GameState {
    /// Removes cards from the hand of a player in the game
    fn remove_cards_from_hand(&mut self, player: &str, cards: &Vec<shared_types::Card>) {
        if let Some(player_data) = self.player_data.get_mut(player) {
            player_data.remove_cards(cards);
        }
    }

    /// This could be a card response or a character ability response.
    fn trigger_responses(
        &mut self,
        triggers: &Vec<game_engine::types::EventTrigger>,
        targets: &Vec<String>,
    ) -> HashMap<String, shared_types::ResponseData> {
        let mut responses: HashMap<String, shared_types::ResponseData> = HashMap::new();
        // check what the possible actions of anyone in the lobby are when a card is played or effect is activated
        // ex.. A targets B for Bang!, B has 2 missed cards and C can also choose to play a helping card to save B

        for (player_id, player_data) in self.player_data.iter() {
            // check if current players being checked is one of the targets
            let is_target: bool = targets.contains(player_id);

            // Search Character abilities that may activate
            let character_data = characters::get_character_data(&player_data.character);
            // if the player is one of the targets
            if triggers
                .iter()
                .any(|trigger| character_data.triggers.contains(trigger))
            {
                if is_target {
                    if character_data.effect_optional {
                        // let response_data: &mut shared_types::ResponseData = responses
                        //   .entry(player_id.clone())
                        //   .or_insert(shared_types::ResponseData {
                        //     cards: Vec::new(),
                        //     characters: Vec::new(),
                        //   });
                        // response_data.characters.push(character_data.)
                    } else {
                        // character_data.effect activate
                    }
                } else {
                    // player not target code
                }
            }

            // Search Character Hand & Field for possible Card responses
            for card in player_data.card_iter() {
                let card_data = get_card_data(&card.name);
                if is_target {
                    if triggers
                        .iter()
                        .any(|trigger| card_data.triggers.contains(trigger))
                    {
                        let response_data: &mut shared_types::ResponseData = responses
                            .entry(player_id.clone())
                            .or_insert(shared_types::ResponseData {
                                cards: Vec::new(),
                                characters: Vec::new(),
                            });
                        response_data.cards.push(card.name.clone());
                    }
                } else {
                    // player not target code
                    // another player could play a card to assist or counter a player's move
                }
            }
        }

        // send messages to players who have the option to use an ability,
        // execute those that activate by default

        // update all players on the activations that took place, then
        // send a message to all players and wait for their responses...

        return responses;
    }
}

static BANG_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    triggers: &[],
    requirements: |user_id, _, targets, game_state| {
        if game_state.is_grace_period() {
            return Err(String::from(GRACE_PERIOD_MSG));
        }
        match game_state.player_distance_from_to(user_id, &targets[0]) {
            Ok(distance) => match game_state.player_data.get(user_id) {
                Some(player_data) => {
                    if distance > player_data.range() {
                        return Err(String::from("Target out of range."));
                    }
                }
                None => {
                    return Err(String::from(
                        "Failed to get Player Data for range calculation",
                    ))
                }
            },
            Err(_) => return Err(String::from("Failed to calculate distance between players")),
        };
        if targets.len() != 1 {
            return Err(String::from("Wrong number of Targets for a Bang"));
        }
        Ok(())
    },
    initiate: |user_id, cards, targets, game_state| {
        // skip handling the user response suggestions for now
        // let responses =
        //     game_state.trigger_responses(&vec![game_engine::types::EventTrigger::Damage], targets);

        game_state.remove_cards_from_hand(user_id, cards);

        game_state
            .event_state_stack
            .push((cards[0].name.clone(), targets.clone(), Vec::new()));

        // designate responses if effect involves others, otherwise relaying display data
        game_state
            .player_order
            .iter()
            .map(|player| {
                (
                    player.clone(),
                    shared_types::ServerEvent {
                        event_code: shared_types::ServerEventCode::Targetted,
                        message: None,
                        data: Some(ServerEventData {
                            client_id: Some(String::from(user_id)),
                            game_data: None,
                            player_data: None,
                            session_client_ids: None,
                            session_id: None,
                        }),
                    },
                )
            })
            // overrite the hashmap values of the targets in the messages
            .chain(targets.iter().map(|player| {
                (
                    player.clone(),
                    shared_types::ServerEvent {
                        event_code: shared_types::ServerEventCode::LogicError, // temp just to print
                        message: Some(String::from(format!(
                            "Targetted by a bang from player {}",
                            user_id
                        ))),
                        data: Some(ServerEventData {
                            client_id: Some(String::from(user_id)),
                            game_data: None,
                            player_data: None,
                            session_client_ids: None,
                            session_id: None,
                        }),
                    },
                )
            }))
            .collect::<HashMap<String, shared_types::ServerEvent>>()
    },
    update: |user_id, cards, targets, game_state| {
        // check if the card is played to reponse
        if !cards.is_empty() {
            let card_data = get_card_data(&cards[0].name);
            match (card_data.requirements)(user_id, &Vec::new(), &Vec::new(), game_state) {
                Ok(_) => {
                    game_state.remove_cards_from_hand(user_id, cards);

                    game_state.event_state_stack.pop();

                    return game_state
                        .player_order
                        .iter()
                        .map(|player| {
                            (
                                player.clone(),
                                shared_types::ServerEvent {
                                    event_code: shared_types::ServerEventCode::LogicError, // should be damage later
                                    message: Some(String::from(format!(
                                        "{} avoided the bang!",
                                        user_id
                                    ))),
                                    data: Some(ServerEventData {
                                        client_id: Some(String::from(user_id)),
                                        game_data: None,
                                        player_data: None,
                                        session_client_ids: None,
                                        session_id: None,
                                    }),
                                },
                            )
                        })
                        .collect::<HashMap<String, shared_types::ServerEvent>>();
                }

                Err(err) => println!("[error] {}", err),
            }
        }

        if let Some(player) = game_state.player_data.get_mut(user_id) {
            game_state.event_state_stack.pop();

            if player.health < 1 {
                player.health = 0;
            } else {
                player.health -= 1;
            }

            if player.health <= 0 {
                player.alive = false;
            }

            let damage_announcement_event_builder =
                ServerEvent::builder(shared_types::ServerEventCode::LogicError)
                    .message(&format!("{} takes 1 damage!", user_id))
                    .data(ServerEventData::builder().client_id(user_id).build());

            game_state
                .player_order
                .iter()
                .map(|player| (player.clone(), damage_announcement_event_builder.build()))
                .collect::<HashMap<String, shared_types::ServerEvent>>()
        } else {
            eprintln!("[error] Player not found in game.");
            HashMap::new()
        }
    },
};

static MISSED_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    triggers: &[game_engine::types::EventTrigger::Damage],
    requirements: |user_id, _, _, game_state| {
        // the user has to currently be targetted by a bang!
        if let Some((card_name, players, _)) = game_state.event_state_stack.first() {
            if !get_card_data(card_name)
                .triggers
                .into_iter()
                .any(|trigger| MISSED_CARD_DATA.triggers.contains(trigger))
            {
                Err(String::from("Nothing to play missed for."))
            } else if players.len() != 1 || players[0] != user_id {
                Err(String::from(
                    "Player is not in the list of expected responses.",
                ))
            } else {
                Ok(())
            }
        } else {
            Err(String::from("No State found for responding with a Missed."))
        }
    },
    initiate: |_, _, _, _| HashMap::new(),
    update: |_, _, _, _| HashMap::new(),
};

static INDIANS_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    triggers: &[],
    requirements: |_, _, _, game_state| {
        if game_state.is_grace_period() {
            Err(String::from(GRACE_PERIOD_MSG))
        } else {
            Ok(())
        }
    },
    initiate: |user_id, cards, _, game_state| {
        game_state.remove_cards_from_hand(user_id, cards);

        let mut players = game_state.player_order.clone();
        players.retain(|player| player != user_id);

        game_state
            .event_state_stack
            .push((cards[0].name.clone(), players, Vec::new()));

        HashMap::new()
    },
    update: |user_id, cards, targets, game_state| {
        let mut messages = HashMap::new();
        if !cards.is_empty() {
            if cards[0].name != CardName::Bang {
                messages.insert(
                    user_id.to_string(),
                    ServerEvent::builder(shared_types::ServerEventCode::LogicError)
                        .message("Cannot play non-Bang card for Indians.")
                        .build(),
                );
            } else {
            }
        } else {
        }
        messages
    },
};

static DUEL_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    triggers: &[],
    requirements: |user_id, _, targets, game_state| {
        if game_state.is_grace_period() {
            Err(String::from(GRACE_PERIOD_MSG))
        } else {
            Ok(())
        }
    },
    initiate: |user_id, cards, targets, game_state| {
        // negates the band effect, meaning the player takes no damage
        HashMap::new()
    },
    update: |user_id, cards, targets, game_state| {
        // there is no effect on the game state for responding to a bang with a missed
        HashMap::new()
    },
};

static GENERAL_STORE_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    triggers: &[],
    requirements: |user_id, _, targets, game_state| Ok(()),
    initiate: |user_id, cards, targets, game_state| {
        // negates the band effect, meaning the player takes no damage
        HashMap::new()
    },
    update: |user_id, cards, targets, game_state| {
        // there is no effect on the game state for responding to a bang with a missed
        HashMap::new()
    },
};
