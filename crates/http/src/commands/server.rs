use gilvave_core::dto::server::ServerView;
use tauri::State;

use crate::{api::Api, state::AppState};

#[tauri::command]
pub async fn get_user_servers(state: State<'_, AppState>) -> Result<Vec<ServerView>, String> {
    Api::get_user_servers(&state.http_client).await
}
