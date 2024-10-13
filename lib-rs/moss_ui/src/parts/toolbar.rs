use moss_uikit::component::{
    layout::{Order, Visibility},
    primitive::{Icon, Tooltip},
};
use serde::Serialize;
use specta::Type;
use typeshare::typeshare;

use super::common::ContextMenu;

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct ToolBarProjectCell {
    pub menu: ContextMenu,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCell {
    pub title: Option<&'static str>,
    pub tooltip: Option<Tooltip>,
    pub order: Option<Order>,
    pub icon: Option<Icon>,
    pub visibility: Visibility,
    pub nested: Option<ContextMenu>,
}
#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct ToolBarLeftSide {
    pub project: ToolBarProjectCell,
    pub activities: Vec<ActivityCell>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct DescribeToolBarOutput {
    pub left_side: ToolBarLeftSide,
}
