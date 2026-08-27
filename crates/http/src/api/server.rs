use gilvave_core::{
    dto::server::{MemberView, Server, ServerCreateInfo, ServerSmallPart},
    error::ErrorInfo,
    ids::ServerId,
    security::get_access_token,
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn get_user_servers(client: &Client) -> Result<Vec<ServerSmallPart>, ErrorInfo> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Vec<ServerSmallPart>>(res).await
    }

    pub async fn get_public_servers(client: &Client) -> Result<Vec<Server>, ErrorInfo> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers/public"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Vec<Server>>(res).await
    }

    pub async fn get_members(
        client: &Client,
        server_id: ServerId,
    ) -> Result<Vec<MemberView>, ErrorInfo> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers/{server_id}/members"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Vec<MemberView>>(res).await
    }

    pub async fn create_server(
        client: &Client,
        server_info: ServerCreateInfo,
    ) -> Result<Server, ErrorInfo> {
        let res = client
            .post(format!("{BASE_HTTP_URL}/servers"))
            .bearer_auth(get_access_token())
            .json(&server_info)
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Server>(res).await
    }

    pub async fn get_server_by_id(
        client: &Client,
        server_id: ServerId,
    ) -> Result<Server, ErrorInfo> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/servers/{server_id}"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<Server>(res).await
    }
}
