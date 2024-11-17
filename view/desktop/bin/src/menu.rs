use strum::{AsRefStr as StrumAsRefStr, Display as StrumDisplay, EnumString as StrumEnumString};
use tauri::{
    menu::{Menu, MenuEvent, MenuId, MenuItemKind, PredefinedMenuItem},
    AppHandle, Emitter, Manager, WebviewWindow, Window, Wry,
};

use crate::{create_main_window, window::create_child_window, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OTHER_WINDOW_PREFIX};

#[derive(Debug, StrumEnumString, StrumDisplay, StrumAsRefStr)]
pub enum BuiltInMenuEvent {
    #[strum(serialize = "file.newWindow")]
    NewWindow,
    #[strum(serialize = "file.closeWindow")]
    CloseWindow,
}

const REQUIRE_LIBRARY: &[BuiltInMenuEvent] = &[BuiltInMenuEvent::NewWindow];

pub fn set_enabled(menu: &Menu<Wry>, event: &BuiltInMenuEvent, enabled: bool) -> tauri::Result<()> {
    match menu.get(event.as_ref()) {
        Some(MenuItemKind::MenuItem(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Submenu(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Predefined(_)) => Ok(()),
        Some(MenuItemKind::Check(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Icon(i)) => i.set_enabled(enabled),
        None => {
            // FIXME: error!("Failed to get menu item: {event:?}");
            Ok(())
        }
    }
}

pub fn handle_event(_window: &Window, webview_label: &str, event: &MenuEvent) {
    let event_id = event.id().0.as_str();
    let app_handle = _window.app_handle().clone();
    match event_id {
        "file.newWindow" => {
            create_main_window(&app_handle, "/");
        },
        _ => {}
    }
}

pub fn app_menu(app_handle: &AppHandle) -> tauri::Result<Menu<Wry>> {
    #[cfg(not(target_os = "macos"))]
    {
        Menu::new(app_handle)
    }

    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};

        unsafe {
            macos_trampoline::set_app_name(&"Moss Studio".into());
        }

        let app_menu = SubmenuBuilder::new(app_handle, "Moss")
            .item(&PredefinedMenuItem::about(
                app_handle,
                Some("About Moss Studio"),
                Some(
                    AboutMetadataBuilder::new()
                        .license(Some(env!("CARGO_PKG_VERSION")))
                        .version(Some(env!("CARGO_PKG_VERSION")))
                        // TODO: .website(Some("https://mossland.dev/"))
                        // TODO: .website_label(Some("mossland.dev.com"))
                        .build(),
                ),
            )?)
            .separator()
            .item(&PredefinedMenuItem::hide(
                app_handle,
                Some("Hide Moss Studio"),
            )?)
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;

        let window_menu = SubmenuBuilder::new(app_handle, "Window")
            .minimize()
            .item(
                &MenuItemBuilder::with_id(BuiltInMenuEvent::NewWindow, "New Window")
                    .build(app_handle)?,
            )
            .build()?;

        let menu = MenuBuilder::new(app_handle)
            .item(&app_menu)
            .item(&window_menu)
            .build()?;

        Ok(menu)
    }
}
