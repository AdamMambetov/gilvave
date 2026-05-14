use gilvave_core::{
    dto::user::ProfileResponse,
    error::CoreError,
    security::{set_access_token, set_refresh_token},
};
use tauri::State;

use crate::{api::Api, state::AppState};

#[tauri::command]
pub async fn register(
    state: State<'_, AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<CoreError, ()> {
    match Api::register(&state.http_client, username, email, password).await {
        Ok(_) => Ok(CoreError::Ok),
        Err(err) => Ok(err),
    }
}

#[tauri::command]
pub async fn login(
    state: State<'_, AppState>,
    email: String,
    password: String,
) -> Result<CoreError, ()> {
    match Api::login(&state.http_client, email, password).await {
        Ok(res) => {
            set_access_token(&res.access_token);
            set_refresh_token(&res.refresh_token);
            Ok(CoreError::Ok)
        }
        Err(err) => Ok(err),
    }
}

#[tauri::command]
pub async fn get_profile(state: State<'_, AppState>) -> Result<ProfileResponse, String> {
    if let Ok(res) = Api::get_profile(&state.http_client).await {
        return Ok(res);
    }
    Err("error".to_string())
}
