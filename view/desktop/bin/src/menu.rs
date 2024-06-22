use strum::{AsRefStr as StrumAsRefStr, Display as StrumDisplay, EnumString as StrumEnumString};
use tauri::{
    menu::{Menu, MenuItemKind},
    AppHandle, Wry,
};

#[derive(Debug, StrumEnumString, StrumDisplay, StrumAsRefStr)]
pub enum MenuEvent {
    NewWindow,
}

const REQUIRE_LIBRARY: &[MenuEvent] = &[MenuEvent::NewWindow];

pub fn set_enabled(menu: &Menu<Wry>, event: &MenuEvent, enabled: bool) -> tauri::Result<()> {
    match menu.get(event.as_ref()) {
        Some(MenuItemKind::MenuItem(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Submenu(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Predefined(_)) => Ok(()),
        Some(MenuItemKind::Check(i)) => i.set_enabled(enabled),
        Some(MenuItemKind::Icon(i)) => i.set_enabled(enabled),
        None => {
            error!("Failed to get menu item: {event:?}");
            Ok(())
        }
    }
}

pub fn setup_window_menu(handle: &AppHandle) -> tauri::Result<Menu<Wry>> {
    handle.on_menu_event(move |_app, _event| {
        // TODO: handle known and unknown menu events
    });

    #[cfg(not(target_os = "macos"))]
    {
        Menu::new(handle)
    }
    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};

        let app_menu = SubmenuBuilder::new(handle, "Moss Compass")
            .about(Some(
                AboutMetadataBuilder::new()
                    // TODO: .authors(Some(vec![]))
                    .license(Some(env!("CARGO_PKG_VERSION")))
                    .version(Some(env!("CARGO_PKG_VERSION")))
                    // TODO: .website(Some("https://mossland.dev/"))
                    // TODO: .website_label(Some("mossland.dev.com"))
                    .build(),
            ))
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;

        let window_menu = SubmenuBuilder::new(handle, "Window")
            .minimize()
            .item(&MenuItemBuilder::with_id(MenuEvent::NewWindow, "New Window").build(handle)?)
            .build()?;

        let menu = MenuBuilder::new(handle)
            .item(&app_menu)
            .item(&window_menu)
            .build()?;

        for event in REQUIRE_LIBRARY {
            if let Err(err) = set_enabled(&menu, event, false) {
                error!("Failed to set up menu item state: {err:#?}");
            }
        }

        Ok(menu)
    }
}
