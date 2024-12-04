use anyhow::Result;
use desktop_models::appearance::theming::ThemeDescriptor;
use std::path::PathBuf;

#[tauri::command(async)]
pub async fn fetch_themes() -> Result<Vec<ThemeDescriptor>, String> {
    Ok(vec![
        ThemeDescriptor {
            id: "theme-light".to_string(),
            name: "Theme Light".to_string(),
            source: PathBuf::from("moss-light.css")
                .to_string_lossy()
                .to_string(),
        },
        ThemeDescriptor {
            id: "theme-dark".to_string(),
            name: "Theme Dark".to_string(),
            source: PathBuf::from("moss-dark.css").to_string_lossy().to_string(),
        },
        ThemeDescriptor {
            id: "theme-pink".to_string(),
            name: "Theme Pink".to_string(),
            source: PathBuf::from("moss-pink.css").to_string_lossy().to_string(),
        },
    ])
}

#[derive(Serialize, Deserialize)]
pub struct Locale {
    code: String,
    name: String,
    direction: Option<String>, // "ltr" or "rtl"
}

#[tauri::command]
pub fn get_locales() -> Vec<Locale> {
    vec![
        Locale {
            code: "en".to_string(),
            name: "English".to_string(),
            direction: Some("ltr".to_string()),
        },
        Locale {
            code: "de".to_string(),
            name: "Deutsche".to_string(),
            direction: Some("ltr".to_string()),
        },
        Locale {
            code: "ru".to_string(),
            name: "Русский".to_string(),
            direction: Some("ltr".to_string()),
        },
    ]
}

#[tauri::command]
pub fn get_translations(language: String, namespace: String) -> Result<serde_json::Value, String> {
    let path = crate::utl::get_home_dir()?
        .join(".config")
        .join("moss")
        .join("locales")
        .join(language)
        .join(format!("{namespace}.json"));

    match std::fs::read_to_string(path) {
        Ok(data) => {
            let translations: serde_json::Value =
                serde_json::from_str(&data).map_err(|err| err.to_string())?;

            dbg!(&translations);
            Ok(translations)
        }
        Err(err) => Err(err.to_string()),
    }
}
