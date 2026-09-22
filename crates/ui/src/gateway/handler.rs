use gilvave_core::dto::ws::ServerRecieve;

pub async fn handle(text: String) {
    web_sys::console::log_1(&format!("[WS RECV HANDLER] text: {text}").into());
    match serde_json::from_str::<ServerRecieve>(&text) {
        Ok(msg) => match msg {
            ServerRecieve::HeartbeatAck => {
                web_sys::console::log_1(&"[WS] HeartbeatAck".into());
            }
            ServerRecieve::Hello => {
                web_sys::console::log_1(&"[WS] Hello received".into());
            }
            ServerRecieve::Error { message } => {
                web_sys::console::error_1(&format!("[WS] Server error: {message}").into());
            }
            ServerRecieve::JoinSuccess => {
                web_sys::console::log_1(&"[WS] JoinSuccess received".into());
            }
            ServerRecieve::MessageNew(message_view) => {
                web_sys::console::log_1(&format!("[WS] MessageNew received: {:?}", message_view).into());
                if let Err(e) = tauri_sys::event::emit("message_new", &message_view).await {
                    web_sys::console::error_1(&format!("[WS] emit message_new error: {e:?}").into());
                } else {
                    web_sys::console::log_1(&"[WS] emit message_new success".into());
                }
            }
            ServerRecieve::ChannelHistoryBefore(messages) => {
                web_sys::console::log_1(&format!("[WS] ChannelHistoryBefore received {} messages", messages.len()).into());
                if let Err(e) = tauri_sys::event::emit("channel_history_before", &messages).await {
                    web_sys::console::error_1(&format!("[WS] emit channel_history_before error: {e:?}").into());
                }
            }
            ServerRecieve::ChannelHistoryAfter(messages) => {
                web_sys::console::log_1(&format!("[WS] ChannelHistoryAfter received {} messages", messages.len()).into());
                if let Err(e) = tauri_sys::event::emit("channel_history_after", &messages).await {
                    web_sys::console::error_1(&format!("[WS] emit channel_history_after error: {e:?}").into());
                }
            }
        },
        Err(e) => {
            web_sys::console::error_1(&format!("[WS RECV HANDLER] parse ServerRecieve error: {e}, text: {text}").into());
        }
    }
}
