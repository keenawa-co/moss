use smol::fs;

#[tauri::command(async)]
pub async fn read_theme_file(path: String) -> Result<String, String> {
    let path = crate::utl::get_themes_dir()?.join(path);

    if path.exists() && path.is_file() {
        fs::read_to_string(path).await.map_err(|e| e.to_string())
    } else {
        Err("File not found or inaccessible".into())
    }
}
