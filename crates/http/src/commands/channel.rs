use gilvave_core::dto::channel::ChannelView;
use tauri::State;

use crate::{api::Api, state::AppState};

#[tauri::command]
pub async fn get_server_channels(
    state: State<'_, AppState>,
    server_id: String,
) -> Result<Vec<ChannelView>, String> {
    Api::get_server_channels(&state.http_client, server_id).await
}
