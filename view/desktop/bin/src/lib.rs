mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use app::context_compact::AppContextCompact;
use platform_configuration_common::configuration_registry::{
    ConfigurationNode, ConfigurationPropertySchema, ConfigurationRegistry,
};
use platform_configuration_common::{
    configuration_registry::{
        ConfigurationNodeType, ConfigurationScope, ConfigurationSource, PropertyMap, PropertyPolicy,
    },
    property_key,
};
use platform_formation_common::service_registry::ServiceRegistry;
use platform_window_tgui::window::NativeWindowConfiguration;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::env;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, AppHandle, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use workbench_tgui::{Workbench, WorkbenchState};
use workspace::WorkspaceId;

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

pub struct DesktopMain {
    native_window_configuration: NativeWindowConfiguration,
}

impl DesktopMain {
    pub fn new(configuration: NativeWindowConfiguration) -> Self {
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

        let service_group = self.initialize_service_group()?;

        let window_state = service_group
            .get_unchecked::<MockStorageService>()
            .get_last_window_state();

        let mut workbench = Workbench::new(service_group, window_state.workspace_id)?;
        workbench.initialize()?;

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

    fn initialize_service_group(&self) -> Result<ServiceRegistry> {
        let mut service_group = ServiceRegistry::new();

        let storage_service = MockStorageService::new();

        service_group.insert(storage_service);

        Ok(service_group)
    }
}

pub struct AppState<'a> {
    pub workbench: Workbench<'a>,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}
