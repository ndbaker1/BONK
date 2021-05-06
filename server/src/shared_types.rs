use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ServerEvent {
  pub event_code: ServerEventCode,
  pub message: Option<String>,
  pub data: Option<ServerEventData>,
}

#[derive(Serialize)]
pub struct ServerEventData {
  pub session_id: Option<String>,
  pub client_id: Option<String>,
  pub session_client_ids: Option<Vec<String>>,
  pub game_state: Option<GameState>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GameState {
  pub turn_index: usize,
  pub turn_orders: Vec<PlayerInfo>,
  pub player_hands: HashMap<String, Vec<CardCode>>,
  pub player_fields: HashMap<String, Vec<CardCode>>,
  pub effect: Option<EffectCode>,
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum EffectCode {
  GeneralStore = 1,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum CardCode {
  // Brown Cards
  Bang = 1,
  // Blue Cards
  Barrel,
  Dynamite,
  // Green Cards
  PonyExpress,
}

#[derive(Serialize, Debug, Clone)]
pub struct PlayerInfo {
  pub client_id: String,
  pub character_code: CharacterCode,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ServerEventCode {
  // session_id, client_id, session_client_ids
  ClientJoined = 1,
  // client_id
  ClientLeft,
  GameStarted,
  // session_id, session_client_ids
  DataResponse,
  // session_id, client_id
  InvalidSessionID,
  // client_id
  TurnStart,
  LogicError,
}

#[derive(Deserialize)]
pub struct ClientEvent {
  pub event_code: ClientEventCode,
  pub target_ids: Option<Vec<String>>,
  pub card_code: Option<CardCode>,
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
  PlayCard,
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum CharacterCode {
  Sheriff = 1,
  Renegade,
  Outlaw,
  Deputy,
}
