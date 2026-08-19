use gilvave_core::dto::ws::ServerSend;
use rusqlite::Connection;
use std::sync::Arc;
use tauri_plugin_http::reqwest::Client;
use tokio::sync::{Mutex, RwLock, mpsc::UnboundedSender};

pub type MaybeSender = Arc<RwLock<Option<UnboundedSender<ServerSend>>>>;
pub type MaybeDatabase = Arc<Mutex<Option<Connection>>>;

pub struct AppState {
    pub sender: MaybeSender,
    pub http_client: Client,
    pub db: MaybeDatabase,
}
