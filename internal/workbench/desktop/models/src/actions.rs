use std::rc::Rc;

use moss_str::{localized_string::LocalizedString, ReadOnlyStr};
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[ts(export, export_to = "actions.ts")]
pub struct Action(#[ts(type = "string")] ReadOnlyStr);

pub type SubmenuRef = moss_str::ReadOnlyStr;
pub type ActionCommandId = ReadOnlyStr;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub enum MenuItem {
    Action(ActionMenuItem),
    Submenu(SubmenuMenuItem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub enum MenuItemVisibility {
    #[default]
    Classic,
    Hidden,
    Compact,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct MenuGroup {
    #[ts(type = "string")]
    id: ReadOnlyStr,
    order: Option<i64>,
    #[ts(type = "LocalizedString | null")]
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

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct CommandAction {
    #[ts(type = "string")]
    pub id: ActionCommandId,
    #[ts(type = "LocalizedString | null")]
    pub title: LocalizedString,
    pub tooltip: Option<String>,
    #[ts(type = "LocalizedString | null")]
    pub description: Option<LocalizedString>,
    pub icon: Option<String>,
    pub toggled: Option<CommandActionToggle>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct CommandActionToggle {
    #[ts(type = "string")]
    pub condition: ReadOnlyStr,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    #[ts(type = "LocalizedString | null")]
    pub title: Option<LocalizedString>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct ActionMenuItem {
    pub command: CommandAction,
    pub group: Option<Rc<MenuGroup>>,
    pub order: Option<i64>,
    #[ts(type = "string | null")]
    pub when: Option<ReadOnlyStr>,
    pub visibility: MenuItemVisibility,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct SubmenuMenuItem {
    #[ts(type = "string")]
    pub submenu_id: ActionCommandId,
    #[ts(type = "string | null")]
    pub default_action_id: Option<ActionCommandId>,
    #[ts(type = "LocalizedString | null")]
    pub title: Option<LocalizedString>,
    pub group: Option<Rc<MenuGroup>>,
    pub order: Option<i64>,
    #[ts(type = "string | null")]
    pub when: Option<ReadOnlyStr>,
    pub visibility: MenuItemVisibility,
}
