use gilvave_core::{
    dto::command::{CommandArgs, CommandResponse, CommandResult},
    error::ErrorInfo,
    security::{set_access_token, set_refresh_token},
};
use gilvave_gateway::service::WsService;
use gilvave_http::api::Api;
use gilvave_state::AppState;
use tauri::{AppHandle, State, async_runtime::block_on};

macro_rules! dispatch {
    // $api_call:expr схватит функцию до первой запятой
    // $ok_mapper:expr схватит лямбду, которая упаковывает ответ
    ($command:expr, $client:expr, $( $variant:pat => $api_call:expr, $ok_mapper:expr ),+ $(,)?) => {
        match $command {
            $(
                $variant => $api_call.await
                    .map_or_else(|e: ErrorInfo| {
                        tracing::info!("{e:?}");
                        if e.0 == 401 {
                            _ = block_on(Api::update_tokens($client));
                            return block_on($api_call).map_or_else(
                                |e2: ErrorInfo| { CommandResult::Error(e2) },
                                $ok_mapper,
                            )
                        }
                        CommandResult::Error(e)
                    }, $ok_mapper)
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
    Ok(dispatch! { command, client,
        CommandArgs::Register { request } =>
            Api::register(client, request.clone()),
            |_| CommandResult::Ok(CommandResponse::Register),
        CommandArgs::Login { request } =>
            Api::login(client, request.clone()),
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
            Api::create_server(client, server_info.clone()),
            |s| CommandResult::Ok(CommandResponse::CreateServer(s)),
        CommandArgs::ListenWebSocket =>
            WsService::listen_web_socket(state.clone(), app_handle.clone()),
            |r| CommandResult::Ok(CommandResponse::ListenWebSocket(r)),
        CommandArgs::JoinChannel { channel_id } =>
            WsService::join_channel(state.sender.clone(), channel_id),
            |_| CommandResult::Ok(CommandResponse::JoinChannel),
        CommandArgs::MessageCreate { channel_id, content } =>
            WsService::message_create(state.sender.clone(), channel_id, content.clone()),
            |_| CommandResult::Ok(CommandResponse::MessageCreate),
    })
}
