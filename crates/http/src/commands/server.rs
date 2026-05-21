use gilvave_core::dto::server::{MemberView, ServerView};
use tauri::State;

use crate::{api::Api, state::AppState};

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
