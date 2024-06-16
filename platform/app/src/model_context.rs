use anyhow::{Context as AnyhowContext, Result};
use derive_more::{Deref, DerefMut};
use futures::Future;
use parking_lot::RwLock;
use slotmap::{KeyData, SlotMap};
use smol::future::FutureExt;
use std::{
    any::TypeId,
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    num::NonZeroU64,
    pin::Pin,
    rc::Rc,
    sync::{atomic::AtomicUsize, Arc, Weak},
    task::Poll,
};

use tokio::{runtime::Runtime, sync::Notify};

use crate::{
    context::AppContext,
    executor::{Task, BackgroundTaskExecutor},
    platform::{current_platform, Platform},
};

slotmap::new_key_type! {
    /// A unique identifier for a model or view across the application.
    pub struct EntityId;
}

impl From<u64> for EntityId {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl EntityId {
    /// Converts this entity id to a [NonZeroU64]
    pub fn as_non_zero_u64(self) -> NonZeroU64 {
        NonZeroU64::new(self.0.as_ffi()).unwrap()
    }

    /// Converts this entity id to a [u64]
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

struct EntityRefCounts {
    counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
}

#[derive(Clone)]
pub struct AnyWeakModel {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

#[derive(Deref, DerefMut)]
pub struct WeakModel<T> {
    #[deref]
    #[deref_mut]
    any_model: AnyWeakModel,
    entity_type: PhantomData<T>,
}

unsafe impl<T> Send for WeakModel<T> {}
unsafe impl<T> Sync for WeakModel<T> {}

impl<T> Clone for WeakModel<T> {
    fn clone(&self) -> Self {
        Self {
            any_model: self.any_model.clone(),
            entity_type: self.entity_type,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct ModelContext<'a, T> {
    #[deref]
    #[deref_mut]
    app: &'a mut AppContext,
    model_state: WeakModel<T>,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn new(app: &'a mut AppContext, model_state: WeakModel<T>) -> Self {
        Self { app, model_state }
    }

    pub fn entity_id(&self) -> EntityId {
        self.model_state.entity_id
    }
}
