use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    dto::{
        channel::ChannelView,
        server::{MemberView, ServerCreateInfo, ServerView},
        user::{AuthTokensResponse, LoginRequest, RegisterRequest, UserView},
    },
    error::ErrorInfo,
    ids::{ChannelId, ServerId},
};

#[derive(Serialize, Deserialize)]
pub enum CommandArgs {
    Register {
        request: RegisterRequest,
    },
    Login {
        request: LoginRequest,
    },
    GetProfile,
    GetMembers {
        server_id: ServerId,
    },
    GetServerChannels {
        server_id: ServerId,
    },
    GetUserServers,
    CreateServer {
        server_info: ServerCreateInfo,
    },
    ListenWebSocket,
    JoinChannel {
        channel_id: ChannelId,
    },
    LeftChannel {
        channel_id: ChannelId,
    },
    MessageCreate {
        channel_id: ChannelId,
        content: String,
    },
    ChannelHistoryBefore {
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    },
    ChannelHistoryAfter {
        channel_id: ChannelId,
        timestamp: time::OffsetDateTime,
    },
}

impl CommandArgs {
    pub fn to_json(self) -> Value {
        json!({"command": self})
    }
}

#[derive(Serialize, Deserialize)]
pub enum CommandResponse {
    Register,
    Login(AuthTokensResponse),
    GetProfile(UserView),
    GetMembers(Vec<MemberView>),
    GetServerChannels(Vec<ChannelView>),
    GetUserServers(Vec<ServerView>),
    CreateServer(ServerView),
    ListenWebSocket(bool),
    JoinChannel,
    LeftChannel,
    MessageCreate,
    ChannelHistoryBefore,
    ChannelHistoryAfter,
}

#[derive(Serialize, Deserialize)]
pub enum CommandResult {
    Ok(CommandResponse),
    Error(ErrorInfo),
}

impl CommandResult {
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Error(_) => false,
        }
    }
}
