mod command;
mod mem;
mod menu;
mod service;

use crate::command::{cmd_base, cmd_dummy};
use anyhow::{Context as ResultContext, Result};
use platform_core::common::context::{
    async_context::AsyncContext, entity::Model, AnyContext, Context,
};
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
<<<<<<< HEAD
use std::rc::Rc;
=======
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use std::rc::Rc;
use std::sync::Arc;
use std::{env, process::ExitCode};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use workbench_service_environment_tgui::environment_service::NativeEnvironmentService;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_tgui::Workbench;

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

pub struct AppState<'a> {
    pub workbench: Model<Workbench<'a>>,
    pub platform_info: NativePlatformInfo,
    pub project_service: ProjectService,
    pub session_service: SessionService,
}

pub struct AppMain {
    native_window_configuration: NativeWindowConfiguration,
}

impl AppMain {
    pub fn new(configuration: NativeWindowConfiguration) -> Self {
        Self {
            native_window_configuration: configuration,
        }
    }

    pub fn run<F>(&self, f: F) -> ExitCode
    where
        F: 'static + FnOnce(&Self, Context) -> Result<()>,
    {
        let ctx = Context::new();

        if let Err(e) = f(self, ctx) {
            error!("{}", e);
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    pub fn open_main_window(&self, ctx: Context) -> Result<()> {
        // ------ Example stream
        // TODO:
        // Used only as an example implementation. Remove this disgrace as soon as possible.
        let (tx, rx) = tokio::sync::broadcast::channel(16);

        ctx.detach(async move {
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
        let db = ctx.block_on(async {
            let db = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("failed to connect to db");
            // let db = Surreal::new::<File>("../rocksdb").await.unwrap();
            db.use_ns("moss").use_db("compass").await.unwrap();

            // let schema = include_str!("../schema.surql");
            // db.query(schema).await.unwrap();

            Arc::new(db)
        });

        let async_ctx = ctx.to_async();

        let app_state = async_ctx.with_mut::<Result<AppState>>(|ctx| {
            let service_group = self.create_service_registry()?;

            let window_state = service_group
                .get_unchecked::<MockStorageService>()
                .get_last_window_state();

            let mut workbench =
                Workbench::new(ctx, service_group, window_state.workspace_id).unwrap();

            ctx.block_on(async {
                workbench
                    .initialize()
                    .await
                    .expect("Failed to initialize the workbench");
            });

            Ok(AppState {
                workbench: ctx.new_model(|_ctx| workbench),
                platform_info: self.native_window_configuration.platform_info.clone(),
                project_service: ProjectService::new(db.clone()),
                session_service: SessionService::new(db.clone()),
            })
        })?;

        let builder = tauri_specta::Builder::<tauri::Wry>::new()
            .events(collect_events![])
            .commands(collect_commands![
                cmd_dummy::workbench_get_state,
                cmd_dummy::create_project,
                cmd_dummy::restore_session,
                cmd_dummy::app_ready,
                cmd_dummy::update_font_size,
                cmd_base::native_platform_info,
            ]);

        #[cfg(debug_assertions)]
        self.export_typescript_bindings(&builder)?;

        Ok(self
            .initialize_app(async_ctx, app_state, builder, rx)?
            .run(|_, _| {}))
    }

    fn initialize_app(
        &self,
        async_ctx: AsyncContext,
        app_state: AppState<'static>,
        builder: tauri_specta::Builder,
        mut rx: tokio::sync::broadcast::Receiver<i32>,
    ) -> Result<App> {
        let builder = tauri::Builder::default()
            .manage(async_ctx)
            .manage(app_state)
            .invoke_handler(builder.invoke_handler())
            .setup(move |app: &mut App| {
                let app_state: State<AppState> = app.state();
                let async_ctx: State<AsyncContext> = app.state();

                let app_handle = app.handle().clone();
                let window = app.get_webview_window("main").unwrap();

                let ctx_lock: &mut Context = &mut async_ctx.lock();
                app_state.workbench.update(ctx_lock, |this, ctx| {
                    this.set_configuration_window_size(window).unwrap();
                    this.set_tao_handle(ctx, Rc::new(app_handle.clone()));

                    ctx.notify();
                });

<<<<<<< HEAD
                    ctx.notify();
                });

                // TODO:
                // Used only as an example implementation. Remove this disgrace as soon as possible.
=======
                init_custom_logging(app_handle.clone());

                // TODO:
                // Used only as an example implementation. Remove this disgrace as soon as possible.
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
            .build(tauri::generate_context!())?;

        Ok(builder)
    }

    fn create_service_registry(&self) -> Result<ServiceRegistry> {
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

    fn export_typescript_bindings(&self, builder: &tauri_specta::Builder) -> Result<()> {
        Ok(builder
            .export(
                specta_typescript::Typescript::default()
                    .formatter(specta_typescript::formatter::prettier),
                "../src/bindings.ts",
            )
            .context("Failed to export typescript bindings")?)
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

