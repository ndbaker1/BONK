use crate::{
    game_engine,
    game_engine::characters,
    shared_types::{self, Card, CardName, ServerEvent, ServerEventCode, ServerEventData},
};
use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
};

use super::types::EventTrigger;

pub fn get_card_data(card_name: &shared_types::CardName) -> &'static game_engine::types::CardData {
    match card_name {
        CardName::Bang => &BANG_CARD_DATA,
        CardName::Missed => &MISSED_CARD_DATA,
        CardName::Indians => &INDIANS_CARD_DATA,
        CardName::Duel => &DUEL_CARD_DATA,
        CardName::GeneralStore => &GENERAL_STORE_CARD_DATA,
        CardName::Beer => &BEER_CARD_DATA,
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
    fn remove_cards_from_hand(&mut self, player: &str, cards: &Vec<Card>) {
        if let Some(player_data) = self.player_data.get_mut(player) {
            player_data.remove_cards(cards);
        } else {
            eprintln!("could not get player {} to remove cards", player);
        }
    }

    /// Adds cards to the hand of a player in the game
    fn add_cards_to_hand(&mut self, player: &str, cards: &Vec<Card>) {
        if let Some(player_data) = self.player_data.get_mut(player) {
            player_data.add_cards_to_hand(cards);
        } else {
            eprintln!("could not get player {} to add cards", player);
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
    response_to: &[],
    triggers: &[EventTrigger::Bang, EventTrigger::Damage],
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
            .push((CardName::Bang, targets.clone(), Vec::new()));

        // designate responses if effect involves others, otherwise relaying display data
        game_state
            .player_order
            .iter()
            .map(|player| {
                (
                    player.clone(),
                    ServerEvent::builder(ServerEventCode::Action)
                        .message(&format!(
                            "{} was targetted by a Bang from player {}",
                            targets[0], user_id
                        ))
                        .data(ServerEventData::builder().client_id(user_id).build())
                        .build(),
                )
            })
            // overrite the hashmap values of the targets in the messages
            .chain(
                [(
                    targets[0].to_string(),
                    ServerEvent::builder(ServerEventCode::Action)
                        .message(&format!("Targetted by a bang from player {}", user_id))
                        .data(ServerEventData::builder().client_id(user_id).build())
                        .build(),
                )]
                .iter()
                .cloned(),
            )
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
                                shared_types::ServerEvent::builder(ServerEventCode::Action)
                                    .message(&format!("{} avoided the bang!", user_id))
                                    .data(ServerEventData::builder().client_id(user_id).build())
                                    .build(),
                            )
                        })
                        .collect::<HashMap<String, shared_types::ServerEvent>>();
                }

                Err(err) => println!("[error] {}", err),
            }
        }

        if let Some(player) = game_state.player_data.get_mut(user_id) {
            game_state.event_state_stack.pop();

            player.health -= 1;

            if player.health <= 0 {
                player.alive = false;
            }

            let damage_announcement_event_builder = ServerEvent::builder(ServerEventCode::Damage)
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
    response_to: &[EventTrigger::Bang],
    triggers: &[],
    requirements: |user_id, _, _, game_state| {
        // the user has to currently be targetted by a bang!
        if let Some((card_name, players, _)) = game_state.event_state_stack.last() {
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
    response_to: &[],
    triggers: &[EventTrigger::Damage],
    requirements: |_, _, _, game_state| {
        if game_state.is_grace_period() {
            Err(String::from(GRACE_PERIOD_MSG))
        } else {
            Ok(())
        }
    },
    initiate: |user_id, cards, _, game_state| {
        // Remove card from the hand of the player
        game_state.remove_cards_from_hand(user_id, cards);
        // create a list of player which doesnt include the player who played the card
        let mut players = game_state.player_order.clone();
        players.retain(|player| player != user_id);
        // push the Indians event to the stack
        game_state
            .event_state_stack
            .push((CardName::Indians, players, Vec::new()));
        // create builder template for notifying players the card has been played
        let event_announcment = ServerEvent::builder(ServerEventCode::Action)
            .message(&format!("{} played Indians!", user_id));
        // return a map the event announcement to each player in the game
        game_state
            .player_order
            .iter()
            .filter(|player| *player != user_id)
            .map(|player| (player.clone(), event_announcment.build()))
            .collect::<HashMap<String, shared_types::ServerEvent>>()
    },
    update: |user_id, cards, _, game_state| {
        let mut messages = HashMap::new();
        // A card was played in response and we need to evaluate it
        if !cards.is_empty() {
            if cards[0].name == CardName::Bang {
                // remove the cards from the players hand
                game_state.remove_cards_from_hand(user_id, cards);

                if let Some((_, players, _)) = game_state.event_state_stack.last_mut() {
                    // remove the player from the list of expecting responses
                    players.retain(|player| player != user_id);
                    // if the player number is empty the event ends
                    if players.is_empty() {
                        // remove the event from the stack
                        game_state.event_state_stack.pop();
                    }
                } else {
                    eprintln!("Could not find event_stack for respones to indians.");
                }
            } else {
                messages.insert(
                    user_id.to_string(),
                    ServerEvent::builder(ServerEventCode::LogicError)
                        .message("Cannot play non-Bang card for Indians.")
                        .build(),
                );
            }
        } else {
            if let Some((_, players, _)) = game_state.event_state_stack.last_mut() {
                // remove the player from the list of expecting responses
                players.retain(|player| player != user_id);
                // if the player number is empty the event ends
                if players.is_empty() {
                    // remove the event from the stack
                    game_state.event_state_stack.pop();
                }
            } else {
                eprintln!("Could not find event_stack for respones to indians.");
            }
            // remove hp from player
            if let Some(player) = game_state.player_data.get_mut(user_id) {
                player.health -= 1;
            } else {
                eprintln!("Could not get player to decrease hp.");
            }
            // notify all players that this user took damage
            let damage_announcement = ServerEvent::builder(ServerEventCode::Damage)
                .data(ServerEventData::builder().client_id(user_id).build());

            messages.extend(
                game_state
                    .player_order
                    .iter()
                    .map(|player| (player.clone(), damage_announcement.build())),
            );
        }
        messages
    },
};

static DUEL_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    response_to: &[],
    triggers: &[EventTrigger::Damage],
    requirements: |_, _, targets, game_state| {
        // cannot play cards that cause damage on the first turn
        if game_state.is_grace_period() {
            Err(String::from(GRACE_PERIOD_MSG))
        }
        // can only the play the card on one opponent
        else if targets.len() != 1 {
            Err(String::from("Incorrect number of Targets."))
        }
        // the play is valid
        else {
            Ok(())
        }
    },
    initiate: |user_id, cards, targets, game_state| HashMap::new(),
    update: |user_id, cards, targets, game_state| HashMap::new(),
};

static GENERAL_STORE_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    response_to: &[],
    triggers: &[],
    requirements: |_, _, _, _| Ok(()),
    initiate: |user_id, cards, _, game_state| {
        // Remove card from the hand of the player
        game_state.remove_cards_from_hand(user_id, cards);
        // create a list of player which doesnt include the player who played the card
        let mut players = game_state
            .player_order
            .iter()
            .cloned()
            .collect::<VecDeque<String>>();
        // create a vec of the players which starts with the player who played the card
        if let Some(index) = players.iter().position(|player| player == user_id) {
            players.rotate_left(index);
        } else {
            eprintln!("could not find player {} in the game order", user_id);
        }

        let general_store_cards = game_state
            .deck
            .drain(..(game_state.deck.len() - players.len()))
            .collect::<Vec<Card>>();
        // push the General Store event to the stack
        game_state.event_state_stack.push((
            CardName::GeneralStore,
            players.into_iter().collect::<Vec<String>>(),
            general_store_cards.clone(),
        ));
        // create builder template for notifying players the card has been played
        let event_announcment = ServerEvent::builder(ServerEventCode::Action)
            .message(&format!("{} choosing a Card from General Store.", user_id))
            .data(
                ServerEventData::builder()
                    .card_options(&general_store_cards)
                    .build(),
            );
        // return a map of the event announcement to each player in the game
        game_state
            .player_order
            .iter()
            .filter(|player| *player != user_id)
            .map(|player| (player.clone(), event_announcment.build()))
            .collect::<HashMap<String, shared_types::ServerEvent>>()
    },
    update: |user_id, cards, _, game_state| {
        // Remove card from the hand of the player
        game_state.add_cards_to_hand(user_id, cards);
        // create a list of player which doesnt include the player who played the card
        if let Some((_, players, card_options)) = game_state.event_state_stack.last_mut() {
            // remove the player who picked the card
            // proceed to make the next player choose
            players.remove(0);
            // remove the card that was picked by the player
            card_options.retain(|card| card != &cards[0]);
            // end the event if all the players have taken their card
            if players.is_empty() {
                // pop the event from the stack
                game_state.event_state_stack.pop();
                // norify all users what card was chosen and complete the event
                HashMap::new()
            } else {
                // create builder template for notifying players of the next player choosing
                let event_announcment = ServerEvent::builder(ServerEventCode::Action)
                    .message(&format!(
                        "{} choosing a Card from General Store.",
                        players[0]
                    ))
                    .data(
                        ServerEventData::builder()
                            .card_options(&card_options)
                            .build(),
                    );
                // return a map of the event announcement to each player in the game
                game_state
                    .player_order
                    .iter()
                    .filter(|player| *player != user_id)
                    .map(|player| (player.clone(), event_announcment.build()))
                    .collect::<HashMap<String, shared_types::ServerEvent>>()
            }
        } else {
            eprintln!("failed to find an event to play on.");
            HashMap::new()
        }
    },
};

static BEER_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
    color: game_engine::types::CardColor::Brown,
    response_to: &[EventTrigger::Damage],
    triggers: &[EventTrigger::Heal],
    requirements: |user_id, _, _, game_state| {
        if let Some(player_data) = game_state.player_data.get(user_id) {
            if player_data.health == player_data.max_health {
                Err("cannot play beer at full health.".to_string())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    },
    initiate: |user_id, cards, _, game_state| {
        // Remove card from the hand of the player
        game_state.remove_cards_from_hand(user_id, cards);

        if let Some(player_data) = game_state.player_data.get_mut(user_id) {
            // increase the player health
            player_data.health += 1;

            let event_announcment = ServerEvent::builder(ServerEventCode::Action)
                .message(&format!("{} used a Beer to heal", user_id));

            return game_state
                .player_order
                .iter()
                .map(|player| (player.clone(), event_announcment.build()))
                .collect::<HashMap<String, shared_types::ServerEvent>>();
        } else {
            eprintln!("could not find player: {} in game.", user_id);
        }

        HashMap::new()
    },
    update: |_, _, _, _| HashMap::new(),
};
