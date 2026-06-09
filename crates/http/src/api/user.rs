use gilvave_core::{
    dto::user::{AuthTokensResponse, LoginRequest, RegisterRequest, UpdateTokensRequest, UserView},
    error::{CoreError, ErrorResponse},
    security::{get_access_token, get_refresh_token, set_access_token, set_refresh_token},
    settings::BASE_HTTP_URL,
};
use tauri_plugin_http::reqwest::Client;

use crate::api::Api;

impl Api {
    pub async fn register(
        client: &Client,
        register_request: RegisterRequest,
    ) -> Result<(), CoreError> {
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/register"))
            .json(&register_request)
            .send()
            .await
            .map_err(|err| CoreError::RegisterFail(err.to_string()))?;
        let status = res.status();
        let is_error = status.is_client_error() || status.is_server_error();

        if is_error {
            let error_text = res.json::<ErrorResponse>().await.unwrap().error;
            Err(CoreError::RegisterFail(error_text))
        } else {
            Ok(())
        }
    }

    pub async fn login(
        client: &Client,
        request: LoginRequest,
    ) -> Result<AuthTokensResponse, CoreError> {
        let res = client
            .post(format!("{BASE_HTTP_URL}/users/login"))
            .json(&request)
            .send()
            .await
            .map_err(|err| CoreError::LoginFail(err.to_string()))?;

        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            let error_text = res.json::<ErrorResponse>().await.unwrap().error;
            Err(CoreError::LoginFail(error_text))
        } else {
            Ok(res.json::<AuthTokensResponse>().await.unwrap())
        }
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

    pub async fn get_profile(client: &Client) -> Result<UserView, CoreError> {
        let res = client
            .get(format!("{BASE_HTTP_URL}/users/me"))
            .bearer_auth(get_access_token())
            .send()
            .await
            .map_err(|err| CoreError::GetProfileFail(err.to_string()))?;

        Ok(res
            .json::<UserView>()
            .await
            .map_err(|err| CoreError::GetProfileFail(err.to_string()))?)
    }
}
