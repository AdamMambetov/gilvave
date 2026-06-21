use futures_util::{SinkExt, StreamExt};
use gilvave_core::{
    dto::ws::ServerSend, error::ErrorInfo, ids::ChannelId, security::get_access_token,
    settings::BASE_WS_URL,
};
use gilvave_state::{AppState, MaybeSender};
use tauri::{AppHandle, State};
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, client::IntoClientRequest},
};

use crate::handler::handle;

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

    pub async fn join_channel(
        sender_ptr: MaybeSender,
        channel_id: ChannelId,
    ) -> Result<(), ErrorInfo> {
        match sender_ptr.read().await.as_ref() {
            Some(sender) => sender
                .send(ServerSend::JoinChannel { channel_id })
                .map_err(|e| ErrorInfo(1u16, e.to_string())),
            None => Err(ErrorInfo(1u16, "Read websocket fail!".to_string())),
        }
    }

    pub async fn listen_web_socket(
        state: State<'_, AppState>,
        app_handle: AppHandle,
    ) -> Result<bool, ErrorInfo> {
        let invalid_sender = state.sender.read().await.is_none();
        if invalid_sender {
            let mut request = BASE_WS_URL.into_client_request().unwrap();
            request.headers_mut().insert(
                "Authorization",
                format!("Bearer {}", get_access_token()).parse().unwrap(),
            );

            let (sender, mut reciever) = mpsc::unbounded_channel::<ServerSend>();

            {
                let mut state_sender = state.sender.write().await;
                *state_sender = Some(sender);
            }

            match connect_async(request).await {
                Ok((ws_stream, _)) => {
                    let (mut ws_sender, mut ws_reciever) = ws_stream.split();

                    let recieve_task = async {
                        tracing::info!("recieve_task start");
                        while let Some(msg) = ws_reciever.next().await {
                            tracing::info!("recieve_task while");
                            match msg {
                                Ok(Message::Text(text)) => {
                                    tracing::info!("Received: {text}");
                                    handle(
                                        state.sender.clone(),
                                        app_handle.clone(),
                                        text.to_string(),
                                    )
                                    .await;
                                }
                                Ok(Message::Close(_)) => {
                                    tracing::warn!("WebSocket connection closed");
                                    break;
                                }
                                Err(e) => {
                                    tracing::error!("WebSocket error: {e}");
                                    break;
                                }
                                _ => {}
                            }
                        }
                        tracing::info!("recieve_task end");
                    };

                    let send_task = async {
                        tracing::info!("send_task start");
                        while let Some(msg) = reciever.recv().await {
                            tracing::info!("send_task while start");
                            let json = serde_json::to_string(&msg).unwrap();
                            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                                tracing::error!("ws_sender error!");
                            }
                            tracing::info!("send_task while end");
                        }
                        tracing::info!("send_task end");
                    };

                    tokio::select! {
                        _ = recieve_task => {
                            tracing::info!("recieve_task finished")
                        },
                        _ = send_task => {
                            tracing::info!("send_task finished")
                        },
                    }
                }
                Err(e) => {
                    tracing::error!("Connection error: {e}");
                }
            };
        }
        Ok(true)
    }
}
