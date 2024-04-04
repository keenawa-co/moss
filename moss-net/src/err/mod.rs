use axum::Error as AxumError;
use std::io::Error as IoError;
use surrealdb::Error as SurrealError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The request body contains invalid data")]
    Request,

    #[error("There was an error with the network: {0}")]
    Axum(#[from] AxumError),

    #[error("There was a problem with the database: {0}")]
    Db(#[from] SurrealError),

    #[error("Couldn't open the specified file: {0}")]
    Io(#[from] IoError),
}
