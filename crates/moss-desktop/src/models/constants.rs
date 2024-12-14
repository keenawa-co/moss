use moss_text::{read_only_str, ReadOnlyStr};

#[rustfmt::skip]
pub mod menu {
    use super::*;

    pub const MENU_NAMESPACE_ID_VIEW_TITLE: ReadOnlyStr = read_only_str!("viewTitle");
    pub const MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT: ReadOnlyStr = read_only_str!("viewTitleContext");
    pub const MENU_NAMESPACE_ID_VIEW_ITEM: ReadOnlyStr = read_only_str!("viewItem");
    pub const MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT: ReadOnlyStr = read_only_str!("viewItemContext");
    pub const MENU_NAMESPACE_ID_HEAD_ITEM: ReadOnlyStr = read_only_str!("headItem");

    pub const MENU_GROUP_ID_THIS: ReadOnlyStr = read_only_str!("this");
    pub const MENU_GROUP_ID_INLINE: ReadOnlyStr = read_only_str!("inline");
    pub const MENU_GROUP_ID_NAVIGATION: ReadOnlyStr = read_only_str!("navigation");
    pub const MENU_GROUP_ID_MODIFICATION: ReadOnlyStr = read_only_str!("modification");
    pub const MENU_GROUP_ID_HELP: ReadOnlyStr = read_only_str!("help");
    pub const MENU_GROUP_ID_PREVIEW: ReadOnlyStr = read_only_str!("preview");
    pub const MENU_GROUP_ID_VIEWS: ReadOnlyStr = read_only_str!("views");
    pub const MENU_GROUP_ID_REMOVE: ReadOnlyStr = read_only_str!("remove");
}

#[rustfmt::skip]
pub mod view {
    use super::*;
    
    pub const VIEW_GROUP_ID_LAUNCHPAD: ReadOnlyStr = read_only_str!("workbench.group.launchpad");
}
