pub mod context;
pub mod event;

mod executor;
mod platform;

use context::{AppCell, AppContext};
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
