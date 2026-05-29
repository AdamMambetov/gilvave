use gilvave_core::{
    dto::server::{MemberView, ServerCreateInfo, ServerView},
    security::get_access_token,
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_user_servers(client: &Client) -> Result<Vec<ServerView>, String> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| e.without_url().to_string())?
            .json::<Vec<ServerView>>()
            .await
            .map_err(|e| e.without_url().to_string());
        if res.is_ok() {
            tracing::info!("{:?}", res.clone().unwrap());
        }
        res
    }

    pub async fn get_members(
        client: &Client,
        server_id: String,
    ) -> Result<Vec<MemberView>, String> {
        client
            .get(format!("{BASE_HTTP_URL}/servers/{server_id}/members"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| e.without_url().to_string())?
            .json::<Vec<MemberView>>()
            .await
            .map_err(|e| e.without_url().to_string())
    }

    pub async fn create_server(
        client: &Client,
        server_info: ServerCreateInfo,
    ) -> Result<ServerView, String> {
        client
            .post(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .json(&server_info)
            .send()
            .await
            .map_err(|e| e.without_url().to_string())?
            .json::<ServerView>()
            .await
            .map_err(|e| e.without_url().to_string())
    }
}
