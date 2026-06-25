use gilvave_core::{
    dto::user::{AuthTokensResponse, LoginRequest, RegisterRequest, UpdateTokensRequest, UserView},
    error::ErrorInfo,
    security::{get_access_token, get_refresh_token, set_access_token, set_refresh_token},
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn register(
        client: &Client,
        register_request: RegisterRequest,
    ) -> Result<(), ErrorInfo> {
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/register"))
            .json(&register_request)
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to_empty(res).await
    }

    pub async fn login(
        client: &Client,
        request: LoginRequest,
    ) -> Result<AuthTokensResponse, ErrorInfo> {
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/login"))
            .json(&request)
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;
        Api::response_to::<AuthTokensResponse>(res).await
    }

    pub async fn update_tokens(client: &Client) -> Result<(), ErrorInfo> {
        tracing::info!("update_tokens!");
        let json = UpdateTokensRequest {
            refresh_token: get_refresh_token(),
        };
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/login"))
            .json(&json)
            .send()
            .await
            .map_err(|e| ErrorInfo::default(e.without_url().to_string()))?;

        let res = Api::response_to::<AuthTokensResponse>(res).await;
        match res {
            Ok(r) => {
                set_access_token(&r.access_token);
                set_refresh_token(&r.refresh_token);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_profile(client: &Client) -> Result<UserView, ErrorInfo> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/users/me"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|e| ErrorInfo(1u16, e.to_string()))?;
        Api::response_to::<UserView>(res).await
    }
}
