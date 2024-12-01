use moss_html::link::HtmlLink;
use moss_str::localized_string::LocalizedString;

#[derive(Serialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct Tooltip {
    #[ts(type = "LocalizedString")]
    pub header: LocalizedString,

    #[ts(optional, type = "LocalizedString")]
    pub text: Option<LocalizedString>,

    #[ts(optional)]
    pub shortcut: Option<&'static str>,

    #[ts(optional)]
    pub link: Option<HtmlLink>,
}

#[derive(Serialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub struct Icon {
    pub name: &'static str,
}
