use std::rc::Rc;

use moss_str::{localized_string::LocalizedString, ReadOnlyStr};
use serde::Serialize;
use ts_rs::TS;

pub type SubmenuRef = moss_str::ReadOnlyStr;
pub type ActionCommandId = ReadOnlyStr;

// #[derive(Debug, Clone, Serialize)]
// pub enum MenuItem {
//     Action(ActionMenuItem),
//     Submenu(SubmenuMenuItem),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
pub enum MenuItemVisibility {
    #[serde(rename = "classic")]
    #[default]
    Classic,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "compact")]
    Compact,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "actions.ts")]
pub struct MenuGroup {
    #[ts(type = "string")]
    id: ReadOnlyStr,
    order: Option<i64>,
    #[ts(type = "LocalizedString | null")]
    description: Option<LocalizedString>,
}

// impl MenuGroup {
//     pub fn new_ordered(order: i64, id: impl Into<ReadOnlyStr>) -> Self {
//         Self {
//             id: id.into(),
//             order: Some(order),
//             description: None,
//         }
//     }

//     pub fn new_unordered(id: impl Into<ReadOnlyStr>) -> Self {
//         Self {
//             id: id.into(),
//             order: None,
//             description: None,
//         }
//     }
// }

// #[derive(Debug, Serialize, Clone)]
// pub struct CommandAction {
//     pub id: ActionCommandId,
//     pub title: LocalizedString,
//     pub tooltip: Option<String>,
//     pub description: Option<LocalizedString>,
//     pub icon: Option<String>,
//     pub toggled: Option<CommandActionToggle>,
// }

// #[derive(Debug, Serialize, Clone)]
// pub struct CommandActionToggle {
//     pub condition: ReadOnlyStr,
//     pub icon: Option<String>,
//     pub tooltip: Option<String>,
//     pub title: Option<LocalizedString>,
// }

// #[derive(Debug, Serialize, Clone)]
// pub struct ActionMenuItem {
//     pub command: CommandAction,
//     pub group: Option<Rc<MenuGroup>>,
//     pub order: Option<i64>,
//     pub when: Option<ReadOnlyStr>,
//     pub visibility: MenuItemVisibility,
// }

// #[derive(Debug, Serialize, Clone)]
// pub struct SubmenuMenuItem {
//     pub submenu_id: ActionCommandId,
//     pub default_action_id: Option<ActionCommandId>,
//     pub title: Option<LocalizedString>,
//     pub group: Option<Rc<MenuGroup>>,
//     pub order: Option<i64>,
//     pub when: Option<ReadOnlyStr>,
//     pub visibility: MenuItemVisibility,
// }
