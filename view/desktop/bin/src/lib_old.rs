mod command;
mod config;
mod mem;
mod menu;

#[cfg(target_os = "macos")]
extern crate objc;
#[cfg(target_os = "macos")]
mod tauri_plugin_mac_window;

use anyhow::{Context as _, Result};
use platform_core::context_v2::async_context::AsyncContext;
use platform_core::context_v2::ContextCell;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use rand::random;
use std::env;
use std::rc::Rc;
use std::sync::Arc;
use tauri::{
    App, AppHandle, Emitter, LogicalSize, Manager, TitleBarStyle, WebviewUrl, WebviewWindow,
};
use tauri_plugin_shell::ShellExt;
use workbench_desktop::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_desktop::Workbench;
use workbench_service_environment_tao::environment_service::NativeEnvironmentService;

use crate::command::*;

const DEFAULT_WINDOW_WIDTH: f64 = 1100.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 600.0;

const MIN_WINDOW_WIDTH: f64 = 300.0;
const MIN_WINDOW_HEIGHT: f64 = 300.0;

const MAIN_WINDOW_PREFIX: &str = "main_";
const OTHER_WINDOW_PREFIX: &str = "other_";

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

pub struct MockStorageService {}

struct SimpleWindowState {
    workspace_id: WorkspaceId,
}

impl MockStorageService {
    fn new() -> Self {
        Self {}
    }

    fn get_last_window_state(&self) -> SimpleWindowState {
        SimpleWindowState {
            workspace_id: WorkspaceId::Some("workspace_path_hash".to_string()),
        }
    }
}

pub struct AppState {
    pub workbench: Arc<Workbench>,
    pub platform_info: NativePlatformInfo,
    // pub project_service: ProjectService,
    // pub session_service: SessionService,
}

pub fn run(native_window_configuration: NativeWindowConfiguration) -> Result<()> {
    let platform_client = Rc::new(CrossPlatformClient::new());
    platform_client.run(async {
        let ctx_cell = ContextCell::new(platform_client.clone());
        let async_ctx = ctx_cell.borrow().to_async();
        let tao_app = initialize_app(
            async_ctx,
            platform_client.clone(),
            native_window_configuration,
        )
        .expect("Failed to build tauri app");

        Ok(tao_app.run(|app_handle, event| {
            // match event {
            //     tauri::RunEvent::Ready => {
            //         dbg!("Hello!");
            //         let w = create_main_window(app_handle, "/");
            //     }

            //     #[cfg(target_os = "macos")]
            //     tauri::RunEvent::ExitRequested { api, .. } => {
            //         app_handle.hide().ok();
            //         api.prevent_exit();
            //     }
            //     _ => (),
            // }

            if let tauri::RunEvent::Ready = event {
                let w = create_main_window(app_handle, "/");
            }

            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                app_handle.hide().ok();
                api.prevent_exit();
            }

            // To make the compiler happy.
            #[cfg(not(target_os = "macos"))]
            {
                let _ = (app_handle, event);
            }
        }))
    })
}

