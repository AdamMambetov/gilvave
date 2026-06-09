use gilvave_core::{
    dto::command::{CommandArgs, CommandResponse, CommandResult},
    security::{set_access_token, set_refresh_token},
};
use gilvave_gateway::service::WsService;
use gilvave_http::api::Api;
use gilvave_state::AppState;
use tauri::{AppHandle, State};

macro_rules! dispatch {
    // $api_call:expr схватит функцию до первой запятой
    // $ok_mapper:expr схватит лямбду, которая упаковывает ответ
    ($command:expr, $( $variant:pat => $api_call:expr, $ok_mapper:expr ),+ $(,)?) => {
        match $command {
            $(
                $variant => $api_call.await
                    .map_or_else(CommandResult::Error, $ok_mapper)
            ),+
        }
    };
}

#[tauri::command]
pub async fn handle_command(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    command: CommandArgs,
) -> Result<CommandResult, ()> {
    let client = &state.http_client;
    Ok(dispatch! { command,
        CommandArgs::Register { request } =>
            Api::register(client, request),
            |_| CommandResult::Ok(CommandResponse::Register),
        CommandArgs::Login { request } =>
            Api::login(client, request),
            |r| {
                set_access_token(&r.access_token);
                set_refresh_token(&r.refresh_token);
                CommandResult::Ok(CommandResponse::Login(r))
            },
        CommandArgs::GetProfile =>
            Api::get_profile(client),
            |p| CommandResult::Ok(CommandResponse::GetProfile(p)),
        CommandArgs::GetMembers { server_id } =>
            Api::get_members(client, server_id),
            |m| CommandResult::Ok(CommandResponse::GetMembers(m)),
        CommandArgs::GetServerChannels { server_id } =>
            Api::get_server_channels(client, server_id),
            |c| CommandResult::Ok(CommandResponse::GetServerChannels(c)),
        CommandArgs::GetUserServers =>
            Api::get_user_servers(client),
            |s| CommandResult::Ok(CommandResponse::GetUserServers(s)),
        CommandArgs::CreateServer { server_info } =>
            Api::create_server(client, server_info),
            |s| CommandResult::Ok(CommandResponse::CreateServer(s)),
        CommandArgs::ListenWebSocket =>
            WsService::listen_web_socket(state.clone(), app_handle.clone()),
            |r| CommandResult::Ok(CommandResponse::ListenWebSocket(r)),
        CommandArgs::JoinChannel =>
            WsService::join_channel(state.sender.clone()),
            |_| CommandResult::Ok(CommandResponse::JoinChannel),
    })
}
