/**
 * This file contains type defintions which are shared between the front and back end applications
 */
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize)]
pub struct ServerEventData {
    pub session_id: Option<String>,
    pub client_id: Option<String>,
    pub session_client_ids: Option<Vec<String>>,
    pub game_data: Option<GameData>,
    pub player_data: Option<PlayerData>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PlayerData {
    pub health: u8,
    pub hand: Vec<Card>,
    pub field: Vec<Card>,
    pub character: Character,
    pub role: Role,
    pub alive: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct GameData {
    pub round: usize,
    pub turn_index: usize,
    pub player_order: Vec<String>,
    pub discard: Vec<Card>,
}

#[derive(Debug, Serialize)]
pub struct ServerEvent {
    pub event_code: ServerEventCode,
    pub message: Option<String>,
    pub data: Option<ServerEventData>,
}

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum ServerEventCode {
    // session_id, client_id, session_client_ids
    ClientJoined = 1,
    // client_id
    ClientLeft,
    GameStarted,
    // session_id, session_client_ids
    DataResponse,
    // client_id
    TurnStart,
    LogicError,
    Draw,
    // indicated a decrease in player hp
    Damage,
    Targetted,
}

#[derive(Deserialize)]
pub struct ClientEvent {
    pub event_code: ClientEventCode,
    pub target_ids: Option<Vec<String>>,
    pub cards: Option<Vec<Card>>,
    pub session_id: Option<String>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ClientEventCode {
    // session_id
    JoinSession = 1,
    CreateSession,
    LeaveSession,
    DataRequest,
    StartGame,
    EndTurn,
    PlayerAction,
}

#[derive(Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct ResponseData {
    pub cards: Vec<CardName>,
    pub characters: Vec<Character>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Card {
    pub name: CardName,
    pub suit: CardSuit,
    pub rank: CardRank,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum CardName {
    // Brown Cards
    Bang = 1,
    Hatchet,
    Indians,
    Missed,
    Beer,
    // Blue Cards
    Barrel,
    Dynamite,
    // Green Cards
    PonyExpress,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum CardSuit {
    Clubs = 1,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum CardRank {
    N1 = 1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    J,
    Q,
    K,
    A,
}

#[derive(Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Role {
    Sheriff = 1,
    Renegade,
    Outlaw,
    Deputy,
}

#[derive(Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Character {
    BillyTheKid = 1,
}
