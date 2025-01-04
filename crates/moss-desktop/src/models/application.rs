use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Hash, Debug, Clone, Eq, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct ThemeDescriptor {
    /// Unique identifier of the theme.
    pub id: String,
    /// Display name of the theme.
    pub name: String,
    /// Source of the theme (e.g., file path or URL).
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct LocaleDescriptor {
    /// The language code of the locale (e.g., "en", "fr").
    pub code: String,
    /// The display name of the locale (e.g., "English", "French").
    pub name: String,
    /// The text direction of the locale, if specified ("ltr" for left-to-right, "rtl" for right-to-left).
    #[ts(optional)]
    pub direction: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct Preferences {
    #[ts(optional)]
    pub theme: Option<ThemeDescriptor>,
    #[ts(optional)]
    pub locale: Option<LocaleDescriptor>,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct Defaults {
    pub theme: ThemeDescriptor,
    pub locale: LocaleDescriptor,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct AppState {
    /// The user preferences for the application.
    pub preferences: Preferences,
    pub defaults: Defaults,
}
