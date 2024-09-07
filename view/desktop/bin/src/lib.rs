mod command;
mod config;
mod mem;
mod menu;
mod service;

use anyhow::{Context as ResultContext, Result};
use async_task::Runnable;
use platform_core::common::context::async_context::ModernAsyncContext;
use platform_core::common::context::Context;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use service::project_service::ProjectService;
use service::session_service::SessionService;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::{env, process::ExitCode};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, Emitter, Manager, State};
use tauri_specta::{collect_commands, collect_events};
use tokio::runtime::Runtime as TokioRuntime;
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
    native_window_configuration: NativeWindowConfiguration,
}

impl AppMain {
    pub fn new(configuration: NativeWindowConfiguration) -> Self {
        Self {
            native_window_configuration: configuration,
        }
    }

    pub fn run(&self) -> ExitCode {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .max_blocking_threads(*config::RUNTIME_MAX_BLOCKING_THREADS)
            .thread_stack_size(*config::RUNTIME_STACK_SIZE)
            .build()
            .unwrap();

        let (main_tx, main_rx) = flume::unbounded::<Runnable>();
        let local_set = tokio::task::LocalSet::new();
        local_set.spawn_local(async move {
            dbg!("spawn_local 1");
            while let Ok(runnable) = main_rx.recv_async().await {
                dbg!("spawn_local 2");
                runnable.run();
            }
        });

        let ctx = Context::new(main_tx);

        runtime.block_on(local_set.run_until(async {
            if let Err(e) = self.open_main_window(ctx).await {
                error!("{}", e);
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }))
    }

    pub async fn open_main_window(&self, ctx: Rc<RefCell<Context>>) -> Result<()> {
        // ------ Example stream
        // TODO:
        // Used only as an example implementation. Remove this disgrace as soon as possible.
        let (tx, rx) = tokio::sync::broadcast::channel(16);

        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let mut count = 0;
        loop {
            interval.tick().await;
            count += 1;
            if tx.send(count).is_err() {
                break;
            }
        }

        // ------

        // TODO: move to StorageService
        let db = {
            let db = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("failed to connect to db");
            // let db = Surreal::new::<File>("../rocksdb").await.unwrap();
            db.use_ns("moss").use_db("compass").await.unwrap();

            // let schema = include_str!("../schema.surql");
            // db.query(schema).await.unwrap();

            Arc::new(db)
        };

        let service_group = self.create_service_registry()?;

        let window_state = service_group
            .get_unchecked::<MockStorageService>()
            .get_last_window_state();

        let workbench = {
            let ctx_mut: &mut Context = &mut ctx.as_ref().borrow_mut();

            Workbench::new(ctx_mut, service_group, window_state.workspace_id).unwrap()
        };

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
                cmd_base::native_platform_info,
            ]);

        #[cfg(debug_assertions)]
        self.export_typescript_bindings(&builder)?;

        let async_ctx = {
            let ctx = ctx.as_ref().borrow();
            ctx.to_async()
        };

        Ok(self
            .initialize_app(async_ctx, app_state, builder, rx)?
            .run(|_, _| {}))
    }

    fn initialize_app(
        &self,
        ctx: ModernAsyncContext,
        app_state: AppState,
        builder: tauri_specta::Builder,
        mut rx: tokio::sync::broadcast::Receiver<i32>,
    ) -> Result<App> {
        let builder = tauri::Builder::default()
            .manage(ctx)
            .manage(app_state)
            .invoke_handler(builder.invoke_handler())
            .setup(move |app: &mut App| {
                let app_state: State<AppState> = app.state();
                let async_ctx: State<ModernAsyncContext> = app.state();

                let app_handle = app.handle().clone();
                let window = app.get_webview_window("main").unwrap();

                let workbench = Arc::clone(&app_state.workbench);

                // async_ctx
                //     .spawn(|ctx| async {
                //         println!("Hello, World!");
                //     })
                //     .detach();

                async_ctx
                    .update(|ctx: &mut Context| {
                        ctx.spawn_local(|cx| async {
                            // println!("hello from spawn!");

                            workbench
                                .initialize(cx)
                                .await
                                .expect("Failed to initialize the workbench");
                        })
                        .detach();

                        app_state
                            .workbench
                            .set_configuration_window_size(window)
                            .unwrap();

                        app_state.workbench.set_tao_handle(ctx, app_handle);
                    })
                    .unwrap();

                // app_state
                //     .workbench
                //     .set_configuration_window_size(window)
                //     .unwrap();

                // app_state.workbench.set_tao_handle(ctx_lock, app_handle);

                // TODO:
                // Used only as an example implementation. Remove this disgrace as soon as possible.
                // tokio::task::block_in_place(|| {
                //     tauri::async_runtime::block_on(async move {
                //         // Example stream data emitting
                //         tokio::spawn(async move {
                //             while let Ok(data) = rx.recv().await {
                //                 app_handle.emit("data-stream", data).unwrap();
                //             }
                //         });
                //     });
                // });

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
