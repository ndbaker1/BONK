use std::sync::Arc;
use tokio::sync::RwLock;

pub type SafeResource<T> = Arc<RwLock<T>>;
