use serde::Serialize;

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/common.ts")]
// pub struct ContextMenuCell {
//     pub icon: Option<Icon>,
//     pub text: &'static str,
//     pub shortcut: Option<&'static str>,
//     pub nested: Box<ContextMenu>,
// }

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/common.ts")]
// pub struct ContextMenu {
//     content: ContextMenuCell,
// }

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/toolbar.ts")]
// pub struct ToolBarProjectCell {
//     name: String,
//     context_menu: ContextMenu,
// }

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/toolbar.ts")]
// pub struct ActivityCell {
//     pub title: Option<String>,
//     pub tooltip: Option<Tooltip>,
//     // pub order: Option<Order>,
//     // pub icon: Option<Icon>,
//     // pub visibility: Visibility,
//     pub nested: Option<ContextMenu>,
// }
// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/toolbar.ts")]
// pub struct ToolBarLeftSide {
//     project: ToolBarProjectCell,
//     activities: Vec<ActivityCell>,
// }

// #[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "parts/toolbar.ts")]
// pub struct DescribeToolBarOutput {
//     left_side: ToolBarLeftSide,
// }
