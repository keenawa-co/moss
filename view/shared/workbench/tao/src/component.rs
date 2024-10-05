use std::any::TypeId;
use std::borrow::Cow;
use std::marker::PhantomData;

use hashbrown::{HashMap, HashSet};

use slotmap::new_key_type;
use slotmap::{SecondaryMap, SlotMap};

new_key_type! {
    pub struct ViewKey;
}

pub trait AnyView {}

pub struct PropertyStore {}

pub enum ActivitySpot {
    Left,
    Right,
}

pub struct Activity {
    name: Cow<'static, str>,
    spot: ActivitySpot,
    order: usize,
}

pub struct Part {
    name: Cow<'static, str>,
    typ: TypeId,
}

impl Part {
    pub fn new<T: 'static>(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            typ: TypeId::of::<T>(),
        }
    }
}

pub type PartId = usize;

pub struct PartSet {
    parts: HashMap<PartId, Part>,
}

pub struct View {}

pub struct Component {}

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub struct ActivityBar {
    orientation: Orientation,
    icon_size: f32,
    content: HashSet<SideBarViewSet>,
}

pub struct SideBarViewSet {
    icon: String,
    title: String,
    tool_list: Vec<String>,
    content: HashSet<SideBarView>,
}

pub struct SideBarView {
    toolbar: Vec<String>,
}

pub struct ActivityBarItem {
    // id
    // title
    // order
    // container: View
}
