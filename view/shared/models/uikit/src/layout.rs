#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "models.ts")]
pub enum Visibility {
    Visible,
    Invisible,
    Collapse,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "models.ts")]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "models.ts")]
pub enum Orientation {
    Vertical,
    Horizontal,
}
