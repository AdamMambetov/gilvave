use futures_util::{SinkExt, StreamExt};
use gilvave_core::{
    dto::ws::ServerSend, error::ErrorInfo, ids::ChannelId, security::get_access_token,
    settings::BASE_WS_URL,
};
use gilvave_state::{AppState, MaybeSender};
use tauri::{AppHandle, State};
use tokio::{
    sync::mpsc,
    time::{Duration, timeout},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, client::IntoClientRequest},
};

use crate::handler::handle;

pub struct WsService;

impl WsService {
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

    pub async fn left_channel(
        sender_ptr: MaybeSender,
        channel_id: ChannelId,
    ) -> Result<(), ErrorInfo> {
        match sender_ptr.read().await.as_ref() {
            Some(sender) => {
                _ = timeout(Duration::from_secs(1), async {});

                sender
                    .send(ServerSend::LeftChannel { channel_id })
                    .map_err(|e| ErrorInfo(1u16, e.to_string()))
            }
            None => Err(ErrorInfo(1u16, "Read websocket fail!".to_string())),
        }
    }

    pub async fn listen_web_socket(
        state: State<'_, AppState>,
        app_handle: AppHandle,
    ) -> Result<bool, ErrorInfo> {
        loop {
            tracing::info!("listen_web_socket start");

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

                    let app_handle_cloned = app_handle.clone();
                    let mut receive_task = tokio::spawn(async move {
                        tracing::info!("recieve_task start");
                        while let Some(msg) = ws_reciever.next().await {
                            tracing::info!("recieve_task while");
                            match msg {
                                Ok(Message::Text(text)) => {
                                    tracing::info!("Received: {text}");
                                    handle(app_handle_cloned.clone(), text.to_string()).await;
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
                    });

                    let mut send_task = tokio::spawn(async move {
                        tracing::info!("send_task start");
                        while let Some(msg) = reciever.recv().await {
                            tracing::info!("send_task while start");
                            let json = serde_json::to_string(&msg).unwrap();
                            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                                tracing::error!("ws_sender error!");
                                break;
                            }
                            tracing::info!("send_task while end");
                        }
                        tracing::info!("send_task end");
                    });

                    tokio::select! {
                        _ = &mut receive_task => {
                            tracing::info!("recieve_task finished");
                            send_task.abort();
                        },
                        _ = &mut send_task => {
                            tracing::info!("send_task finished");
                            receive_task.abort();
                        },
                    }
                }
                Err(e) => {
                    // TODO: подождать и сделать переподключение
                    tracing::error!("Connection error: {e}");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };
        }
        Ok(true)
    }

    pub async fn message_create(
        sender_ptr: MaybeSender,
        channel_id: ChannelId,
        content: String,
    ) -> Result<(), ErrorInfo> {
        match sender_ptr.read().await.as_ref() {
            Some(sender) => sender
                .send(ServerSend::MessageCreate {
                    channel_id,
                    content,
                })
                .map_err(|e| ErrorInfo(1u16, e.to_string())),
            None => Err(ErrorInfo(1u16, "Read websocket fail!".to_string())),
        }
    }

    pub async fn channel_history_before(
        sender_ptr: MaybeSender,
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    ) -> Result<(), ErrorInfo> {
        match sender_ptr.read().await.as_ref() {
            Some(sender) => sender
                .send(ServerSend::ChannelHistoryBefore {
                    channel_id,
                    timestamp,
                })
                .map_err(|e| ErrorInfo(1u16, e.to_string())),
            None => Err(ErrorInfo(1u16, "Read websocket fail!".to_string())),
        }
    }

    pub async fn channel_history_after(
        sender_ptr: MaybeSender,
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    ) -> Result<(), ErrorInfo> {
        match sender_ptr.read().await.as_ref() {
            Some(sender) => sender
                .send(ServerSend::ChannelHistoryAfter {
                    channel_id,
                    timestamp,
                })
                .map_err(|e| ErrorInfo(1u16, e.to_string())),
            None => Err(ErrorInfo(1u16, "Read websocket fail!".to_string())),
        }
    }
}
