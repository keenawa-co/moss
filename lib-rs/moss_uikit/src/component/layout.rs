use serde::Serialize;
use specta::Type;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub value: usize,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub enum Alignment {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Visible,
    Invisible,
    Collapse,
}
