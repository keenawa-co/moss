
use derive_more::{Deref, DerefMut};
use futures::Future;


use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};
use tokio::runtime::Runtime;

use crate::{
    context_model::{EntryMap},
    executor::TaskCompact,
};

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
    pub(crate) this: Weak<AppCellCompact>,
    pub(crate) runtime: Rc<Runtime>,
    pub(crate) entry_map: Rc<RefCell<EntryMap>>,
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
                entry_map: Rc::new(RefCell::new(EntryMap::new())),
            }),
        })
    }

    // pub fn insert_model<T: 'static>(&mut self, build_fn: impl FnOnce() -> T) -> Model<T> {
    //     let mut entry_map = self.entry_map.borrow_mut();
    //     let slot = entry_map.reserve::<T>();
    //     entry_map.insert(slot, build_fn())
    // }

    // pub fn read_model<T: 'static>(&self) {
    //     self.entry_map.borrow().read(model)
    // }

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
