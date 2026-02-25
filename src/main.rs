mod error;
mod messages;
mod mcp;
mod state;
mod types;

use std::io::{self, BufRead, Write};
use messages::JsonRpcRequest;
use state::AppState;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut state = AppState::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) if l.trim().is_empty() => continue,
            Ok(l) => l,
            Err(e) => {
                eprintln!("Erreur lecture stdin: {e}");
                break;
            }
        };

        let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
            Err(e) => serde_json::json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {
                    "code": -32700,
                    "message": format!("Parse error: {e}")
                }
            }),
            Ok(req) => {
                eprintln!("[mcp-math] méthode reçue: {}", req.method);
                match req.method.as_str() {
                    "initialize"        => mcp::handle_initialize(req.id),
                    "tools/list"        => mcp::handle_list_tools(req.id),
                    "tools/call"        => mcp::handle_call_tool(req.id, req.params, &mut state),
                    "notifications/initialized" => continue, // notification, pas de réponse
                    other               => mcp::handle_unknown_method(req.id, other),
                }
            }
        };

        if let Ok(json) = serde_json::to_string(&response) {
            writeln!(out, "{json}").ok();
            out.flush().ok();
        }
    }
}
