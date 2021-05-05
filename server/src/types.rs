use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::{HashMap, HashSet};

pub type CardID = u8;

#[derive(Serialize)]
pub struct ServerEvent {
  pub event_code: ServerEventCodes,
  pub session_id: Option<String>,
  pub client_id: Option<String>,
  pub session_client_ids: Option<Vec<String>>,
  pub game_state: Option<GameState>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GameState {
  pub turn_index: usize,
  pub turn_orders: Vec<PlayerInfo>,
  pub player_blue_cards: HashMap<String, HashSet<BlueCards>>,
  pub player_green_cards: HashMap<String, HashSet<GreenCards>>,
  pub effect: Option<EffectCodes>,
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum EffectCodes {
  GeneralStore = 1,
}

#[derive(Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum BlueCards {
  Barrel = 1,
  Dynamite,
}

#[derive(Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum GreenCards {
  PonyExpress = 1,
}

#[derive(Serialize, Debug, Clone)]
pub struct PlayerInfo {
  pub client_id: String,
  pub character_code: CharacterCodes,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ServerEventCodes {
  // session_id, client_id, session_client_ids
  ClientJoined = 1,
  // client_id
  ClientLeft,
  GameStarted,
  // session_id, session_client_ids
  DataResponse,
  // session_id, client_id
  InvalidSessionID,
}

#[derive(Deserialize)]
pub struct ClientEvent {
  pub event_code: ClientEventCodes,
  pub target_ids: Option<Vec<String>>,
  pub card_id: Option<CardID>,
  pub session_id: Option<String>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ClientEventCodes {
  // session_id
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  DataRequest,
  StartGame,
  EndTurn,
  PlayCard,
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum CharacterCodes {
  Sheriff = 1,
  Renegade,
  Outlaw,
  Deputy,
}
