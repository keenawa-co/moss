use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const MANIFEST_FILENAME: &'static str = "Moss.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddonActivationEvents {
    OnStartUp,
    OnCommand(String),
    OnLanguage(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum AddonCategories {
    Themes,
    LanguagePacks,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeContribution {
    pub label: String,
    pub path: PathBuf,
}

// FIXME: Temporarily changed to match the LocaleDescriptor
// Until Localization addon pack is fully implemented
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationContribution {
    pub code: String,
    pub name: String,
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddonContributions {
    pub themes: Option<Vec<ThemeContribution>>,
    pub localizations: Option<Vec<LocalizationContribution>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddonInfo {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub categories: Option<Vec<AddonCategories>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddonManifest {
    pub addon: AddonInfo,
    pub contributes: AddonContributions,
    pub activation_events: Option<Vec<AddonActivationEvents>>,
}
