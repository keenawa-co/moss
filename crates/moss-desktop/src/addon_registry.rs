use anyhow::Result;
use async_trait::async_trait;
use fnv::FnvHashMap;
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use std::fmt::Debug;
use std::{
    any::{Any, TypeId},
    path::PathBuf,
    sync::Arc,
};
use tauri::{AppHandle, Listener};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Eq, Hash, PartialEq)]
pub enum ActivationEvent {
    OnStartUp,
    OnCommand(String),
    OnLanguage(String),
}

pub enum AddonType {
    BuiltIn,
    Installed,
}

pub struct AddonDescription {
    pub id: AddonId,
    pub name: String,
    pub ty: AddonType,
    pub version: Option<String>,
    pub source: PathBuf,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct AddonId(ReadOnlyStr);

pub struct AddonRegistry {
    addons: RwLock<HashMap<AddonId, AddonDescription>>,
    activation_queue: RwLock<FnvHashMap<ActivationEvent, Vec<AddonId>>>,
}

impl AddonRegistry {
    pub fn new() -> Self {
        Self {
            addons: RwLock::new(HashMap::new()),
            activation_queue: RwLock::new(FnvHashMap::default()),
        }
    }

    pub fn register(&self, addon: AddonDescription) {
        let mut addons_lock = self.addons.write();
        addons_lock.insert(addon.id.clone(), addon);
    }
}
