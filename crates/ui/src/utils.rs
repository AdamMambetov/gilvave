use gilvave_core::dto::command::CommandResult;
use serde_json::Value;
#[cfg(not(target_os = "unknown"))]
use tauri_sys::core::invoke;

#[cfg(not(target_os = "unknown"))]
pub async fn invoke_command(args: Value) -> CommandResult {
    invoke::<CommandResult>("handle_command", args).await
}

#[cfg(target_os = "unknown")]
pub async fn invoke_command(args: Value) -> CommandResult {
    handle_command_web(args).await
}

// #[cfg(target_os = "unknown")]
#[rustfmt::skip]
async fn handle_command_web(args: Value) -> CommandResult {
    use crate::{gateway::service::WsService, http::api::Api};
    use gilvave_core::{
        dto::{
            command::{CommandArgs, CommandResponse},
            user::AuthTokensResponse,
        },
        error::ErrorInfo,
        security::{set_access_token, set_refresh_token},
    };

    let command: CommandArgs = match args.get("command") {
        Some(cmd_val) => match serde_json::from_value(cmd_val.clone()) {
            Ok(c) => c,
            Err(e) => {
                return CommandResult::Error(ErrorInfo(
                    1,
                    format!("Failed to deserialize command: {e}"),
                ));
            }
        },
        None => match serde_json::from_value(args) {
            Ok(c) => c,
            Err(e) => {
                return CommandResult::Error(ErrorInfo(
                    1,
                    format!("Failed to deserialize command: {e}"),
                ));
            }
        },
    };

    macro_rules! dispatch {
        ($api_call:expr, $ok_mapper:expr) => {
            match $api_call.await {
                Ok(val) => CommandResult::Ok($ok_mapper(val)),
                Err(e) => {
                    tracing::info!("{e:?}");
                    if e.0 == 401 {
                        let update_res = Api::update_tokens().await;
                        if update_res.is_ok() {
                            match $api_call.await {
                                Ok(val2) => CommandResult::Ok($ok_mapper(val2)),
                                Err(e2) => CommandResult::Error(e2),
                            }
                        } else {
                            CommandResult::Error(e)
                        }
                    } else {
                        CommandResult::Error(e)
                    }
                }
            }
        };
    }

    match command {
        // ========================= Http Handlers ========================= //
        CommandArgs::Register { request } => dispatch!(
            Api::register(request.clone()), 
            |_| CommandResponse::Register
        ),
        CommandArgs::Login { request } => dispatch!(
            Api::login(request.clone()), 
            |r: AuthTokensResponse| {
                set_access_token(&r.access_token);
                set_refresh_token(&r.refresh_token);
                CommandResponse::Login(r)
            }
        ),
        CommandArgs::GetProfile => dispatch!(
            Api::get_profile(), 
            |p| CommandResponse::GetProfile(p)
        ),
        CommandArgs::GetMembers { server_id } => dispatch!(
            Api::get_members(server_id), 
            |m| CommandResponse::GetMembers(m)
        ),
        CommandArgs::GetServerChannels { server_id } => dispatch!(
            Api::get_server_channels(server_id), 
            |c| CommandResponse::GetServerChannels(c)
        ),
        CommandArgs::GetServerById { server_id } => dispatch!(
            Api::get_server_by_id(server_id), 
            |s| CommandResponse::GetServerById(s)
        ),
        CommandArgs::GetUserServers => dispatch!(
            Api::get_user_servers(), 
            |s| CommandResponse::GetUserServers(s)
        ),
        CommandArgs::GetPublicServers { page } => dispatch!(
            Api::get_public_servers(page), 
            |s| CommandResponse::GetPublicServers(s)
        ),
        CommandArgs::CreateServer { server_info } => dispatch!(
            Api::create_server(server_info.clone()), 
            |s| CommandResponse::CreateServer(s)
        ),
        CommandArgs::JoinPublicServer { server_id } => dispatch!(
            Api::join_public_server(server_id),
            |_| CommandResponse::JoinPublicServer
        ),

        // ========================= Gateway Handlers ========================= //
        CommandArgs::ListenWebSocket => dispatch!(
            WsService::listen_web_socket(), 
            |r| CommandResponse::ListenWebSocket(r)
        ),
        CommandArgs::JoinChannel { channel_id } => dispatch!(
            WsService::join_channel(channel_id),
            |_| CommandResponse::JoinChannel
        ),
        CommandArgs::LeftChannel { channel_id } => dispatch!(
            WsService::left_channel(channel_id),
            |_| CommandResponse::LeftChannel
        ),
        CommandArgs::MessageCreate {
            channel_id,
            content,
        } => dispatch!(
            WsService::message_create(channel_id, content.clone()),
            |_| CommandResponse::MessageCreate
        ),
        CommandArgs::ChannelHistoryBefore {
            channel_id,
            timestamp,
        } => dispatch!(
            WsService::channel_history_before(channel_id, timestamp),
            |_| CommandResponse::ChannelHistoryBefore
        ),
        CommandArgs::ChannelHistoryAfter {
            channel_id,
            timestamp,
        } => dispatch!(
            WsService::channel_history_after(channel_id, timestamp),
            |_| CommandResponse::ChannelHistoryAfter
        ),
    }
}

pub fn get_local_offset() -> time::UtcOffset {
    #[cfg(target_arch = "wasm32")]
    {
        let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i32;
        time::UtcOffset::from_whole_seconds(-offset_minutes * 60).unwrap_or(time::UtcOffset::UTC)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        time::UtcOffset::UTC
    }
}

pub fn to_local_datetime(dt: time::OffsetDateTime) -> time::OffsetDateTime {
    dt.to_offset(get_local_offset())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gilvave_core::dto::command::CommandArgs;
    use time::OffsetDateTime;

    #[test]
    fn test_to_local_datetime() {
        let dt = OffsetDateTime::now_utc();
        let local = to_local_datetime(dt);
        assert_eq!(dt.unix_timestamp(), local.unix_timestamp());
    }

    #[test]
    fn test_command_args_json_roundtrip() {
        let cmd = CommandArgs::GetProfile;
        let json = cmd.to_json();
        assert!(json.get("command").is_some());
        let parsed: CommandArgs = serde_json::from_value(json["command"].clone()).unwrap();
        assert!(matches!(parsed, CommandArgs::GetProfile));
    }

    #[test]
    fn test_event_message_view_deserialization() {
        use gilvave_core::dto::message::MessageView;
        use gilvave_core::ids::{ChannelId, MessageId, UserId};

        let view = MessageView {
            id: MessageId::default(),
            channel_id: ChannelId::default(),
            author_id: Some(UserId::default()),
            author_name: "test_user".to_string(),
            content: "hello world".to_string(),
            created_at: OffsetDateTime::now_utc(),
        };
        let payload_json = serde_json::to_value(&view).unwrap();
        let event_json = serde_json::json!({
            "event": "message_new",
            "id": 1,
            "payload": payload_json
        });
        let raw_str = serde_json::to_string(&event_json).unwrap();
        let deserialized: tauri_sys::event::Event<MessageView> =
            serde_json::from_str(&raw_str).unwrap();
        assert_eq!(deserialized.event, "message_new");
        assert_eq!(deserialized.id, 1);
        assert_eq!(deserialized.payload, view);
    }
}
