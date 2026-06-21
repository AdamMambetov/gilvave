use gilvave_core::{
    dto::channel::ChannelView, error::ErrorInfo, ids::ServerId, settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_server_channels(
        client: &Client,
        server_id: ServerId,
    ) -> Result<Vec<ChannelView>, ErrorInfo> {
        let res = client
            .get(format!(
                "{BASE_HTTP_URL}/servers/{}/channels",
                server_id.0.to_string()
            ))
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Vec<ChannelView>>(res).await
    }
}
