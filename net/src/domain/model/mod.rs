pub mod error;
pub mod notification;
pub mod project;
pub mod result;
pub mod session;

use crate::domain::model::{error::*, result::Result};

pub trait OptionExtension<T> {
    fn ok_or_config_invalid(self, detail: &str, error_code: Option<String>) -> Result<T>;
    fn ok_or_resource_invalid(self, detail: &str, error_code: Option<String>) -> Result<T>;
    fn ok_or_resource_not_found(self, detail: &str, error_code: Option<String>) -> Result<T>;
    fn ok_or_resource_precondition_invalid(
        self,
        detail: &str,
        error_code: Option<String>,
    ) -> Result<T>;

    fn ok_or_resource_precondition_required(
        self,
        detail: &str,
        error_code: Option<String>,
    ) -> Result<T>;
}

impl<T> OptionExtension<T> for Option<T> {
    fn ok_or_config_invalid(self, detail: &str, error_code: Option<String>) -> Result<T> {
        self.ok_or_else(|| Error::config_invalid(detail, error_code))
    }

    fn ok_or_resource_invalid(self, detail: &str, error_code: Option<String>) -> Result<T> {
        self.ok_or_else(|| Error::resource_invalid(detail, error_code))
    }

    fn ok_or_resource_not_found(self, detail: &str, error_code: Option<String>) -> Result<T> {
        self.ok_or_else(|| Error::resource_not_found(detail, error_code))
    }

    fn ok_or_resource_precondition_invalid(
        self,
        detail: &str,
        error_code: Option<String>,
    ) -> Result<T> {
        self.ok_or_else(|| Error::resource_precondition_invalid(detail, error_code))
    }

    fn ok_or_resource_precondition_required(
        self,
        detail: &str,
        error_code: Option<String>,
    ) -> Result<T> {
        self.ok_or_else(|| Error::resource_precondition_required(detail, error_code))
    }
}
