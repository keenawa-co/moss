use std::sync::Arc;

use moss_jsonlogic::raw_rule::RawRule;
use moss_text::{localized_string::LocalizedString, ReadOnlyStr};
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[ts(export, export_to = "actions.ts")]
pub struct Action(#[ts(type = "string")] ReadOnlyStr);

pub type SubmenuRef = moss_text::ReadOnlyStr;
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

    #[ts(optional)]
    order: Option<i64>,

    #[ts(optional, type = "LocalizedString")]
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

    #[ts(type = "LocalizedString")]
    pub title: LocalizedString,

    #[ts(optional)]
    pub tooltip: Option<String>,

    #[ts(optional, type = "LocalizedString")]
    pub description: Option<LocalizedString>,

    #[ts(optional)]
    pub icon: Option<String>,

    #[ts(optional)]
    pub toggled: Option<CommandActionToggle>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct CommandActionToggle {
    #[ts(type = "string")]
    pub condition: RawRule,

    #[ts(optional)]
    pub icon: Option<String>,

    #[ts(optional)]
    pub tooltip: Option<String>,

    #[ts(optional, type = "LocalizedString")]
    pub title: Option<LocalizedString>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct ActionMenuItem {
    pub command: CommandAction,

    #[ts(optional)]
    pub group: Option<Arc<MenuGroup>>,

    #[ts(optional)]
    pub order: Option<i64>,

    #[ts(optional, type = "object")]
    pub when: Option<RawRule>,

    pub visibility: MenuItemVisibility,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "actions.ts")]
pub struct SubmenuMenuItem {
    #[ts(type = "string")]
    pub submenu_id: ActionCommandId,

    #[ts(optional, type = "string")]
    pub default_action_id: Option<ActionCommandId>,

    #[ts(optional, type = "LocalizedString")]
    pub title: Option<LocalizedString>,

    #[ts(optional)]
    pub group: Option<Arc<MenuGroup>>,

    #[ts(optional)]
    pub order: Option<i64>,

    #[ts(optional, type = "string")]
    pub when: Option<RawRule>,

    pub visibility: MenuItemVisibility,
}
