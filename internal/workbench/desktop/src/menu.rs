use hashbrown::HashMap;
use parking_lot::RwLock;
use static_str_ops::destaticize;
use std::sync::Arc;
use strum::Display;

use crate::util::ReadOnlyId;

#[derive(Debug, Display)]
pub enum Menus {
    #[strum(to_string = "ViewTitleContext")]
    ViewTitleContext,
    #[strum(to_string = "ViewTitle")]
    ViewTitle,
    #[strum(to_string = "ViewItemContext")]
    ViewItemContext,
    #[strum(to_string = "ViewItem")]
    ViewItem,
}

impl From<Menus> for ReadOnlyId {
    fn from(value: Menus) -> Self {
        ReadOnlyId::from(value.to_string())
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
    pub when: &'static str,
    pub toggled: Option<&'static str>,
}

impl Drop for ActionMenuItem {
    fn drop(&mut self) {
        destaticize(self.when);
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SubmenuMenuItem {
    pub submenu_id: ReadOnlyId,
    pub title: String,
    pub group: Option<String>,
    pub order: Option<i64>,
    pub when: &'static str,
}

impl Drop for SubmenuMenuItem {
    fn drop(&mut self) {
        destaticize(self.when);
    }
}

pub struct MenuRegistry {
    menus: HashMap<ReadOnlyId, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: ReadOnlyId, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (ReadOnlyId, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items(&self, menu_id: &ReadOnlyId) -> Option<&Vec<MenuItem>> {
        self.menus.get(menu_id)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MenuGroup {
    order: Option<i64>,
    content: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Menu(HashMap<ReadOnlyId, Vec<MenuGroup>>);

impl Menu {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

pub struct MenuService {
    registry: Arc<RwLock<MenuRegistry>>,
}

impl MenuService {
    pub fn new(registry: Arc<RwLock<MenuRegistry>>) -> Self {
        Self { registry }
    }

    pub fn create_menu_by_menu_id(
        &self,
        id: impl AsRef<ReadOnlyId>,
        f: impl FnOnce(&Vec<MenuItem>) -> Menu,
    ) -> Option<Menu> {
        let registry_lock = self.registry.read();
        let items = registry_lock.menus.get(id.as_ref())?;

        Some(f(items))
    }
}
