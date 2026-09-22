use gilvave_core::dto::ws::ServerRecieve;

pub async fn handle(text: String) {
    if let Ok(msg) = serde_json::from_str::<ServerRecieve>(&text) {
        match msg {
            ServerRecieve::Hello => {
                tracing::info!("Hello");
            }
            ServerRecieve::JoinSuccess => {
                tracing::info!("join success");
            }
            ServerRecieve::MessageNew(message_view) => {
                let _ = tauri_sys::event::emit("message_new", &message_view).await;
            }
            ServerRecieve::ChannelHistoryBefore(messages) => {
                let _ = tauri_sys::event::emit("channel_history_before", &messages).await;
            }
            ServerRecieve::ChannelHistoryAfter(messages) => {
                let _ = tauri_sys::event::emit("channel_history_after", &messages).await;
            }
        }
    }
}
