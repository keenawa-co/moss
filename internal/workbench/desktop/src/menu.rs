use hashbrown::{HashMap, HashSet};
use std::collections::LinkedList;
use strum::Display;

#[derive(Debug, Display)]
pub enum Menus {
    #[strum(to_string = "RecentsContext")]
    RecentsContext,
    #[strum(to_string = "LinksContext")]
    LinksContext,
}

pub trait MenuItem {
    fn id(&self) -> &str;
    fn group(&self) -> &str;
}

pub struct CommandAction {
    id: String,
    title: String,
    tooltip: Option<String>,
    description: Option<String>,
}

pub struct ActionMenuItem {
    command: CommandAction,
    group: Option<String>,
    order: Option<i64>,
}

// impl MenuItem for ActionMenuItem {
//     fn id(&self) -> &str {
//         &self.command.id
//     }

//     fn group(&self) -> &Option<String> {
//       let r =  &self.group;
//     }
// }

pub struct SubmenuMenuItem {
    title: String,
    submenu_id: String,
    group: Option<String>,
    order: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuId(String);

pub struct MenuRegistry {
    menus: HashMap<MenuId, LinkedList<Box<dyn MenuItem>>>,
    groups: HashMap<String, HashSet<String>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn append_menu_item<T>(&mut self, menu_id: MenuId, item: T)
    where
        T: MenuItem + 'static,
    {
        self.groups
            .entry(item.group().to_string())
            .or_insert_with(HashSet::new)
            .insert(item.id().to_string());

        self.menus
            .entry(menu_id)
            .or_insert_with(LinkedList::new)
            .push_back(Box::new(item));
    }

    pub fn append_menu_items<I, T>(&mut self, items: I)
    where
        I: IntoIterator<Item = (MenuId, T)>,
        T: MenuItem + 'static,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu(&self, menu_id: &MenuId) -> Option<&LinkedList<Box<dyn MenuItem>>> {
        self.menus.get(menu_id)
    }
}
