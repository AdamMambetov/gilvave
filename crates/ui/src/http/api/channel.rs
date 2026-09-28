use gilvave_core::{
    dto::channel::{ChannelCreateInfo, ChannelView},
    error::ErrorInfo,
    ids::ServerId,
    settings::BASE_HTTP_URL,
};

use crate::http::api::Api;

impl Api {
    pub async fn get_server_channels(server_id: ServerId) -> Result<Vec<ChannelView>, ErrorInfo> {
        let token = Self::fetch_access_token().await;
        let res = Api::request_raw(
            "GET",
            &format!("{BASE_HTTP_URL}/servers/{server_id}/channels"),
            (),
            Some(&token),
        )
        .await?;
        Api::response_to::<Vec<ChannelView>>(res).await
    }

    pub async fn create_channel(
        server_id: ServerId,
        channel_info: ChannelCreateInfo,
    ) -> Result<ChannelView, ErrorInfo> {
        let token = Self::fetch_access_token().await;
        let res = Api::request_raw(
            "POST",
            &format!("{BASE_HTTP_URL}/servers/{server_id}/channels"),
            &channel_info,
            Some(&token),
        )
        .await?;
        Api::response_to::<ChannelView>(res).await
    }
}
