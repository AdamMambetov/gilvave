use gilvave_core::{
    dto::channel::ChannelView, error::CoreError, ids::ServerId, security::get_access_token,
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_server_channels(
        client: &Client,
        server_id: ServerId,
    ) -> Result<Vec<ChannelView>, CoreError> {
        client
            .get(format!(
                "{BASE_HTTP_URL}/servers/{}/channels",
                server_id.0.to_string()
            ))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| CoreError::GetServerChannelsFail(e.without_url().to_string()))?
            .json::<Vec<ChannelView>>()
            .await
            .map_err(|e| CoreError::GetServerChannelsFail(e.without_url().to_string()))
    }
}
