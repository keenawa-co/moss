use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "window.ts")]
pub struct LocaleDescriptor {
    pub code: String,
    pub name: String,
    #[ts(optional)]
    pub direction: Option<String>, // "ltr" or "rtl"
}
