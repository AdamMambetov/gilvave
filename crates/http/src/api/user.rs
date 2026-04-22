use gilvave_core::{
    dto::user::{
        AuthTokensResponse, LoginRequest, ProfileResponse, RegisterRequest, RegisterResponse,
        UpdateTokensRequest,
    },
    security::{get_access_token, get_refresh_token, set_access_token, set_refresh_token},
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn register(
        client: &Client,
        username: String,
        email: String,
        password: String,
    ) -> anyhow::Result<RegisterResponse> {
        let json = RegisterRequest {
            username,
            email,
            password,
        };
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/register"))
            .json(&json)
            .send()
            .await?;
        Ok(res.json::<RegisterResponse>().await?)
    }

    pub async fn login(
        client: &Client,
        email: String,
        password: String,
    ) -> anyhow::Result<AuthTokensResponse> {
        let json = LoginRequest { email, password };
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/login"))
            .json(&json)
            .send()
            .await?
            .json::<AuthTokensResponse>()
            .await?;

        Ok(res)
    }

    pub async fn update_tokens(client: &Client) -> anyhow::Result<AuthTokensResponse> {
        let json = UpdateTokensRequest {
            refresh_token: get_refresh_token(),
        };
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/login"))
            .json(&json)
            .send()
            .await?
            .json::<AuthTokensResponse>()
            .await?;
        set_access_token(&res.access_token);
        set_refresh_token(&res.refresh_token);
        Ok(res)
    }

    pub async fn get_profile(client: &Client) -> anyhow::Result<ProfileResponse> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/users/me"))
            .bearer_auth(get_access_token())
            .send()
            .await?
            .json::<ProfileResponse>()
            .await?;
        Ok(res)
    }
}
