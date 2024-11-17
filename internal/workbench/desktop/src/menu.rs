use moss_str::{localized_string::LocalizedString, read_only_str, ReadOnlyStr};
use std::rc::Rc;

pub type ActionCommandId = ReadOnlyStr;

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

#[derive(Debug, Clone, Serialize)]
pub enum MenuItem {
    Action(ActionMenuItem),
    Submenu(SubmenuMenuItem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
pub enum MenuItemVisibility {
    #[serde(rename = "classic")]
    #[default]
    Classic,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "compact")]
    Compact,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuGroup {
    id: ReadOnlyStr,
    order: Option<i64>,
    description: Option<LocalizedString>,
}

impl MenuGroup {
    pub fn new_ordered(order: i64, id: impl Into<ReadOnlyStr>) -> Self {
        Self {
            id: id.into(),
            order: Some(order),
            description: None,
        }
    }

    pub fn new_unordered(id: impl Into<ReadOnlyStr>) -> Self {
        Self {
            id: id.into(),
            order: None,
            description: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CommandAction {
    pub id: ActionCommandId,
    pub title: LocalizedString,
    pub tooltip: Option<String>,
    pub description: Option<LocalizedString>,
    pub icon: Option<String>,
    pub toggled: Option<CommandActionToggle>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CommandActionToggle {
    pub condition: ReadOnlyStr,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    pub title: Option<LocalizedString>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ActionMenuItem {
    pub command: CommandAction,
    pub group: Option<Rc<MenuGroup>>,
    pub order: Option<i64>,
    pub when: Option<ReadOnlyStr>,
    pub visibility: MenuItemVisibility,
}

pub type SubmenuRef = moss_str::ReadOnlyStr;

#[derive(Debug, Serialize, Clone)]
pub struct SubmenuMenuItem {
    pub submenu_id: ActionCommandId,
    pub default_action_id: Option<ActionCommandId>,
    pub title: Option<LocalizedString>,
    pub group: Option<Rc<MenuGroup>>,
    pub order: Option<i64>,
    pub when: Option<ReadOnlyStr>,
    pub visibility: MenuItemVisibility,
}
