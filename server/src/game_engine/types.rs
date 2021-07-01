use crate::shared_types::{self, Card};
use std::collections::HashMap;

pub type GameStates = HashMap<String, GameState>;

#[derive(Clone)]
pub enum EventStateData {
    Cards(Vec<Card>),
    Players(Vec<String>),
    PlayersAndCards {
        cards: Vec<Card>,
        players: Vec<String>,
    },
    None,
}

#[derive(Clone)]
pub struct EventState {
    // the ID of the player who
    pub initiator: String,
    // the card which was played
    pub card_name: shared_types::CardName,
    // only respondents are allowed to play in response to an event
    // unless there is a card that can be played out of turn to cancel an effect (escaped?)
    pub respondents: Vec<String>,
    // data that is held for the span of the state in order to track actions
    pub data: EventStateData,
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
}

// pub type CharacterEffect = &'static str; /* i have no dam clue wat type this is */
pub struct CharacterData {
    pub hp: u8,
    pub triggers: &'static [EventTrigger],
    // pub effect: CharacterEffect,
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
    pub response_to: &'static [EventTrigger],
    pub triggers: &'static [EventTrigger],
    pub requirements: CardConditions,
    pub initiate: CardEffect,
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
