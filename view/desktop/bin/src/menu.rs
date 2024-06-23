use strum::{AsRefStr as StrumAsRefStr, Display as StrumDisplay, EnumString as StrumEnumString};
use tauri::{
    menu::{Menu, MenuItemKind, PredefinedMenuItem},
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

pub fn setup_window_menu(manager: &AppHandle) -> tauri::Result<Menu<Wry>> {
    manager.on_menu_event(move |_app, _event| {
        // TODO: handle known and unknown menu events
    });

    #[cfg(not(target_os = "macos"))]
    {
        Menu::new(manager)
    }
    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};

        unsafe {
            macos_trampoline::set_app_name(&"Moss Compass".into());
        }

        let app_menu = SubmenuBuilder::new(manager, "Moss")
            .item(&PredefinedMenuItem::about(
                manager,
                Some("About Moss Compass"),
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
                manager,
                Some("Hide Moss Compass"),
            )?)
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?;

        let window_menu = SubmenuBuilder::new(manager, "Window")
            .minimize()
            .item(&MenuItemBuilder::with_id(MenuEvent::NewWindow, "New Window").build(manager)?)
            .build()?;

        let menu = MenuBuilder::new(manager)
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
