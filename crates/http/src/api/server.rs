use gilvave_core::{
    dto::server::{MemberView, ServerCreateInfo, ServerView},
    error::CoreError,
    ids::ServerId,
    security::get_access_token,
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_user_servers(client: &Client) -> Result<Vec<ServerView>, CoreError> {
        client
            .get(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| CoreError::GetUserServersFail(e.without_url().to_string()))?
            .json::<Vec<ServerView>>()
            .await
            .map_err(|e| CoreError::GetUserServersFail(e.without_url().to_string()))
    }

    pub async fn get_members(
        client: &Client,
        server_id: ServerId,
    ) -> Result<Vec<MemberView>, CoreError> {
        client
            .get(format!(
                "{BASE_HTTP_URL}/servers/{}/members",
                server_id.0.to_string(),
            ))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| CoreError::GetMembersFail(e.without_url().to_string()))?
            .json::<Vec<MemberView>>()
            .await
            .map_err(|e| CoreError::GetMembersFail(e.without_url().to_string()))
    }

    pub async fn create_server(
        client: &Client,
        server_info: ServerCreateInfo,
    ) -> Result<ServerView, CoreError> {
        client
            .post(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .json(&server_info)
            .send()
            .await
            .map_err(|e| CoreError::CreateServerFail(e.without_url().to_string()))?
            .json::<ServerView>()
            .await
            .map_err(|e| CoreError::CreateServerFail(e.without_url().to_string()))
    }
}
