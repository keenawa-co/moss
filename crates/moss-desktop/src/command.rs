use anyhow::Result;
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{cell::RefCell, sync::Arc};
use tauri::{AppHandle, Window};
use thiserror::Error;

use crate::state::AppState;

#[derive(Error, Debug)]
pub enum CommandContextError {
    #[error("Argument '{key}' is not found")]
    ArgNotFound { key: String },

    #[error("Failed to deserialize argument '{key}': {source}")]
    DeserializationError {
        key: String,
        #[source]
        source: serde_json::Error,
    },
}

impl From<CommandContextError> for String {
    fn from(err: CommandContextError) -> Self {
        err.to_string()
    }
}

pub struct CommandContext {
    pub app_handle: AppHandle,
    pub window: Window,

    args: RefCell<HashMap<String, Value>>,
}

impl CommandContext {
    pub fn new(app_handle: AppHandle, window: Window, args: HashMap<String, Value>) -> Self {
        Self {
            app_handle,
            window,
            args: RefCell::new(args),
        }
    }

    pub fn take_arg<T>(&self, key: &str) -> Result<T, CommandContextError>
    where
        T: DeserializeOwned,
    {
        let mut args = self.args.borrow_mut();
        let value = args.remove(key).ok_or(CommandContextError::ArgNotFound {
            key: key.to_string(),
        })?;

        serde_json::from_value(value).map_err(|e| CommandContextError::DeserializationError {
            key: key.to_string(),
            source: e,
        })
    }

    pub fn get_arg<T>(&self, key: &str) -> Result<T, CommandContextError>
    where
        T: DeserializeOwned,
    {
        let args = self.args.borrow();
        let value = args.get(key).ok_or(CommandContextError::ArgNotFound {
            key: key.to_string(),
        })?;

        serde_json::from_value(value.clone()).map_err(|e| {
            CommandContextError::DeserializationError {
                key: key.to_string(),
                source: e,
            }
        })
    }
}

pub type CommandHandler =
    Arc<dyn Fn(CommandContext, &AppState) -> Result<Value, String> + Send + Sync>;

#[derive(Debug)]
pub struct CommandDecl {
    pub name: ReadOnlyStr,
    pub callback: fn(CommandContext, &AppState) -> Result<serde_json::Value, String>,
}
