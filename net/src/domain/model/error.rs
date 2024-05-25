use thiserror::Error;

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
    #[error("Configuration is invalid. {detail:?}")]
    Invalid {
        detail: String,
        error_code: Option<String>,
    },
}

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("Cannot or will not process the request. {detail}")]
    Invalid {
        detail: String,
        error_code: Option<String>,
    },

    #[error("Cannot find the requested resource. {detail}")]
    NotFound {
        detail: String,
        error_code: Option<String>,
    },

    #[error(transparent)]
    Precondition(PreconditionError),
}

#[derive(Error, Debug)]
pub enum PreconditionError {
    #[error("Prerequisites are not met. {detail}")]
    Required {
        detail: String,
        error_code: Option<String>,
    },

    #[error("Prerequisites are met, but not correctly. {detail}")]
    Invalid {
        detail: String,
        error_code: Option<String>,
    },
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

    #[error(transparent)]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error("An unexpected internal error occurred. {detail}")]
    Unexpected {
        detail: String,
        error_code: Option<String>,
    },
}

impl Error {
    pub fn system_unexpected(detail: &str, error_code: Option<String>) -> Self {
        let err = SystemError::Unexpected {
            detail: detail.to_string(),
            error_code,
        };

        Error::System(err)
    }

    pub fn config_invalid(detail: &str, error_code: Option<String>) -> Self {
        let err = ConfigError::Invalid {
            detail: detail.to_string(),
            error_code,
        };

        Error::Config(err)
    }

    pub fn resource_not_found(detail: &str, error_code: Option<String>) -> Self {
        let err = ResourceError::NotFound {
            detail: detail.to_string(),
            error_code,
        };

        Error::Resource(err)
    }

    pub fn resource_invalid(detail: &str, error_code: Option<String>) -> Self {
        let err = ResourceError::Invalid {
            detail: detail.to_string(),
            error_code,
        };

        Error::Resource(err)
    }

    pub fn resource_precondition_invalid(detail: &str, error_code: Option<String>) -> Self {
        let err = PreconditionError::Invalid {
            detail: detail.to_string(),
            error_code,
        };

        Error::Resource(ResourceError::Precondition(err))
    }

    pub fn resource_precondition_required(detail: &str, error_code: Option<String>) -> Self {
        let err = PreconditionError::Required {
            detail: detail.to_string(),
            error_code,
        };

        Error::Resource(ResourceError::Precondition(err))
    }
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

this_transparent_error!(Error, System, SystemError::Database, sea_orm::DbErr);
this_transparent_error!(Error, System, SystemError::Anyhow, anyhow::Error);
this_transparent_error!(Error, System, SystemError::Notify, notify::Error);
this_transparent_error!(Error, System, SystemError::IO, std::io::Error);
this_transparent_error!(Error, System, SystemError::JWT, jsonwebtoken::errors::Error);
