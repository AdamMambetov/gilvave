use gilvave_core::{
    dto::server::ServerView, error::CoreError, security::get_access_token, settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_user_servers(client: &Client) -> Result<Vec<ServerView>, String> {
        client
            .get(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| e.without_url().to_string())?
            .json::<Vec<ServerView>>()
            .await
            .map_err(|e| e.without_url().to_string())
    }
}
