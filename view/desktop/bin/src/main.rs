// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::{
    menu,
    service::{
        project_service::{CreateProjectInput, ProjectDTO, ProjectService},
        session_service::{SessionInfoDTO, SessionService},
    },
    AppState,
};
use parking_lot::Mutex;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tauri::{App, AppHandle, Manager, State};
use tauri_specta::{collect_commands, collect_events, ts};
use tracing::error;
use workbench::configuration::{
    configuration_registry::ConfigurationRegistry, configuration_service::ConfigurationService,
    AbstractConfigurationService,
};

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

#[tauri::command(async)]
#[specta::specta]
async fn create_project(
    state: State<'_, AppState>,
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
    state: State<'_, AppState>,
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

fn main() {
    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        if let Err(err) = run(ctx) {
            error!("{err:#?}")
        }
    })
}

pub fn run(ctx: &mut AppContextCompact) -> tauri::Result<()> {
    let (tx, mut rx) = tokio::sync::broadcast::channel(16);

    // Example stream
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

    let db = ctx.block_on(|ctx| async {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        // let db = Surreal::new::<File>("../rocksdb").await.unwrap();
        db.use_ns("moss").use_db("compass").await.unwrap();

        // let schema = include_str!("../schema.surql");
        // db.query(schema).await.unwrap();

        Arc::new(db)
    });

    let registry = Arc::new(Mutex::new(ConfigurationRegistry::new()));
    let config_service =
        ConfigurationService::new(registry, "../../../.moss/settings.json").unwrap();

    let value = config_service.get_value("editor.fontSize", None);
    println!("Config Value: {:?}", value);

    let value = config_service.get_value("editor.fontSize", Some("mossql"));
    println!("Config Value: {:?}", value);

    let value = config_service.get_value("editor.fontSize", Some("mossql"));
    println!("Config Value: {:?}", value);

    // let value = config_service.get_value("editor.fontSize", Some("[mossql]/[test]"));
    // println!("Config Value: {:?}", value);

    // let schema = schemars::schema_for!(WindowSettingsSchema);

    // println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .events(collect_events![])
            .commands(collect_commands![
                create_project,
                restore_session,
                app_ready,
            ])
            .config(specta::ts::ExportConfig::new().formatter(specta::ts::formatter::prettier));

        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .manage(AppState {
            project_service: ProjectService::new(db.clone()),
            session_service: SessionService::new(db.clone()),
        })
        .invoke_handler(invoke_handler)
        .setup(move |app: &mut App| {
            let app_handle = app.handle().clone();

            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    register_events(app);

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
        .build(tauri::generate_context!())?
        .run(|_, _| {});

    Ok(())
}
