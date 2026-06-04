use gilvave_core::dto::ws::ServerSend;
use std::sync::Arc;
use tauri_plugin_http::reqwest::Client;
use tokio::sync::{RwLock, mpsc::UnboundedSender};

pub type MaybeSender = Arc<RwLock<Option<UnboundedSender<ServerSend>>>>;

pub struct AppState {
    pub sender: MaybeSender,
    pub http_client: Client,
}
