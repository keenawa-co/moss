use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[macro_export]
macro_rules! internal {
    ($err:expr) => {{
        $crate::domain::Error::Internal(
            $crate::domain::model::result::InternalError::Unknown(format!("{}:{}: {}", file!(), line!(), $err)),
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Internal(
            $crate::domain::model::result::InternalError::Unknown(format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),*))),
        )
    }};
}

#[macro_export]
macro_rules! bad {
    ($err:expr) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::BadRequest(format!("{}:{}: {}", file!(), line!(), $err)),
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::BadRequest(format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),*))),
        )
    }};
}

#[macro_export]
macro_rules! not_found {
    ($err:expr) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::NotFound(format!("{}:{}: {}", file!(), line!(), $err)),
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::NotFound(format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),*))),
        )
    }};
}

macro_rules! transparent_error {
    ($outer_variant:ident, $inner_variant:path, $err_type:ty) => {
        impl From<$err_type> for Error {
            fn from(err: $err_type) -> Self {
                Error::$outer_variant($inner_variant(err))
            }
        }
    };
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Client(ClientError),

    #[error(transparent)]
    Internal(InternalError),
}

transparent_error!(Internal, InternalError::Database, sea_orm::DbErr);
transparent_error!(Internal, InternalError::Io, std::io::Error);
transparent_error!(Internal, InternalError::Anyhow, anyhow::Error);
transparent_error!(Internal, InternalError::Notify, notify::Error);
transparent_error!(Internal, InternalError::Axum, axum::Error);

transparent_error!(Client, ClientError::Serde, serde_json::Error);

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("Cannot or will not process the request: {0}")]
    BadRequest(String),

    #[error("Cannot find the requested resource: {0}")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Unknown server error: {0}")]
    Unknown(String),

    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Axum(#[from] axum::Error),

    #[error(transparent)]
    Notify(#[from] notify::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
