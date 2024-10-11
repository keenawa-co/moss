use std::any::Any;

use moss_uikit::{
    layout::{Order, Visibility},
    primitive::{Icon, Tooltip},
};
use serde::Serialize;
use specta::Type;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
pub struct ContextMenuCell {
    pub icon: Option<Icon>,
    pub text: &'static str,
    pub shortcut: Option<&'static str>,
    pub nested: Box<ContextMenu>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
pub struct ContextMenu {
    content: ContextMenuCell,
}

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// pub struct ToolBarCell {
//     pub title: String,
//     pub tooltip: Tooltip,
//     pub order: Order,
//     pub icon: Icon,
//     pub visibility: Visibility,
// }

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// pub struct ToolBarProjectCell {
//     name: String,
//     badge: String,
//     context_menu: ContextMenu,
// }

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
pub struct ActivityCell {
    pub id: String,
    pub title: Option<String>,
    pub tooltip: Option<Tooltip>,
    pub order: Option<Order>,
    pub icon: Option<Icon>,
    pub visibility: Visibility,
    pub shortcut: Option<&'static str>,
    pub nested: Option<ContextMenu>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
pub struct ActivityCellGroup {
    id: String,
    content: ActivityCell,
}

// #[ts(export, export_to = "toolbar.ts")]
pub trait AnyActivityCell {}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[ts(rename = "DescribeToolBarOutput")]
// #[serde(rename_all = "camelCase")]
pub struct DescribeToolBarOutput<T> {
    left_side: Vec<T>,
}

#[derive(Type)]
pub struct GenericType<A> {
    pub my_field: String,
    pub generic: A,
}

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "toolbar.ts")]
// pub struct DescribeToolBarOutput<T>
// where
//     T: AnyActivityCell + TS,
// {
//     left_side: Vec<T>,
// }

// fn te() {
//     let r = DescribeToolBarOutput {
//         left_side: vec![Re {
//             ne: "sff".to_string(),
//         }],
//     };
// }
