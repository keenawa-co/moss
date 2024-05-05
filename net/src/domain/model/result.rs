use crate::domain::model::error::Error;

pub type Result<T, E = super::error::Error> = core::result::Result<T, E>;

pub trait ResultExtension<T, E: std::fmt::Display> {
    fn ok_or_system_unexpected(self, detail: &str, error_code: Option<String>) -> Result<T, Error>;
}

impl<T, E: std::fmt::Display> ResultExtension<T, E> for Result<T, E> {
    fn ok_or_system_unexpected(self, detail: &str, error_code: Option<String>) -> Result<T, Error> {
        self.map_err(|e| Error::system_unexpected(&format!("{detail}: {e}"), error_code))
    }
}
