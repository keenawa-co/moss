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
    #[error("Configuration not initialized correct: {0}")]
    Configuration(String),

    #[error("The request body contains invalid data")]
    Request,

    #[error("There was an error with the network: {0}")]
    Axum(#[from] AxumError),

    #[error("There was a problem with serialization/deserialization: {0}")]
    Serde(#[from] SerdeError),

    #[error("There was a problem with the database: {0}")]
    DbError(DbError),

    #[error("Couldn't open the specified file: {0}")]
    Io(#[from] IoError),
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::DbError(DbError::Original(err))
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("No record found for the specified ID: {0}")]
    RecordNotFound(String),

    #[error("There was a problem executing the database operation: {0}")]
    Original(#[from] SeaOrmDbError),
}

pub(crate) fn error_record_not_found(id: impl ToString) -> Error {
    Error::DbError(DbError::RecordNotFound(id.to_string()))
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
