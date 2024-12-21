use dashmap::DashMap;
use moss_text::ReadOnlyStr;

use crate::models::actions::MenuItem;

pub struct Menus {
    state: DashMap<ReadOnlyStr, Vec<MenuItem>>,
}

impl Menus {
    // TODO: rename to insert
    pub fn append_menu_item(&mut self, key: ReadOnlyStr, item: MenuItem) {
        self.state.entry(key).or_insert_with(Vec::new).push(item);
    }
}
