use anyhow::Result;
use derive_more::{Deref, DerefMut};
use futures::Future;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};
use tokio::runtime::Runtime;

use crate::{context_model::EntryMap, executor::TaskCompact};

pub struct AppCellCompact {
    app: RefCell<AppContextCompact>,
}

#[derive(Deref, DerefMut)]
pub struct AppRef<'a>(Ref<'a, AppContextCompact>);

#[derive(Deref, DerefMut)]
pub struct AppRefMut<'a>(RefMut<'a, AppContextCompact>);

impl AppCellCompact {
    pub fn borrow(&self) -> AppRef {
        println!("borrowed");
        AppRef(self.app.borrow())
    }

    pub fn borrow_mut(&self) -> AppRefMut {
        println!("borrowed mut");
        AppRefMut(self.app.borrow_mut())
    }
}

#[derive(Clone)]
pub struct AppContextCompact {
    this: Weak<AppCellCompact>,
    runtime: Rc<Runtime>,
    // entry_map: EntryMap,
}

unsafe impl Sync for AppContextCompact {}
unsafe impl Send for AppContextCompact {}

impl AppContextCompact {
    pub fn new() -> Rc<AppCellCompact> {
        let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        else {
            panic!("failed to build async runtime");
        };

        Rc::new_cyclic(|this: &Weak<AppCellCompact>| AppCellCompact {
            app: RefCell::new(AppContextCompact {
                this: this.clone(),
                runtime: Rc::new(runtime),
            }),
        })
    }

    pub fn detach<'a, Fut, R>(
        &'a self,
        f: impl FnOnce(&'a AppContextCompact) -> Fut,
    ) -> TaskCompact<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        TaskCompact::Spawned(self.runtime.spawn(f(self)))
    }

    pub fn block_on<'a, Fut, R>(&'a self, f: impl FnOnce(&'a AppContextCompact) -> Fut) -> R
    where
        Fut: Future<Output = R>,
    {
        self.runtime.block_on(f(self))
    }
}
