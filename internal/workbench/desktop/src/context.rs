use anyhow::Result;
use dashmap::DashMap;
use moss_text::ReadOnlyStr;
use std::{any::Any, sync::Arc};
use tauri::AppHandle;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Key '{0}' not found in the context.")]
    ValueKeyNotFound(String),

    #[error("Type mismatch for key '{0}' in the context.")]
    ValueTypeMismatch(String),
}

pub struct Context {
    pub app_handle: AppHandle,
    pub signals: DashMap<ReadOnlyStr, Arc<dyn Fn(&Context) -> Result<()>>>,
    values: DashMap<String, Box<dyn Any>>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            signals: DashMap::new(),
            values: DashMap::new(),
        }
    }

    pub fn insert_value<T: 'static + Send + Sync>(&self, key: String, value: T) {
        self.values.insert(key, Box::new(value));
    }

    pub fn take_value<T: 'static + Send + Sync>(&self, key: &str) -> Result<T, ContextError> {
        let (_, boxed) = self
            .values
            .remove(key)
            .ok_or_else(|| ContextError::ValueKeyNotFound(key.to_string()))?;

        match boxed.downcast::<T>() {
            Ok(b) => Ok(*b),
            Err(_) => Err(ContextError::ValueTypeMismatch(key.to_string())),
        }
    }
}

pub fn handle_change_theme(ctx: &Context) -> anyhow::Result<()> {
    println!("Hello from signal handler!");
    Ok(())
}
