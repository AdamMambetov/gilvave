use gilvave_core::{
    dto::user::{AuthTokensResponse, ProfileResponse, RegisterResponse},
    error::CoreError,
    security::{set_access_token, set_refresh_token},
};
use tauri::State;
use tauri_plugin_http::Error;

use crate::{api::Api, state::AppState};

#[tauri::command]
pub async fn register(
    state: State<'_, AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<RegisterResponse, String> {
    if let Ok(res) = Api::register(&state.http_client, username, email, password).await {
        return Ok(res);
    }
    Err("error".to_string())
}

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    email: String,
    password: String,
) -> Result<CoreError, ()> {
    if let Ok(res) = Api::login(&state.http_client, email, password).await {
        set_access_token(&res.access_token);
        set_refresh_token(&res.refresh_token);
        return Ok(CoreError::Ok);
    }
    Ok(CoreError::LoginFail)
}

#[tauri::command]
pub async fn get_profile(state: State<'_, AppState>) -> Result<ProfileResponse, String> {
    if let Ok(res) = Api::get_profile(&state.http_client).await {
        return Ok(res);
    }
    Err("error".to_string())
}
