use std::sync::Arc;

use gilvave_gateway::commands;
use gilvave_http::commands::{channel, server, user};
use gilvave_state::AppState;
use tauri::Manager;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_tracing::{Builder, LevelFilter, WebviewLayer};
use tokio::sync::RwLock;
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
            user::register,
            user::login,
            user::get_profile,
            server::get_user_servers,
            server::get_members,
            server::create_server,
            channel::get_server_channels,
            commands::listen_web_socket,
            commands::join_channel,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            app_handle.manage(AppState {
                sender: Arc::new(RwLock::new(None)),
                http_client: Client::new(),
            });

            Registry::default()
                .with(fmt::layer())
                .with(WebviewLayer::new(app.handle().clone()))
                .with(filter)
                .init();

            tracing::debug!("Debug from app!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
