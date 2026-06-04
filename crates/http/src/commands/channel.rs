use gilvave_core::dto::channel::ChannelView;
use gilvave_state::AppState;
use tauri::State;

use crate::api::Api;

#[tauri::command]
pub async fn get_server_channels(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<Vec<ChannelView>, String> {
    Api::get_server_channels(&state.http_client, server_id).await
}
