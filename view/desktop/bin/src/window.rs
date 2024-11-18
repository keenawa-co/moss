use log::{info, warn};
use rand::random;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WindowEvent};

use crate::{menu, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};

pub struct CreateWindowInput<'s> {
    pub url: &'s str,
    pub label: &'s str,
    pub title: &'s str,
    pub inner_size: (f64, f64),
    pub position: (f64, f64),
}

pub fn create_window(app_handle: &AppHandle, input: CreateWindowInput) -> WebviewWindow {
    info!("Create new window label={}", input.label);

    #[cfg(target_os = "macos")]
    {
        let menu = menu::app_menu(app_handle).unwrap();
        app_handle.set_menu(menu).expect("Failed to set app menu");
    }

    let mut win_builder = tauri::WebviewWindowBuilder::new(
        app_handle,
        input.label,
        WebviewUrl::App(input.url.into()),
    )
    .title(input.title)
    .center()
    .resizable(true)
    .visible(true)
    .disable_drag_drop_handler()
    .inner_size(input.inner_size.0, input.inner_size.1)
    .position(input.position.0, input.position.1)
    .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

    #[cfg(target_os = "windows")]
    {
        win_builder = win_builder
            .transparent(true)
            .shadow(false)
            .decorations(false);
    }

    #[cfg(target_os = "macos")]
    {
        win_builder = win_builder
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay);
    }

    let webview_window = win_builder
        .build()
        .map_err(|e| format!("Failed to build window: {}", e))
        .unwrap();

    if let Err(err) = webview_window.set_focus() {
        warn!(
            "Failed to set focus to window {} when creating it: {}",
            input.label, err
        );
    }

    webview_window
}

pub fn create_child_window(
    parent_window: WebviewWindow,
    url: &str,
    label: &str,
    title: &str,
    inner_size: (f64, f64),
) -> Result<(), String> {
    let app_handle = parent_window.app_handle();
    let config = CreateWindowInput {
        url,
        label,
        title,
        inner_size,
        position: (
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };

    let child_window = create_window(&app_handle, config);

    {
        let parent_window = parent_window.clone();
        let child_window = child_window.clone();
        child_window.clone().on_window_event(move |e| match e {
            // When the new window is destroyed, bring the other up behind it
            WindowEvent::Destroyed => {
                if let Some(w) = parent_window.get_webview_window(child_window.label()) {
                    w.set_focus().unwrap();
                }
            }
            _ => {}
        });
    }

    {
        let parent_window = parent_window.clone();
        let child_window = child_window.clone();
        parent_window.clone().on_window_event(move |e| match e {
            // When the parent window is focused, bring the child above
            WindowEvent::Focused(focus) => {
                if *focus {
                    if let Some(w) = parent_window.get_webview_window(child_window.label()) {
                        w.set_focus().unwrap();
                    };
                }
            }
            _ => {}
        });
    }

    Ok(())
}