fn initialize_app(
    ctx: AsyncContext,
    platform_client: Rc<CrossPlatformClient>,
    native_window_configuration: NativeWindowConfiguration,
) -> Result<App> {
    // let builder = create_specta_builder();

    // #[cfg(debug_assertions)]
    // export_typescript_bindings(&builder)?;

    //  TODO: move to StorageService
    // let db = Arc::new(
    //     platform_client
    //         .background_executor()
    //         .block_on(init_db_client())?,
    // );

    let platform_info_clone = native_window_configuration.platform_info.clone();
    let service_group = create_service_registry(native_window_configuration)?;
    let mut builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
        cmd_initial::main_window_is_ready,
        cmd_dummy::workbench_get_state,
        // cmd_dummy::create_project,
        // cmd_dummy::restore_session,
        cmd_dummy::app_ready,
        cmd_dummy::update_font_size,
        cmd_dummy::fetch_all_themes,
        cmd_dummy::read_theme,
        cmd_base::native_platform_info,
        cmd_base::describe_primary_activitybar_part,
        cmd_base::describe_primary_sidebar_part,
        cmd_base::get_view_content,
        cmd_base::get_menu_items,
    ]);

    // #[cfg(target_os = "macos")]
    // {
    //     builder = builder.plugin(tauri_plugin_mac_window::init());
    // }

    let tao_app = builder
        .setup(move |app: &mut App| setup_app(app, ctx, service_group, platform_info_clone))
        .menu(menu::setup_window_menu)
        // .on_window_event(|window, event| match event {
        //     #[cfg(target_os = "macos")]
        //     tauri::WindowEvent::CloseRequested { api, .. } => {
        //         if window.app_handle().webview_windows().len() == 1 {
        //             window.app_handle().hide().ok();
        //             api.prevent_close();
        //         }
        //     }
        //     tauri::WindowEvent::Destroyed => {}
        //     tauri::WindowEvent::Focused(focused) if *focused => {}
        //     _ => {}
        // })
        .plugin(tauri_plugin_os::init())
        .build(tauri::generate_context!())?;

    Ok(tao_app)
}

fn setup_app(
    app: &mut App,
    mut ctx: AsyncContext,
    service_group: ServiceRegistry,
    // db: Arc<Surreal<Client>>,
    platform_info: NativePlatformInfo,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window_state = service_group
        .get_unchecked::<MockStorageService>()
        .get_last_window_state();

    let mut workbench = Workbench::new(&mut ctx, service_group, window_state.workspace_id)?;
    workbench.initialize(&mut ctx)?;

    let window = app.get_webview_window("main").unwrap();

    let app_state = AppState {
        workbench: Arc::new(workbench),
        platform_info,
        // project_service: ProjectService::new(db.clone()),
        // session_service: SessionService::new(db.clone()),
    };

    ctx.apply(|tx_ctx| {
        app_state
            .workbench
            .set_configuration_window_size(&window)
            .unwrap();

        app_state
            .workbench
            .set_tao_handle(tx_ctx, app.handle().clone());
    })?;

    {
        app.handle().manage(ctx);
        app.handle().manage(app_state);
    }

    // let window = app.get_webview_window("main").unwrap();
    // window.emit("app-loaded", "Hello, World!").unwrap();

    Ok(())
}

fn create_service_registry(
    native_window_configuration: NativeWindowConfiguration,
) -> Result<ServiceRegistry> {
    let mut service_registry = ServiceRegistry::new();

    let mock_storage_service = MockStorageService::new();

    let fs_service = DiskFileSystemService::new();
    let environment_service =
        NativeEnvironmentService::new(native_window_configuration.home_dir.clone());

    service_registry.insert(mock_storage_service);
    service_registry.insert(environment_service);
    service_registry.insert(Arc::new(fs_service));

    Ok(service_registry)
}

fn create_main_window(handle: &AppHandle, url: &str) -> WebviewWindow {
    let label = format!("{MAIN_WINDOW_PREFIX}{}", handle.webview_windows().len());
    let config = CreateWindowConfig {
        url,
        label: label.as_str(),
        title: "Moss Studio",
        inner_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        position: (
            // Offset by random amount so it's easier to differentiate
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };
    create_window(handle, config)
}

struct CreateWindowConfig<'s> {
    url: &'s str,
    label: &'s str,
    title: &'s str,
    inner_size: (f64, f64),
    position: (f64, f64),
}

