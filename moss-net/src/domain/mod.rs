pub mod model;
pub mod port;
pub mod service;

use axum::Error as AxumError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use surrealdb::Error as SurrealError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration not initialized correct: {0}")]
    Configuration(String),

    #[error("The request body contains invalid data")]
    Request,

    #[error("There was an error with the network: {0}")]
    Axum(#[from] AxumError),

    #[error("There was a problem with serialization/deserialization: {0}")]
    Serde(#[from] SerdeError),

    #[error("There was a problem with the database: {0}")]
    Db(#[from] SurrealError),

    #[error("Couldn't open the specified file: {0}")]
    Io(#[from] IoError),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
