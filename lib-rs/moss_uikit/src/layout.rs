use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "components/layout.ts")]
pub struct Order {
    pub value: usize,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export, export_to = "components/layout.ts")]
pub enum Alignment {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export, export_to = "components/layout.ts")]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}
