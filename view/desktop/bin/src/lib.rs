mod mem;
mod menu;
mod service;

use service::ProjectService;
use tauri::{App, AppHandle, Manager, State};
use tauri_specta::{collect_commands, collect_events, ts};

use surrealdb::{
    engine::local::{Db, File},
    Surreal,
};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

#[tauri::command]
#[specta::specta]
fn create_project(state: State<AppState>, name: String) {
    state.project_service.create_project(name).unwrap();
}

struct AppState {
    project_service: service::ProjectService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Person {
    title: String,
    marketing: bool,
}

pub fn run() -> tauri::Result<()> {
    let (tx, mut rx) = tokio::sync::broadcast::channel(16);

    // Example stream
    tokio::spawn(async move {
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

    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .events(collect_events![])
            .commands(collect_commands![create_project, app_ready, greet])
            .config(specta::ts::ExportConfig::new().formatter(specta::ts::formatter::prettier));

        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .manage(AppState {
            project_service: ProjectService::new("db".to_string()),
        })
        .invoke_handler(invoke_handler)
        .setup(move |app: &mut App| {
            let app_handle = app.handle().clone();

            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    register_events(app);

                    let db = Surreal::new::<File>("../rocksdb").await.unwrap();
                    db.use_ns("test").use_db("test").await.unwrap();

                    let created: Vec<Person> = db
                        .create("person")
                        .content(Person {
                            title: "Founder & CEO".into(),
                            marketing: true,
                        })
                        .await
                        .unwrap();

                    dbg!(created);

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
