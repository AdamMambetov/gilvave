// use gilvave_core::state::AppState;
use gilvave_http::{
    commands::{channel, server, user},
    state::AppState,
};
use tauri::Manager;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_tracing::{Builder, LevelFilter, WebviewLayer};
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// async fn connect_websocket(
//     url: String,
// ) -> Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>> {
//     match connect_async(&url).await {
//         Ok((ws_stream, _)) => Some(Arc::new(Mutex::new(ws_stream))),
//         Err(e) => {
//             eprintln!("Connection error: {}", e);
//             None
//         }
//     }
// }

// fn start_websocket_listener(websocket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>) {
//     println!("start_websocket_listener");
//     tauri::async_runtime::spawn(async move {
//         let mut ws_guard = websocket.lock().await;
//         while let Some(msg) = ws_guard.next().await {
//             println!("while");
//             match msg {
//                 Ok(Message::Text(text)) => {
//                     println!("Received: {}", text);
//                     // Здесь можно отправить событие на фронтенд
//                     // app_handle.emit("websocket-message", text).ok();
//                 }
//                 Ok(Message::Close(_)) => {
//                     println!("WebSocket connection closed");
//                     break;
//                 }
//                 Err(e) => {
//                     eprintln!("WebSocket error: {}", e);
//                     break;
//                 }
//                 _ => {}
//             }
//         }
//     });
//     println!("start_websocket_listener end");
// }

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
        ])
        .setup(|app| {
            app.handle().manage(AppState {
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
