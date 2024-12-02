use smol::fs;
use tauri::{AppHandle, Emitter, EventTarget};

// #[tauri::command(async)]
// pub async fn apply_theme(app_handle: AppHandle, path: String) -> Result<(), String> {
//     let theme_css = read_theme_css(path).await?;
//     let path = crate::utl::get_themes_dir()?.join(path);
//
//     if path.exists() && path.is_file() {
//         let theme_css = fs::read_to_string(path).await.map_err(|e| e.to_string());
//         app_handle.emit_filter("apply_theme", theme_css, |t| match t {
//             EventTarget::WebviewWindow {..} => true,
//             _ => false,
//         });
//         Ok(())
//     } else {
//         Err("File not found or inaccessible".into())
//     }
// }

#[tauri::command(async)]
pub async fn select_theme(app_handle: AppHandle, theme_id: String) -> Result<(), String> {
    app_handle.emit_filter("select_theme", theme_id, |t| match t {
        EventTarget::WebviewWindow { .. } => true,
        _ => false,
    });
    Ok(())
}
