use std::collections::HashMap;
use tokio::sync::{broadcast, Mutex};

use mathexpr::Executable;
pub type CompiledExpr = Executable;

pub struct MathState {
    pub cache: HashMap<String, CompiledExpr>,
}

impl MathState {
    pub fn new() -> Self {
        MathState { cache: HashMap::new() }
    }
}

pub struct AppState {
    pub tx: broadcast::Sender<String>,
    pub math_state: Mutex<MathState>,
}

impl AppState {
    pub fn new(tx: broadcast::Sender<String>) -> Self {
        AppState {
            tx,
            math_state: Mutex::new(MathState::new()),
        }
    }
}
