use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

pub fn tool_definition() -> Value {
    serde_json::json!({
        "name": "evaluate",
        "description": "Évalue une expression mathématique. Supporte +,-,*,/,^,%, sin, cos, tan, sqrt, log, abs, ceil, floor, round, et les constantes pi et e.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "L'expression mathématique à calculer, ex: 'sqrt(x^2 + y^2)' ou '2*pi*r'"
                },
                "vars": {
                    "type": "object",
                    "description": "Variables nommées utilisées dans l'expression, ex: {\"x\": 3.0, \"y\": 4.0}",
                    "additionalProperties": { "type": "number" }
                }
            },
            "required": ["expression"]
        }
    })
}
