use hashbrown::HashMap;
use once_cell::sync::Lazy;
use std::{any::Any, fmt::Debug, sync::Arc};

use moss_str::{localized_string::LocalizedString, read_only_str, ReadOnlyStr};

pub type GroupId = ReadOnlyStr;

#[rustfmt::skip]
lazy_static! {
    static ref VIEW_GROUP_ID_LAUNCHPAD: ReadOnlyStr = read_only_str!("workbench.group.launchpad");
}

#[derive(Debug)]
pub enum BuiltInViewGroups {
    Launchpad,
}

impl From<BuiltInViewGroups> for ReadOnlyStr {
    fn from(value: BuiltInViewGroups) -> Self {
        use BuiltInViewGroups as Group;

        match value {
            Group::Launchpad => VIEW_GROUP_ID_LAUNCHPAD.clone(),
        }
    }
}

impl ToString for BuiltInViewGroups {
    fn to_string(&self) -> String {
        use BuiltInViewGroups as Group;

        match &self {
            Group::Launchpad => VIEW_GROUP_ID_LAUNCHPAD.to_string(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct TreeViewGroup {
    pub id: ReadOnlyStr,
    pub name: LocalizedString,
    pub order: usize,
}

#[derive(Debug)]
pub struct TreeViewDescriptor {
    pub id: String,
    pub name: LocalizedString,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
    pub collapsed: bool,
    pub model: Lazy<Arc<dyn Any + Send + Sync>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TreeViewOutput {
    pub id: String,
    pub name: LocalizedString,
    pub order: usize,
    pub hide_by_default: bool,
    pub can_toggle_visibility: bool,
    pub collapsed: bool,
}

impl From<&TreeViewDescriptor> for TreeViewOutput {
    fn from(value: &TreeViewDescriptor) -> Self {
        TreeViewOutput {
            id: value.id.clone(),
            name: value.name.clone(),
            order: value.order,
            hide_by_default: value.hide_by_default,
            can_toggle_visibility: value.can_toggle_visibility,
            collapsed: value.collapsed,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum TreeViewGroupLocation {
    PrimaryBar,
    SecondaryBar,
}
