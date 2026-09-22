use gilvave_core::{
    dto::channel::ChannelView, error::ErrorInfo, ids::ServerId, settings::BASE_HTTP_URL,
};

use crate::http::api::{Api, Client};

impl Api {
    pub async fn get_server_channels(
        _client: &Client,
        server_id: ServerId,
    ) -> Result<Vec<ChannelView>, ErrorInfo> {
        let res = Api::request_raw(
            "GET",
            &format!(
                "{BASE_HTTP_URL}/servers/{}/channels",
                server_id.0.to_string()
            ),
            None,
            None,
        )
        .await?;
        Api::response_to::<Vec<ChannelView>>(res).await
    }
}
