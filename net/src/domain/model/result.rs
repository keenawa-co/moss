use std::fmt;

use async_graphql::{ErrorExtensionValues, ErrorExtensions};
use bitflags::bitflags;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[macro_export]
macro_rules! internal {
    ($msg:expr) => {{
        $crate::domain::Error::Internal(
            $crate::domain::model::result::InternalError::Unknown(
                format!("{}:{}: {}", file!(), line!(), $msg),
                None
            )
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Internal(
            $crate::domain::model::result::InternalError::Unknown(
                format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
                None
            )
        )
    }};
}

#[macro_export]
macro_rules! bad_request {
    ($msg:expr) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::BadRequest(
                format!("{}:{}: {}", file!(), line!(), $msg),
                None
            )
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::BadRequest(
                format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
                None
            )
        )
    }};
}

#[macro_export]
macro_rules! not_found {
    ($msg:expr) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::NotFound(
                format!("{}:{}: {}", file!(), line!(), $msg),
                None
            )
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::Error::Client(
            $crate::domain::model::result::ClientError::NotFound(
                format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
                None
            )
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
   pub struct ErrorCode: u32 {
        const EXPECTED_BUT_NOT_FOUND = 0b00000001;
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bits = Vec::new();

        if self.contains(ErrorCode::EXPECTED_BUT_NOT_FOUND) {
            bits.push("0b00000001");
        }

        if bits.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", bits.join("|"))
        }
    }
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

    #[error("Cannot or will not process the request. {0}")]
    BadRequest(String, Option<ErrorCode>),

    #[error("Cannot find the requested resource. {0}")]
    NotFound(String, Option<ErrorCode>),

    #[error("The origin server requires the request to be conditional. {0}")]
    PreconditionRequired(String, Option<ErrorCode>),
}

#[derive(Error, Debug)]
pub enum InternalError {
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

    #[error("An unexpected internal error occurred. {0}")]
    Unknown(String, Option<ErrorCode>),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        let (summary, detail) = if let Some((summary, detail)) = self.to_string().split_once(".") {
            (summary.trim().to_string(), Some(detail.trim().to_string()))
        } else {
            (self.to_string().trim().to_string(), None)
        };

        async_graphql::Error::new(summary.to_string()).extend_with(|_, e| match self {
            Error::Client(err) => match err {
                ClientError::BadRequest(_, code) => extend_graphql_error(e, code, &detail),
                ClientError::NotFound(_, code) => extend_graphql_error(e, code, &detail),
                _ => (),
            },

            Error::Internal(err) => match err {
                InternalError::Unknown(_, code) => extend_graphql_error(e, code, &detail),
                _ => (),
            },
        })
    }
}

fn extend_graphql_error(
    e: &mut ErrorExtensionValues,
    code: &Option<ErrorCode>,
    detail: &Option<String>,
) {
    if let Some(_code) = code {
        e.set("code", _code.to_string());
    }
    if let Some(_detail) = &detail {
        e.set("detail", _detail.clone());
    }
}
