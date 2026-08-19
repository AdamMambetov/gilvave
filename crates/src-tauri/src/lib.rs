use std::sync::Arc;

use gilvave_state::AppState;
use tauri::Manager;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_tracing::{Builder, LevelFilter, WebviewLayer};
use tokio::sync::{Mutex, RwLock};
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::database::initialize_database;

mod database;
mod handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tracing_builder = Builder::new()
        .with_max_level(LevelFilter::DEBUG)
        .with_target("hyper", LevelFilter::WARN);
    let filter = tracing_builder.build_filter();

    tauri::Builder::default()
        .plugin(tracing_builder.build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handler::handle_command,
            handler::window_minimize,
            handler::window_toggle_maximize,
            handler::window_close,
            handler::window_start_dragging,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            Registry::default()
                .with(fmt::layer())
                .with(WebviewLayer::new(app.handle().clone()))
                .with(filter)
                .init();

            let db = initialize_database(&app_handle).unwrap();
            app_handle.manage(AppState {
                sender: Arc::new(RwLock::new(None)),
                http_client: Client::new(),
                db: Arc::new(Mutex::new(Some(db))),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
