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
  StartGame,
  EndTurn,
  PlayCard,
  DataRequest,
}
