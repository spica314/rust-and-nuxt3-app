use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub value: Arc<Mutex<i32>>,
}
