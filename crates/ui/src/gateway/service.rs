use futures_channel::mpsc::{UnboundedSender, unbounded};
use futures_util::{SinkExt, StreamExt, lock::Mutex};
use gilvave_core::{
    dto::ws::ServerSend, error::ErrorInfo, ids::ChannelId, security::get_access_token,
    settings::BASE_WS_URL,
};
use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, Ordering},
};
use ws_stream_wasm::{WsMessage, WsMeta};

use crate::gateway::handler::handle;

static IS_LISTENING: AtomicBool = AtomicBool::new(false);
static SENDER: LazyLock<Arc<Mutex<Option<UnboundedSender<ServerSend>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub struct WsService;

impl WsService {
    pub async fn join_channel(channel_id: ChannelId) -> Result<(), ErrorInfo> {
        let sender_ptr = SENDER.lock().await;
        match sender_ptr.as_ref() {
            Some(sender) => sender
                .unbounded_send(ServerSend::JoinChannel { channel_id })
                .map_err(|e| ErrorInfo(1, e.to_string())),
            None => Err(ErrorInfo(1, "WebSocket sender not initialized".to_string())),
        }
    }

    pub async fn left_channel(channel_id: ChannelId) -> Result<(), ErrorInfo> {
        let sender_ptr = SENDER.lock().await;
        match sender_ptr.as_ref() {
            Some(sender) => sender
                .unbounded_send(ServerSend::LeftChannel { channel_id })
                .map_err(|e| ErrorInfo(1, e.to_string())),
            None => Err(ErrorInfo(1, "WebSocket sender not initialized".to_string())),
        }
    }

    pub async fn message_create(
        channel_id: ChannelId,
        content: String,
    ) -> Result<(), ErrorInfo> {
        let sender_ptr = SENDER.lock().await;
        match sender_ptr.as_ref() {
            Some(sender) => sender
                .unbounded_send(ServerSend::MessageCreate {
                    channel_id,
                    content,
                })
                .map_err(|e| ErrorInfo(1, e.to_string())),
            None => Err(ErrorInfo(1, "WebSocket sender not initialized".to_string())),
        }
    }

    pub async fn channel_history_before(
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    ) -> Result<(), ErrorInfo> {
        let sender_ptr = SENDER.lock().await;
        match sender_ptr.as_ref() {
            Some(sender) => sender
                .unbounded_send(ServerSend::ChannelHistoryBefore {
                    channel_id,
                    timestamp,
                })
                .map_err(|e| ErrorInfo(1, e.to_string())),
            None => Err(ErrorInfo(1, "WebSocket sender not initialized".to_string())),
        }
    }

    pub async fn channel_history_after(
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    ) -> Result<(), ErrorInfo> {
        let sender_ptr = SENDER.lock().await;
        match sender_ptr.as_ref() {
            Some(sender) => sender
                .unbounded_send(ServerSend::ChannelHistoryAfter {
                    channel_id,
                    timestamp,
                })
                .map_err(|e| ErrorInfo(1, e.to_string())),
            None => Err(ErrorInfo(1, "WebSocket sender not initialized".to_string())),
        }
    }

    pub async fn listen_web_socket() -> Result<bool, ErrorInfo> {
        if IS_LISTENING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            tracing::warn!("listen_web_socket is already running! Skipping duplicate call.");
            return Ok(true);
        }

        wasm_bindgen_futures::spawn_local(async {
            loop {
                tracing::info!("listen_web_socket start");

                let token = get_access_token();
                let ws_url = if token.is_empty() {
                    BASE_WS_URL.to_string()
                } else {
                    format!("{BASE_WS_URL}?token={token}")
                };

                let (sender, mut receiver) = unbounded::<ServerSend>();

                {
                    let mut state_sender = SENDER.lock().await;
                    *state_sender = Some(sender);
                }

                match WsMeta::connect(&ws_url, None).await {
                    Ok((_meta, ws_stream)) => {
                        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                        let receive_task = async {
                            tracing::info!("receive_task start");
                            while let Some(msg) = ws_receiver.next().await {
                                match msg {
                                    WsMessage::Text(text) => {
                                        tracing::info!("Received: {text}");
                                        handle(text).await;
                                    }
                                    _ => {}
                                }
                            }
                            tracing::info!("receive_task end");
                        };

                        let send_task = async {
                            tracing::info!("send_task start");
                            while let Some(msg) = receiver.next().await {
                                if let Ok(json) = serde_json::to_string(&msg) {
                                    if ws_sender.send(WsMessage::Text(json)).await.is_err() {
                                        tracing::error!("ws_sender error!");
                                        break;
                                    }
                                }
                            }
                            tracing::info!("send_task end");
                        };

                        futures_util::future::select(Box::pin(receive_task), Box::pin(send_task)).await;


                        gloo_timers::future::sleep(core::time::Duration::from_millis(1000)).await;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket connection error: {e:?}");
                        gloo_timers::future::sleep(core::time::Duration::from_secs(2)).await;
                        continue;
                    }
                }
            }
        });

        Ok(true)
    }
}
