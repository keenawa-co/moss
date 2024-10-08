use derive_more::{Deref, DerefMut};

// new_key_type! {
//     pub struct ViewKey;
// }

// pub trait AnyView {}

// pub struct PropertyStore {}

// pub enum ActivitySpot {
//     Left,
//     Right,
// }

// pub struct Activity {
//     pub title: String,
//     pub icon: String,
//     pub spot: ActivitySpot,
//     pub order: Option<usize>,
// }

// pub struct ActivityView<T> {
//     pub title: String,
//     pub tools: Vec<String>,
//     pub content: T,
// }

// pub struct Column {
//     data: String,
// }

// pub struct PropertyId(Cow<'static, str>);

// pub struct Table {
//     columns: HashMap<PropertyId, Column>,
//     components: Vec<String>,
// }

// pub struct CompositeComponentSet {
//     table: Table,
// }

// pub struct Part {
//     name: Cow<'static, str>,
//     typ: TypeId,
// }

// impl Part {
//     pub fn new<T: 'static>(name: impl Into<Cow<'static, str>>) -> Self {
//         Self {
//             name: name.into(),
//             typ: TypeId::of::<T>(),
//         }
//     }
// }

// pub type PartId = usize;

// pub struct PartSet {
//     parts: HashMap<PartId, Part>,
// }

// pub struct View {}

// pub struct Component {}

// pub enum Orientation {
//     Vertical,
//     Horizontal,
// }

// pub struct ActivityBar {
//     orientation: Orientation,
//     icon_size: f32,
//     content: HashSet<SideBarViewSet>,
// }

// pub struct SideBarViewSet {
//     icon: String,
//     title: String,
//     tool_list: Vec<String>,
//     content: HashSet<SideBarView>,
// }

// pub struct SideBarView {
//     toolbar: Vec<String>,
// }

// pub struct ActivityBarItem {
//     // id
//     // title
//     // order
//     // container: View
// }

#[derive(Deref, DerefMut, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Tooltip(pub &'static str);

#[derive(Deref, DerefMut, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Order(pub usize);
