mod mem;

pub mod menu;
pub mod service;

use anyhow::Result;
use app::context_compact::AppContextCompact;

use parking_lot::Mutex;
use platform_core::{
    common::{
        context::{entity::Model, AnyContext, Context},
        runtime::AsyncRuntime,
    },
    tao::context::command_context::CommandAsyncContext,
};
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::env;
use std::rc::Rc;
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
async fn update_font_size(
    cmd_ctx: State<'_, CommandAsyncContext>,
    state: State<'_, AppState<'_>>,
    input: i32,
) -> Result<(), String> {
    cmd_ctx.with_mut(|ctx| {
        state.workbench.update(ctx, |this, cx| {
            this.update_conf(cx, input as usize).unwrap();
            cx.notify();
        });

        Ok(())
    })
}

#[tauri::command(async)]
#[specta::specta]
async fn workbench_get_state(state: State<'_, AppState<'_>>) -> Result<WorkbenchState, String> {
    // Ok(state.workbench.get_state())
    dbg!(1);
    Ok(WorkbenchState::Empty)
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
        Self {
            native_window_configuration: configuration,
        }
    }

    pub fn open(&self, ctx2: &mut AppContextCompact) -> Result<()> {
        // ------ Example stream
        let (tx, mut rx) = tokio::sync::broadcast::channel(16);

        ctx2.detach(|_| async move {
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
        let db = ctx2.block_on(|ctx| async {
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

        let rt = AsyncRuntime::new();

        let cx2 = ctx2.clone();

        rt.run(move |ctx: Arc<Mutex<Context>>| {
            let mut ctx_lock = ctx.lock();
            let mut workbench =
                Workbench::new(&mut ctx_lock, service_group, window_state.workspace_id).unwrap();

            cx2.block_on(|_| async {
                workbench
                    .initialize()
                    .await
                    .expect("Failed to initialize the workbench");
            });

            let app_state = AppState {
                workbench: ctx_lock.new_model(|_ctx| workbench),
                project_service: ProjectService::new(db.clone()),
                session_service: SessionService::new(db.clone()),
            };

            let builder = tauri_specta::Builder::<tauri::Wry>::new()
                .events(collect_events![])
                .commands(collect_commands![
                    workbench_get_state,
                    create_project,
                    restore_session,
                    app_ready,
                    update_font_size,
                ]);

            #[cfg(debug_assertions)] // <- Only export on non-release builds
            builder
                .export(
                    specta_typescript::Typescript::default()
                        .formatter(specta_typescript::formatter::prettier),
                    "../src/bindings.ts",
                )
                .expect("Failed to export typescript bindings");

            drop(ctx_lock);
            tauri::Builder::default()
                .manage(CommandAsyncContext::from(Arc::clone(&ctx)))
                .manage(app_state)
                .invoke_handler(builder.invoke_handler())
                .setup(move |app: &mut App| {
                    let app_state: State<AppState> = app.state();

                    let app_handle = app.handle().clone();
                    let window = app.get_webview_window("main").unwrap();

                    let ctx_lock: &mut Context = &mut ctx.lock();
                    app_state.workbench.update(ctx_lock, |this, ctx| {
                        this.set_configuration_window_size(window).unwrap();
                        this.set_tao_handle(ctx, Rc::new(app_handle.clone()));

                        ctx.notify();
                    });

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
                .build(tauri::generate_context!())
                .unwrap()
                .run(|_, _| {});
        });

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

pub struct AppState<'a> {
    pub workbench: Model<Workbench<'a>>,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}
