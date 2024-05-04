use std::fmt;

use async_graphql::{ErrorExtensionValues, ErrorExtensions};
use thiserror::Error;

use crate::domain::model::result::Result;

#[macro_export]
macro_rules! resource_not_found {
    ($msg:expr) => {{
        $crate::domain::model::error::Error::Resource(
            $crate::domain::model::error::ResourceError::NotFound(
                format!("{}:{}: {}", file!(), line!(), $msg),
                None
            )
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::model::error::Error::Resource(
            $crate::domain::model::error::ResourceError::NotFound(
                format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
            )
        )
    }};
}

#[macro_export]
macro_rules! resource_invalid {
    ($msg:expr) => {{
        $crate::domain::model::error::Error::Resource(
            $crate::domain::model::error::ResourceError::Invalid(
                format!("{}:{}: {}", file!(), line!(), $msg)
            )
        )
    }};

    ($fmt:expr, $($arg:expr),*) => {{
        $crate::domain::model::error::Error::Resource(
            $crate::domain::model::error::ResourceError::Invalid(
                format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
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

pub trait OptionExtension<T> {
    fn or_else_config_invalid(self, detail: &str) -> Result<T>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Resource(ResourceError),

    #[error(transparent)]
    System(SystemError),

    #[error(transparent)]
    Config(ConfigError),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration is invalid. {0}")]
    Invalid(String),
}

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Cannot find the requested resource. {0}")]
    NotFound(String),

    #[error("Cannot or will not process the request. {0}")]
    Invalid(String),
}

#[derive(Error, Debug)]
pub enum SystemError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Notify(#[from] notify::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("The origin server requires the request to be conditional. {0}")]
    Precondition(String),

    #[error("An unexpected internal error occurred. {0}")]
    Unexpected(String),
}

impl Error {
    const DELIMITER: &'static str = ".";

    pub fn decompose(&self) -> (String, Option<String>) {
        if let Some((summary, detail)) = self.to_string().split_once(Self::DELIMITER) {
            (summary.trim().to_string(), Some(detail.trim().to_string()))
        } else {
            (self.to_string().trim().to_string(), None)
        }
    }
}

transparent_error!(System, SystemError::Database, sea_orm::DbErr);
transparent_error!(System, SystemError::Anyhow, anyhow::Error);
transparent_error!(System, SystemError::Notify, notify::Error);
transparent_error!(System, SystemError::IO, std::io::Error);

impl<T> OptionExtension<T> for Option<T> {
    fn or_else_config_invalid(self, detail: &str) -> Result<T> {
        let err = ConfigError::Invalid(detail.to_string());
        self.ok_or_else(|| Error::Config(err))
    }
}
