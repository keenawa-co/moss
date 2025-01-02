use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct TauriError(String);

impl std::fmt::Display for TauriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<anyhow::Error> for TauriError {
    fn from(e: anyhow::Error) -> Self {
        TauriError(e.to_string())
    }
}

pub type TauriResult<T> = Result<T, TauriError>;
