use gilvave_core::dto::ws::ServerRecieve;
use tauri::{AppHandle, Emitter};

pub async fn handle(app_handle: AppHandle, text: String) {
    if let Ok(msg) = serde_json::from_str::<ServerRecieve>(&text) {
        match msg {
            ServerRecieve::Hello => {
                tracing::info!("Hello")
            }
            ServerRecieve::JoinSuccess => {
                tracing::info!("join success")
            }
            ServerRecieve::MessageNew(message_view) => {
                app_handle.emit("message_new", message_view).ok();
            }
            ServerRecieve::ChannelHistoryBefore(messages) => {
                app_handle.emit("channel_history_before", messages).ok();
            }
            ServerRecieve::ChannelHistoryAfter(messages) => {
                app_handle.emit("channel_history_after", messages).ok();
            }
        }
    }
}
