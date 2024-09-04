mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use app::context_compact::AppContextCompact;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use std::borrow::Cow;
use std::env;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, AppHandle, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use workbench_service_environment_tgui::environment_service::NativeEnvironmentService;
use workbench_tgui::window::NativeWindowConfiguration;
use workbench_tgui::{Workbench, WorkbenchState};

use crate::service::{
    project_service::{CreateProjectInput, ProjectDTO},
    session_service::SessionInfoDTO,
};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[tauri::command(async)]
#[specta::specta]
async fn workbench_get_state(state: State<'_, AppState<'_>>) -> Result<WorkbenchState, String> {
    Ok(state.workbench.get_state())
}

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

#[tauri::command(async)]
#[specta::specta]
async fn create_project(
    state: State<'_, AppState<'_>>,
    input: CreateProjectInput,
) -> Result<Option<ProjectDTO>, String> {
    match state.project_service.create_project(&input).await {
        Ok(Some(project)) => return Ok(Some(project.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while creating the project: {e}");
            error!(err);
            return Err(err);
        }
    }
}

#[tauri::command(async)]
#[specta::specta]
async fn restore_session(
    state: State<'_, AppState<'_>>,
    project_source: Option<String>,
) -> Result<Option<SessionInfoDTO>, String> {
    match state.session_service.restore_session(project_source).await {
        Ok(Some(session_info)) => return Ok(Some(session_info.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while restoring the session: {e}");
            error!(err);
            return Err(err);
        }
    }
}

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

pub struct DesktopMain<'a> {
    native_window_configuration: NativeWindowConfiguration<'a>,
}

impl<'a> DesktopMain<'a> {
    pub fn new(configuration: NativeWindowConfiguration<'a>) -> Self {
        dbg!(&configuration);
        Self {
            native_window_configuration: configuration,
        }
    }

    pub fn open(&self, ctx: &mut AppContextCompact) -> Result<()> {
        // ------ Example stream
        let (tx, mut rx) = tokio::sync::broadcast::channel(16);

        ctx.detach(|_| async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            let mut count = 0;
            loop {
                interval.tick().await;
                count += 1;
                if tx.send(count).is_err() {
                    break;
                }
            }
        });
        // ------

        // TODO: move to StorageService
        let db = ctx.block_on(|ctx| async {
            let db = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("failed to connect to db");
            // let db = Surreal::new::<File>("../rocksdb").await.unwrap();
            db.use_ns("moss").use_db("compass").await.unwrap();

            // let schema = include_str!("../schema.surql");
            // db.query(schema).await.unwrap();

            Arc::new(db)
        });

        let service_group = self.initialize_service_registry()?;

        let window_state = service_group
            .get_unchecked::<MockStorageService>()
            .get_last_window_state();

        let mut workbench = Workbench::new(service_group, window_state.workspace_id)?;
        ctx.block_on(|_| async {
            workbench
                .initialize()
                .await
                .expect("Failed to initialize the workbench");
        });

        let app_state = AppState {
            workbench,
            project_service: ProjectService::new(db.clone()),
            session_service: SessionService::new(db.clone()),
        };

        let builder = tauri_specta::Builder::<tauri::Wry>::new()
            .events(collect_events![])
            .commands(collect_commands![
                workbench_get_state,
                create_project,
                restore_session,
                app_ready
            ]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        builder
            .export(
                specta_typescript::Typescript::default(),
                "../src/bindings.ts",
            )
            .expect("Failed to export typescript bindings");

        tauri::Builder::default()
            .plugin(tauri_plugin_fs::init())
            .manage(app_state)
            .invoke_handler(builder.invoke_handler())
            .setup(move |app: &mut App| {
                let app_state: State<AppState> = app.state();

                let app_handle = app.handle().clone();
                let window = app.get_webview_window("main").unwrap();

                app_state
                    .workbench
                    .apply_configuration_window_size(window)
                    .unwrap();

                init_custom_logging(app_handle.clone());

                tokio::task::block_in_place(|| {
                    tauri::async_runtime::block_on(async move {
                        // Example stream data emitting
                        tokio::spawn(async move {
                            while let Ok(data) = rx.recv().await {
                                app_handle.emit("data-stream", data).unwrap();
                            }
                        });
                    });
                });

                Ok(())
            })
            .menu(menu::setup_window_menu)
            .plugin(tauri_plugin_os::init())
            .build(tauri::generate_context!())?
            .run(|_, _| {});

        Ok(())
    }

    fn initialize_service_registry(&'a self) -> Result<ServiceRegistry> {
        let mut service_registry = ServiceRegistry::new();

        let mock_storage_service = MockStorageService::new();

        let fs_service = DiskFileSystemService::new();
        let environment_service =
            NativeEnvironmentService::new(self.native_window_configuration.home_dir.clone());

        service_registry.insert(mock_storage_service);
        service_registry.insert(environment_service);
        service_registry.insert(Arc::new(fs_service));

        Ok(service_registry)
    }
}

// An example of how the logging could function
fn init_custom_logging(app_handle: tauri::AppHandle) {
    struct TauriLogWriter {
        app_handle: tauri::AppHandle,
    }

    impl std::io::Write for TauriLogWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let log_message = String::from_utf8_lossy(buf).to_string();
            let _ = self.app_handle.emit("logs-stream", log_message);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    tracing_subscriber::registry()
        // log to stdout
        .with(
            tracing_subscriber::fmt::layer()
            .with_writer(std::io::stdout)
        )
        // log to frontend
        .with(
    tracing_subscriber::fmt::layer()
            .with_writer(move || TauriLogWriter {
                app_handle: app_handle.clone(),
            })  
        )
        .init();
    
    event!(tracing::Level::DEBUG, "Logging init");
    info!("Logging initialized");
}

pub struct AppState<'a> {
    pub workbench: Workbench<'a>,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}
