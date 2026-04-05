
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use gilvave_http::user_api;

#[derive(Clone)]
struct AppState {
    ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct A1 {
    op: String,
}

#[tauri::command]
async fn request(state: State<'_, AppState>) -> Result<String, String> {
    println!("request debug print");

    let json = serde_json::to_string(&A1 {
        op: "Heartbeat".to_string(),
    })
    .map_err(|e| e.to_string())?;

    let mut ws_guard = state.ws_stream.lock().await;
    let res = ws_guard.send(Message::Text(json.into())).await;

    if let Err(e) = res {
        println!("Error sending message: {}", e);
        return Err(e.to_string());
    }

    Ok("Message sent".to_string())
}

async fn connect_websocket(
    url: String,
) -> Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>> {
    match connect_async(&url).await {
        Ok((ws_stream, _)) => Some(Arc::new(Mutex::new(ws_stream))),
        Err(e) => {
            eprintln!("Connection error: {}", e);
            None
        }
    }
}

fn start_websocket_listener(websocket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>) {
    println!("start_websocket_listener");
    tauri::async_runtime::spawn(async move {
        let mut ws_guard = websocket.lock().await;
        while let Some(msg) = ws_guard.next().await {
            println!("while");
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received: {}", text);
                    // Здесь можно отправить событие на фронтенд
                    // app_handle.emit("websocket-message", text).ok();
                }
                Ok(Message::Close(_)) => {
                    println!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    println!("start_websocket_listener end");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![request, user_api::register])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Используем существующий асинхронный рантайм Tauri
            tauri::async_runtime::spawn(async move {
                match connect_websocket("ws://26.186.139.15:3000/ws".to_string()).await {
                    Some(ws) => {
                        app_handle.manage(AppState {
                            ws_stream: ws.clone(),
                        });
                        start_websocket_listener(ws.clone());
                        println!("WebSocket connected successfully");
                    }
                    None => {
                        eprintln!("Failed to connect to WebSocket");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
