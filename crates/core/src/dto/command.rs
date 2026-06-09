use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    dto::{
        channel::ChannelView,
        server::{MemberView, ServerCreateInfo, ServerView},
        user::{AuthTokensResponse, LoginRequest, RegisterRequest, UserView},
    },
    error::CoreError,
    ids::ServerId,
};

#[derive(Serialize, Deserialize)]
pub enum CommandArgs {
    Register { request: RegisterRequest },
    Login { request: LoginRequest },
    GetProfile,
    GetMembers { server_id: ServerId },
    GetServerChannels { server_id: ServerId },
    GetUserServers,
    CreateServer { server_info: ServerCreateInfo },
    ListenWebSocket,
    JoinChannel,
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
}

#[derive(Serialize, Deserialize)]
pub enum CommandResult {
    Ok(CommandResponse),
    Error(CoreError),
}

impl CommandResult {
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            Self::Error(_) => false,
        }
    }
}
