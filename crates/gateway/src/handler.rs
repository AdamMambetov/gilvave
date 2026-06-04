use gilvave_core::dto::ws::ServerRecieve;
use gilvave_state::MaybeSender;
use tokio::time::{Duration, sleep};

use crate::service::WsService;

pub async fn handle(web_socket: MaybeSender, text: String) {
    if let Ok(msg) = serde_json::from_str::<ServerRecieve>(&text) {
        match msg {
            ServerRecieve::Hello { heartbeat_interval } => {
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_millis(heartbeat_interval - 5000)).await;
                        WsService::heartbeat(web_socket.clone()).await;
                    }
                });
            }
            ServerRecieve::JoinSuccess => {
                tracing::info!("join success")
            }
            ServerRecieve::MessageNew(message_view) => {}
        }
    }
}
