mod command;
mod config;
mod mem;
mod menu;
mod service;

use anyhow::{Context as _, Result};
use platform_core::context_v2::async_context::AsyncContext;
use platform_core::context_v2::{Context, ContextCell};
use platform_core::executor::BackgroundExecutor;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_core::platform::AnyPlatform;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::{env, process::ExitCode};
use surrealdb::engine::remote::ws::Client;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use workbench_service_environment_tgui::environment_service::NativeEnvironmentService;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_tgui::Workbench;

use crate::command::{cmd_base, cmd_dummy};

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
    pub project_service: ProjectService,
    pub session_service: SessionService,
}

pub struct AppMain {
    platform_client: Rc<CrossPlatformClient>,
    native_window_configuration: NativeWindowConfiguration,
}

impl AppMain {
    pub fn new(configuration: NativeWindowConfiguration) -> Self {
        Self {
            platform_client: Rc::new(CrossPlatformClient::new()),
            native_window_configuration: configuration,
        }
    }

    pub fn run(&self) -> ExitCode {
        let platform_client_clone = self.platform_client.clone();
        let r = self.platform_client.background_executor();
        self.platform_client.run(async {
            let ctx = ContextCell::new(platform_client_clone);

            if let Err(e) = self.open_main_window(ctx).await {
                error!("{}", e);
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        })
    }

    pub async fn open_main_window(&self, ctx_cell: Rc<ContextCell>) -> Result<()> {
        // TODO: move to StorageService
        let db = Arc::new(
            self.platform_client
                .background_executor()
                .block_on(init_db_client())?,
        );

        let service_group = self.create_service_registry()?;

        let window_state = service_group
            .get_unchecked::<MockStorageService>()
            .get_last_window_state();

        let async_ctx = {
            let ctx = ctx_cell.as_ref().borrow();
            ctx.to_async()
        };

        let workbench = Workbench::new(&async_ctx, service_group, window_state.workspace_id)?;
        workbench.initialize(&async_ctx)?;

        let app_state = AppState {
            workbench: Arc::new(workbench),
            platform_info: self.native_window_configuration.platform_info.clone(),
            project_service: ProjectService::new(db.clone()),
            session_service: SessionService::new(db.clone()),
        };

        let builder = tauri_specta::Builder::<tauri::Wry>::new()
            .events(collect_events![])
            .commands(collect_commands![
                cmd_dummy::workbench_get_state,
                cmd_dummy::create_project,
                cmd_dummy::restore_session,
                cmd_dummy::app_ready,
                cmd_dummy::update_font_size,
                cmd_dummy::fetch_all_themes,
                cmd_dummy::read_theme,
                cmd_base::native_platform_info,
            ]);

        #[cfg(debug_assertions)]
        self.export_typescript_bindings(&builder)?;

        async_ctx
            .spawn_local(|_| async { println!("Hello from awaited spawn!") })
            .await;

        async_ctx
            .spawn_local(|_| async { println!("Hello from detached spawn!") })
            .detach();

        async_ctx.block_on(async { println!("Hello from blocked!") });

        Ok(self
            .initialize_app(async_ctx, app_state, builder)?
            .run(|_, _| {}))
    }

    fn initialize_app(
        &self,
        ctx: AsyncContext,
        app_state: AppState,
        builder: tauri_specta::Builder,
    ) -> Result<App> {
        let builder = tauri::Builder::default()
            .manage(ctx)
            .manage(app_state)
            .invoke_handler(builder.invoke_handler())
            .setup(move |app: &mut App| {
                let app_state: State<AppState> = app.state();
                let async_ctx: State<AsyncContext> = app.state();

                let app_handle = app.handle().clone();
                let window = app.get_webview_window("main").unwrap();

                async_ctx
                    .apply(|ctx| {
                        app_state
                            .workbench
                            .set_configuration_window_size(window)
                            .unwrap();

                        app_state.workbench.set_tao_handle(ctx, app_handle.clone());
                    })
                    .unwrap();

                // init_custom_logging(app_handle.clone());

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
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        // log to frontend
        .with(
            tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
                app_handle: app_handle.clone(),
            }),
        )
        .init();

    event!(tracing::Level::DEBUG, "Logging init");
    info!("Logging initialized");
}

async fn init_db_client() -> Result<Surreal<Client>> {
    // let db = Surreal::new::<File>("../rocksdb").await.unwrap();

    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("failed to connect to db");
    db.use_ns("moss").use_db("compass").await?;

    Ok(db)
}
