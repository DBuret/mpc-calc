use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Parse(String),
    Compile(String),
    Eval(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e)      => write!(f, "IO error: {e}"),
            AppError::Json(e)    => write!(f, "JSON error: {e}"),
            AppError::Parse(e)   => write!(f, "Parse error: {e}"),
            AppError::Compile(e) => write!(f, "Compile error: {e}"),
            AppError::Eval(e)    => write!(f, "Eval error: {e}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self { AppError::Json(e) }
}
