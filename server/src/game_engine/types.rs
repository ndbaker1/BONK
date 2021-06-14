use crate::shared_types;
use std::collections::HashMap;

pub type GameStates = HashMap<String, GameState>;

/// the card whose effect is being played, and the list of players it wants a respons from
type EventState = (shared_types::CardName, Vec<String>);

#[derive(Clone)]
pub struct EventData {
    cards: Option<Vec<shared_types::Card>>,
}

#[derive(Clone)]
pub struct GameState {
    pub round: usize,
    pub turn_index: usize,
    pub player_order: Vec<String>,
    pub player_data: HashMap<String, shared_types::PlayerData>,
    pub deck: Vec<shared_types::Card>,
    pub discard: Vec<shared_types::Card>,
    // data for dynamic gameplay
    pub event_state_stack: Vec<EventState>,
    pub event_data: Option<EventData>,
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

/// how a card effects the event state of the Game
pub type CardEffect = fn(
    &str,
    &Vec<shared_types::Card>,
    &Vec<String>,
    &mut GameState,
) -> HashMap<String, shared_types::ServerEvent>;

/// A function which makes modifications to a GameState as a result of game mechanics
pub type GameStateUpdate = fn(
    &str,
    &Vec<shared_types::Card>,
    &Vec<String>,
    &mut GameState,
) -> HashMap<String, shared_types::ServerEvent>;

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
