pub mod context;
pub mod context_compact;
pub mod context_model;
pub mod event;

mod executor;
mod platform;

use context::{AppCell, AppContext};
use context_compact::{AppCellCompact, AppContextCompact};
use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

#[macro_use]
extern crate anyhow;

pub struct App(Rc<AppCell>);

impl App {
    pub fn new() -> Self {
        Self(AppContext::new(platform::current_platform()))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&AppContext),
    {
        let this = self.0.clone();
        // let platform = self.0.app.borrow().platform.clone();
        // platform.run(Box::new(move || {
        //     let ctx: &mut RefMut<AppContext> = &mut *this.borrow_mut();
        //     on_finish_launching(ctx);
        // }));

        let ctx: &Ref<AppContext> = &*this.borrow();
        // let c = ctx.to_owned();
        on_finish_launching(ctx);
    }
}

pub struct AppCompact(Rc<AppCellCompact>);

impl AppCompact {
    pub fn new() -> Self {
        Self(AppContextCompact::new())
    }

    pub fn run<F>(self, launching: F)
    where
        F: 'static + FnOnce(&mut AppContextCompact),
    {
        let this = self.0.clone();
        let ctx: &mut RefMut<AppContextCompact> = &mut *this.borrow_mut();

        launching(ctx);
    }
}
