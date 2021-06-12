use crate::shared_types;
use std::collections::HashMap;

pub type GameStates = HashMap<String, GameState>;

#[derive(Clone)]
pub struct GameState {
  pub round: usize,
  pub turn_index: usize,
  pub player_order: Vec<String>,
  pub player_data: HashMap<String, shared_types::PlayerData>,
  pub deck: Vec<shared_types::Card>,
  pub discard: Vec<shared_types::Card>,
  // data for dynamic gameplay
  pub event_stack: Vec<(shared_types::CardName, Vec<String>)>,
  pub card_events: Vec<shared_types::CardName>,
  pub trigger_queue: HashMap<String, shared_types::ResponseData>,
  pub response_queue: HashMap<String, shared_types::ResponseData>,
}
impl GameState {
  pub fn to_game_data(&self) -> shared_types::GameData {
    shared_types::GameData {
      round: self.round,
      turn_index: self.turn_index,
      card_events: self.card_events.clone(),
      discard: self.discard.clone(),
      player_order: self.player_order.clone(),
    }
  }
}

pub type CharacterEffect = &'static str; /* i have no dam clue wat type this is */
pub struct CharacterData {
  pub hp: u8,
  pub triggers: &'static [EventTrigger],
  pub effect: CharacterEffect,
  pub effect_optional: bool,
}

/// Card Preconditions should be game-logic based.
/// Do not worry about the player having the Cards or any state-based logic
pub type CardConditions =
  fn(&str, &Vec<shared_types::Card>, &Vec<String>, &mut GameState) -> Result<(), String>;

///
pub type CardEffect = fn(
  &str,
  &Vec<shared_types::Card>,
  &Vec<String>,
  &mut GameState,
) -> HashMap<String, shared_types::ServerEvent>;

/// A function which makes modifications to a GameState as a result of game mechanics
pub type GameStateUpdate = fn(&str, &Vec<shared_types::Card>, &Vec<String>, &mut GameState) -> ();

pub struct CardData {
  pub color: CardColor,

  pub triggers: &'static [EventTrigger],
  pub preconditions: CardConditions,
  pub effect: CardEffect,
  pub update: GameStateUpdate,
}

#[derive(Debug, Clone)]
pub enum CardColor {
  Brown = 1,
  Blue,
  Green,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventTrigger {
  Damage = 1,
  Draw,
  Bang,
  Heal,
  Target,
  EndOfTurnDiscard,
  EffectDiscard,
}
