use hashbrown::HashMap;

use crate::util::ReadOnlyStr;

pub type MenuId = ReadOnlyStr;

#[rustfmt::skip]
lazy_static! {
    static ref MENU_NAMESPACE_ID_VIEW_TITLE: ReadOnlyStr = ReadOnlyStr::new("viewTitle");
    static ref MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT: ReadOnlyStr = ReadOnlyStr::new("viewTitleContext");
    static ref MENU_NAMESPACE_ID_VIEW_ITEM: ReadOnlyStr = ReadOnlyStr::new("viewItem");
    static ref MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT: ReadOnlyStr = ReadOnlyStr::new("viewItemContext");
}

#[derive(Debug)]
pub enum BuiltInMenuNamespaces {
    ViewTitle,
    ViewTitleContext,
    ViewItem,
    ViewItemContext,
}

#[rustfmt::skip]
impl From<BuiltInMenuNamespaces> for ReadOnlyStr {
    fn from(value: BuiltInMenuNamespaces) -> Self {
        use BuiltInMenuNamespaces as Namespace;

        match value {
            Namespace::ViewTitle => MENU_NAMESPACE_ID_VIEW_TITLE.clone(),
            Namespace::ViewTitleContext => MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT.clone(),
            Namespace::ViewItem => MENU_NAMESPACE_ID_VIEW_ITEM.clone(),
            Namespace::ViewItemContext => MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT.clone(),
        }
    }
}

#[rustfmt::skip]
impl ToString for BuiltInMenuNamespaces {
    fn to_string(&self) -> String {
        use BuiltInMenuNamespaces as Namespace;

        match &self {
            Namespace::ViewTitle => MENU_NAMESPACE_ID_VIEW_TITLE.to_string(),
            Namespace::ViewTitleContext => MENU_NAMESPACE_ID_VIEW_TITLE_CONTEXT.to_string(),
            Namespace::ViewItem => MENU_NAMESPACE_ID_VIEW_ITEM.to_string(),
            Namespace::ViewItemContext => MENU_NAMESPACE_ID_VIEW_ITEM_CONTEXT.to_string(),
        }
    }
}

#[rustfmt::skip]
lazy_static! {
    static ref MENU_GROUP_ID_THIS: ReadOnlyStr = ReadOnlyStr::new("this");
    static ref MENU_GROUP_ID_INLINE: ReadOnlyStr = ReadOnlyStr::new("inline");
    static ref MENU_GROUP_ID_NAVIGATION: ReadOnlyStr = ReadOnlyStr::new("navigation");
    static ref MENU_GROUP_ID_MODIFICATION: ReadOnlyStr = ReadOnlyStr::new("modification");
    static ref MENU_GROUP_ID_HELP: ReadOnlyStr = ReadOnlyStr::new("help");
    static ref MENU_GROUP_ID_PREVIEW: ReadOnlyStr = ReadOnlyStr::new("preview");
    static ref MENU_GROUP_ID_VIEWS: ReadOnlyStr = ReadOnlyStr::new("views");
    static ref MENU_GROUP_ID_REMOVE: ReadOnlyStr = ReadOnlyStr::new("remove");
}

#[derive(Debug)]
pub enum BuiltInMenuGroups {
    This,
    Inline,
    Navigation,
    Modification,
    Help,
    Preview,
    Views,
    Remove,
}

#[rustfmt::skip]
impl From<BuiltInMenuGroups> for ReadOnlyStr {
    fn from(value: BuiltInMenuGroups) -> Self {
        use BuiltInMenuGroups as Group;

        match value {
            Group::This => MENU_GROUP_ID_THIS.clone(),
            Group::Inline => MENU_GROUP_ID_INLINE.clone(),
            Group::Navigation => MENU_GROUP_ID_NAVIGATION.clone(),
            Group::Modification => MENU_GROUP_ID_MODIFICATION.clone(),
            Group::Help => MENU_GROUP_ID_HELP.clone(),
            Group::Preview => MENU_GROUP_ID_PREVIEW.clone(),
            Group::Views => MENU_GROUP_ID_VIEWS.clone(),
            Group::Remove => MENU_GROUP_ID_REMOVE.clone(),
        }
    }
}

#[rustfmt::skip]
impl ToString for BuiltInMenuGroups {
    fn to_string(&self) -> String {
        use BuiltInMenuGroups as Group;

        match &self {
            Group::This => MENU_GROUP_ID_THIS.to_string(),
            Group::Inline => MENU_GROUP_ID_INLINE.to_string(),
            Group::Navigation => MENU_GROUP_ID_NAVIGATION.to_string(),
            Group::Modification => MENU_GROUP_ID_MODIFICATION.to_string(),
            Group::Help => MENU_GROUP_ID_HELP.to_string(),
            Group::Preview => MENU_GROUP_ID_PREVIEW.to_string(),
            Group::Views => MENU_GROUP_ID_VIEWS.to_string(),
            Group::Remove => MENU_GROUP_ID_REMOVE.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum MenuItem {
    Action(ActionMenuItem),
    Submenu(SubmenuMenuItem),
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuGroup {
    order: Option<i64>,
    name: ReadOnlyStr,
}

impl MenuGroup {
    pub fn new_ordered(order: i64, name: impl Into<ReadOnlyStr>) -> Self {
        Self {
            order: Some(order),
            name: name.into(),
        }
    }

    pub fn new_unordered(name: impl Into<ReadOnlyStr>) -> Self {
        Self {
            order: None,
            name: name.into(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CommandAction {
    pub id: MenuId,
    pub title: String,
    pub tooltip: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ActionMenuItem {
    pub command: CommandAction,
    pub group: Option<MenuGroup>,
    pub order: Option<i64>,
    pub when: Option<ReadOnlyStr>,
    pub toggled: Option<ReadOnlyStr>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SubmenuMenuItem {
    pub submenu_id: MenuId,
    pub title: String,
    pub group: Option<MenuGroup>,
    pub order: Option<i64>,
    pub when: Option<ReadOnlyStr>,
}

pub struct MenuRegistry {
    menus: HashMap<ReadOnlyStr, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: ReadOnlyStr, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (ReadOnlyStr, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items(&self, menu_id: &ReadOnlyStr) -> Option<&Vec<MenuItem>> {
        self.menus.get(menu_id)
    }
}
