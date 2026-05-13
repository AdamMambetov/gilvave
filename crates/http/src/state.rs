use tauri_plugin_http::reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    // ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub http_client: Client,
}
