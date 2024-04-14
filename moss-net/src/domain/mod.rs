pub mod model;
pub mod port;
pub mod service;

use axum::Error as AxumError;
use sea_orm::DbErr as SeaOrmDbError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
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

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
