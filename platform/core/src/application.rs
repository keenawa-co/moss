use std::rc::Rc;

use crate::platform::AnyPlatform;

pub trait AnyApp {
    fn run(&self);
    fn quit(&self);

    fn platform(&self) -> Rc<dyn AnyPlatform>;
}
