use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "components/primitive.ts")]
pub struct Link {
    pub title: Option<&'static str>,
    pub href: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "components/primitive.ts")]
pub struct Tooltip {
    pub header: &'static str,
    pub text: Option<&'static str>,
    pub shortcut: Option<&'static str>,
    pub link: Option<Link>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "components/primitive.ts")]
pub struct Icon {
    pub name: &'static str,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "components/primitive.ts")]
pub struct Button {
    pub text: &'static str,
}
