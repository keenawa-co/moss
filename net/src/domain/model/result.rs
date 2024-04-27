use anyhow::Error as AnyhowError;
use axum::Error as AxumError;
use notify::Error as NotifyError;
use sea_orm::DbErr as SeaOrmDbError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Notify(#[from] NotifyError),

    #[error("{0}")]
    Anyhow(#[from] AnyhowError),

    #[error("Invalid request body")]
    Request,

    #[error("Network error: {0}")]
    Axum(#[from] AxumError),

    #[error("Serialization/deserialization error: {0}")]
    Serde(#[from] SerdeError),

    #[error("Database operation failed: {0}")]
    Database(
        #[source]
        #[from]
        SeaOrmDbError,
    ),

    #[error("File access error: {0}")]
    Io(#[from] IoError),

    #[error("Unknown internal error: {0}")]
    Internal(String),
}

#[macro_export]
macro_rules! internal {
    ($err:expr) => {{
        $crate::domain::Error::Internal(format!("{}:{}: {}", file!(), line!(), $err))
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Internal(format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),*)))
    }};
}
