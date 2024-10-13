use moss_uikit::component::{
    accessibility::Action,
    layout::{Order, Visibility},
    primitive::{Icon, Tooltip},
};
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenuCell {
    pub action: Option<Action>,
    pub icon: Option<Icon>,
    pub text: &'static str,
    pub shortcut: Option<&'static str>,
    pub nested: Option<Box<ContextMenu>>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct ContextMenu {
    pub content: Vec<ContextMenuCell>,
}
