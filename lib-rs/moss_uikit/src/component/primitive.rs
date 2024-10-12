use serde::Serialize;
use specta::Type;

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub title: Option<&'static str>,
    pub href: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    pub header: &'static str,
    pub text: Option<&'static str>,
    pub shortcut: Option<&'static str>,
    pub link: Option<Link>,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub name: &'static str,
}

#[derive(Serialize, Debug, Clone, Default, Eq, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub text: &'static str,
}
