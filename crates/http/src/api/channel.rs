use gilvave_core::{
    dto::channel::ChannelView, security::get_access_token, settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_server_channels(
        client: &Client,
        server_id: String,
    ) -> Result<Vec<ChannelView>, String> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers/{server_id}/channels"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| e.without_url().to_string())?
            .json::<Vec<ChannelView>>()
            .await
            .map_err(|e| e.without_url().to_string());
        if res.is_ok() {
            tracing::info!("{:?}", res.clone().unwrap());
        }
        res
    }
}
