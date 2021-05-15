use crate::{game_types, session_types};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SafeResource<T> = Arc<RwLock<T>>;

pub type SafeClients = SafeResource<session_types::Clients>;
pub type SafeSessions = SafeResource<session_types::Sessions>;
pub type SafeGameStates = SafeResource<game_types::GameStates>;
pub type SafeGameDictionary = Arc<game_types::GameDictionary>;
