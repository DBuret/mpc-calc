use std::collections::HashMap;
use mathexpr::CompiledExpr;

pub struct AppState {
    // Cache: expression_string -> compiled expression
    pub cache: HashMap<String, CompiledExpr>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            cache: HashMap::new(),
        }
    }
}
