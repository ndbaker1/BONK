use crate::{game_engine, shared_types};
use std::collections::HashMap;

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
      let character_data = get_character_data(&player_data.character);
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

/// Creates a starting deck of Cards for the game
pub fn generate_deck() -> Vec<shared_types::Card> {
  let mut deck: Vec<shared_types::Card> = Vec::with_capacity(80);
  // copying same cards atm
  for _ in 0..20 {
    deck.push(shared_types::Card {
      name: shared_types::CardName::Bang,
      suit: shared_types::CardSuit::Clubs,
      rank: shared_types::CardRank::N1,
    });
    deck.push(shared_types::Card {
      name: shared_types::CardName::Bang,
      suit: shared_types::CardSuit::Diamonds,
      rank: shared_types::CardRank::N2,
    });
    deck.push(shared_types::Card {
      name: shared_types::CardName::Missed,
      suit: shared_types::CardSuit::Hearts,
      rank: shared_types::CardRank::N1,
    });
    deck.push(shared_types::Card {
      name: shared_types::CardName::Missed,
      suit: shared_types::CardSuit::Spades,
      rank: shared_types::CardRank::N2,
    });
  }
  game_engine::shuffle_deck(&mut deck);
  return deck;
}

pub fn get_card_data(card_name: &shared_types::CardName) -> &'static game_engine::types::CardData {
  match card_name {
    shared_types::CardName::Bang => &BANG_CARD_DATA,
    shared_types::CardName::Missed => &MISSED_CARD_DATA,
    _ => &BANG_CARD_DATA,
  }
}

pub fn get_character_data(
  character: &shared_types::Character,
) -> &'static game_engine::types::CharacterData {
  match character {
    shared_types::Character::BillyTheKid => &BILLYTHEKID_CHARACTER_DATA,
  }
}

// HELPER FUNCTIONS TO BE MOVED TO AN APPROPRIATE LOCATION LATER

fn get_player_distance(_player: &String) -> u8 {
  return 1;
}

fn player_range() -> u8 {
  return 1;
}

// static game data

// character data

static BILLYTHEKID_CHARACTER_DATA: game_engine::types::CharacterData =
  game_engine::types::CharacterData {
    hp: 5,
    triggers: &[game_engine::types::EventTrigger::Damage],
    effect_optional: true,
    effect: "wat is this type, idk ",
  };

// card data

static BANG_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
  color: game_engine::types::CardColor::Brown,
  triggers: &[],
  preconditions: |user_id, _, targets, game_state| {
    match game_state.player_data.get(user_id) {
      Some(player_data) => {
        if get_player_distance(&targets[0]) > player_range() {
          return Err(String::from("Target out of range."));
        }
      }
      None => return Err(String::from("Player does not have the cards")),
    }
    if targets.len() != 1 {
      return Err(String::from("Wrong number of Targets for a Bang"));
    }
    return Ok(());
  },
  effect: |user_id, cards, targets, game_state| {
    let responses =
      game_state.trigger_responses(&vec![game_engine::types::EventTrigger::Damage], targets);

    game_state.remove_cards_from_hand(user_id, cards);

    if responses.is_empty() {
      for (card_name, targets) in game_state.event_stack.iter() {
        // dsd
      }
    } else {
      // set the current card for this action
      game_state.card_events.push(cards[0].name.clone());
      // set the expectng responses
      game_state.trigger_queue = responses.clone();
      game_state.response_queue = responses;
    }
    return HashMap::new();
  },
  update: |user_id, cards, targets, game_state| {
    if let Some(player) = game_state.player_data.get_mut(&targets[0]) {
      player.health -= 1;
    }
  },
};

static MISSED_CARD_DATA: game_engine::types::CardData = game_engine::types::CardData {
  color: game_engine::types::CardColor::Brown,
  triggers: &[game_engine::types::EventTrigger::Damage],
  preconditions: |user_id, _, targets, game_state| {
    // the user has to currently be targetted by a bang!
    return Ok(());
  },
  effect: |user_id, cards, targets, game_state| {
    // negates the band effect, meaning the player takes no damage
    return HashMap::new();
  },
  update: |user_id, cards, targets, game_state| {
    // there is no effect on the game state for responding to a bang with a missed
  },
};
