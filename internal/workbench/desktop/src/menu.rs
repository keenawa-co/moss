use hashbrown::HashMap;
use strum::Display;

#[derive(Debug, Display)]
pub enum Menus {
    #[strum(to_string = "ViewItemContext")]
    ViewItemContext,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct MenuId(String);

impl MenuId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for MenuId {
    fn from(s: &str) -> Self {
        MenuId(s.to_owned())
    }
}

impl From<String> for MenuId {
    fn from(s: String) -> Self {
        MenuId(s)
    }
}

impl Into<MenuId> for Menus {
    fn into(self) -> MenuId {
        MenuId::from(self.to_string())
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum MenuItem {
    Action(ActionMenuItem),
    Submenu(SubmenuMenuItem),
}

#[derive(Debug, Serialize, Clone)]
pub struct CommandAction {
    pub id: String,
    pub title: String,
    pub tooltip: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ActionMenuItem {
    pub command: CommandAction,
    pub group: Option<String>,
    pub order: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SubmenuMenuItem {
    pub submenu_id: MenuId,
    pub title: String,
    pub group: Option<String>,
    pub order: Option<i64>,
}

pub struct MenuRegistry {
    menus: HashMap<MenuId, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: MenuId, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (MenuId, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items(&self, menu_id: &MenuId) -> Option<&Vec<MenuItem>> {
        self.menus.get(menu_id)
    }
}
