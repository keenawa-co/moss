pub mod manifest;

use std::{path::PathBuf, sync::LazyLock};

pub static BUILTIN_ADDONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("BUILTIN_ADDONS_DIR")
        .unwrap_or_else(|_| {
            "Environment variable `BUILTIN_ADDONS_DIR` is not set or is invalid".to_string()
        })
        .into()
});

pub static INSTALLED_ADDONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("INSTALLED_ADDONS_DIR")
        .unwrap_or_else(|_| {
            "Environment variable `INSTALLED_ADDONS_DIR` is not set or is invalid".to_string()
        })
        .into()
});
