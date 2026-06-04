use gilvave_core::dto::server::{MemberView, ServerCreateInfo, ServerView};
use gilvave_state::AppState;
use tauri::State;

use crate::api::Api;

#[tauri::command]
pub async fn get_user_servers(state: State<'_, AppState>) -> Result<Vec<ServerView>, String> {
    Api::get_user_servers(&state.http_client).await
}

#[tauri::command]
pub async fn get_members(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<Vec<MemberView>, String> {
    Api::get_members(&state.http_client, server_id).await
}

#[tauri::command]
pub async fn create_server(
    state: State<'_, AppState>,
    name: String,
    icon_url: Option<String>,
    is_public: bool,
) -> Result<ServerView, String> {
    Api::create_server(
        &state.http_client,
        ServerCreateInfo {
            name,
            icon_url,
            is_public,
        },
    )
    .await
}