fn create_window(handle: &AppHandle, config: CreateWindowConfig) -> WebviewWindow {
    // #[allow(unused_variables)]
    // let menu = app_menu(handle).unwrap();

    // This causes the window to not be clickable (in AppImage), so disable on Linux
    // #[cfg(not(target_os = "linux"))]
    // handle.set_menu(menu).expect("Failed to set app menu");

    info!("Create new window label={}", config.label);

    let mut win_builder =
        tauri::WebviewWindowBuilder::new(handle, config.label, WebviewUrl::App(config.url.into()))
            .title(config.title)
            .resizable(true)
            .visible(false) // To prevent theme flashing, the frontend code calls show() immediately after configuring the theme
            .fullscreen(false)
            .disable_drag_drop_handler() // Required for frontend Dnd on windows
            .inner_size(config.inner_size.0, config.inner_size.1)
            .position(config.position.0, config.position.1)
            .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

    // Add macOS-only things
    #[cfg(target_os = "macos")]
    {
        win_builder = win_builder
            .hidden_title(true)
            .title_bar_style(TitleBarStyle::Overlay);
    }

    // Add non-MacOS things
    #[cfg(not(target_os = "macos"))]
    {
        // Doesn't seem to work from Rust, here, so we do it in main.tsx
        win_builder = win_builder.decorations(false);
    }

    if let Some(w) = handle.webview_windows().get(config.label) {
        info!(
            "Webview with label {} already exists. Focusing existing",
            config.label
        );
        w.set_focus().unwrap();
        return w.to_owned();
    }

    let win = win_builder.build().unwrap();

    let webview_window = win.clone();
    win.on_menu_event(move |w, event| {
        if !w.is_focused().unwrap() {
            return;
        }

        let event_id = event.id().0.as_str();
        match event_id {
            "quit" => std::process::exit(0),
            "close" => w.close().unwrap(),
            "zoom_reset" => w.emit("zoom_reset", true).unwrap(),
            "zoom_in" => w.emit("zoom_in", true).unwrap(),
            "zoom_out" => w.emit("zoom_out", true).unwrap(),
            "settings" => w.emit("settings", true).unwrap(),

            // Commands for development
            "dev.reset_size" => webview_window
                .set_size(LogicalSize::new(
                    DEFAULT_WINDOW_WIDTH,
                    DEFAULT_WINDOW_HEIGHT,
                ))
                .unwrap(),
            "dev.refresh" => webview_window.eval("location.reload()").unwrap(),
            "dev.generate_theme_css" => {
                w.emit("generate_theme_css", true).unwrap();
            }
            "dev.toggle_devtools" => {
                if webview_window.is_devtools_open() {
                    webview_window.close_devtools();
                } else {
                    webview_window.open_devtools();
                }
            }
            _ => {}
        }
    });

    win
}

fn export_typescript_bindings(builder: &tauri_specta::Builder) -> Result<()> {
    Ok(builder
        .export(
            specta_typescript::Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */"),
            "../src/bindings.ts",
        )
        .context("Failed to export typescript bindings")?)
}

// async fn init_db_client() -> Result<Surreal<Client>> {
//     // let db = Surreal::new::<File>("../rocksdb").await.unwrap();

//     let db = Surreal::new::<Ws>("127.0.0.1:8000")
//         .await
//         .expect("failed to connect to db");
//     db.use_ns("moss").use_db("compass").await?;

//     Ok(db)
// }

// An example of how the logging could function
// fn init_custom_logging(app_handle: tauri::AppHandle) {
//     struct TauriLogWriter {
//         app_handle: tauri::AppHandle,
//     }

//     impl std::io::Write for TauriLogWriter {
//         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//             let log_message = String::from_utf8_lossy(buf).to_string();
//             let _ = self.app_handle.emit("logs-stream", log_message);
//             Ok(buf.len())
//         }

//         fn flush(&mut self) -> std::io::Result<()> {
//             Ok(())
//         }
//     }

//     tracing_subscriber::registry()
//         // log to stdout
//         .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
//         // log to frontend
//         .with(
//             tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
//                 app_handle: app_handle.clone(),
//             }),
//         )
//         .init();

//     event!(tracing::Level::DEBUG, "Logging init");
//     info!("Logging initialized");
// }
