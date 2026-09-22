use gilvave_core::{
    dto::user::{AuthTokensResponse, LoginRequest, RegisterRequest, UpdateTokensRequest, UserView},
    error::ErrorInfo,
    security::{get_access_token, get_refresh_token, set_access_token, set_refresh_token},
    settings::BASE_HTTP_URL,
};

use crate::http::api::{Api, Client};

impl Api {
    pub async fn register(
        _client: &Client,
        register_request: RegisterRequest,
    ) -> Result<(), ErrorInfo> {
        let body =
            serde_json::to_string(&register_request).map_err(|e| ErrorInfo(1, e.to_string()))?;
        let res = Api::request_raw(
            "POST",
            &format!("{BASE_HTTP_URL}/users/register"),
            Some(body),
            None,
        )
        .await?;
        Api::response_to_empty(res).await
    }

    pub async fn login(
        _client: &Client,
        request: LoginRequest,
    ) -> Result<AuthTokensResponse, ErrorInfo> {
        let body = serde_json::to_string(&request).map_err(|e| ErrorInfo(1, e.to_string()))?;
        let res = Api::request_raw(
            "POST",
            &format!("{BASE_HTTP_URL}/users/login"),
            Some(body),
            None,
        )
        .await?;
        Api::response_to::<AuthTokensResponse>(res).await
    }

    pub async fn update_tokens(_client: &Client) -> Result<(), ErrorInfo> {
        tracing::info!("update_tokens!");
        let json = UpdateTokensRequest {
            refresh_token: get_refresh_token(),
        };
        let body = serde_json::to_string(&json).map_err(|e| ErrorInfo(1, e.to_string()))?;
        let res = Api::request_raw(
            "POST",
            &format!("{BASE_HTTP_URL}/users/login"),
            Some(body),
            None,
        )
        .await?;

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

    pub async fn get_profile(_client: &Client) -> Result<UserView, ErrorInfo> {
        let token = get_access_token();
        let res = Api::request_raw(
            "GET",
            &format!("{BASE_HTTP_URL}/users/me"),
            None,
            Some(&token),
        )
        .await?;
        Api::response_to::<UserView>(res).await
    }
}
