use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type CardID = u8;

#[derive(Serialize)]
pub struct ServerEvent {
  pub event_code: ServerEventCodes,
  pub session_id: Option<String>,
  pub client_id: Option<String>,
  pub session_client_ids: Option<Vec<String>>,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ServerEventCodes {
  ClientJoined = 1,
  ClientLeft,
  GameStarted,
  TurnStart,
  DataResponse,
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
  JoinSession = 1,
  CreateSession,
  LeaveSession,
  StartGame,
  EndTurn,
  PlayCard,
  DataRequest,
}
