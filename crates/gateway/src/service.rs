use gilvave_core::{dto::ws::ServerSend, ids::ChannelId};
use gilvave_state::MaybeSender;

pub struct WsService;

impl WsService {
    pub async fn heartbeat(web_socket: MaybeSender) {
        if let Some(sender) = web_socket.read().await.as_ref() {
            let res = sender.send(ServerSend::Heartbeat);
            if res.is_err() {
                tracing::error!("heartbeat error: {}", res.err().unwrap())
            }
        }
    }

    pub async fn join_channel(web_socket: MaybeSender) {
        if let Some(sender) = web_socket.read().await.as_ref() {
            sender
                .send(ServerSend::JoinChannel {
                    channel_id: ChannelId::try_from("5bd34915-3649-4644-81c4-a6ec89a9a7ee")
                        .unwrap(),
                })
                .ok();
        }
    }
}
