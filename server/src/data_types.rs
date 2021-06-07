use crate::{game_engine, session_types};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SafeResource<T> = Arc<RwLock<T>>;

pub type SafeClients = SafeResource<session_types::Clients>;
pub type SafeSessions = SafeResource<session_types::Sessions>;
pub type SafeGameStates = SafeResource<game_engine::types::GameStates>;
