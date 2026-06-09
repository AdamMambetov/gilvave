use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoreError {
    Ok,
    LoginFail(String),
    RegisterFail(String),
    GetProfileFail(String),
    GetMembersFail(String),
    GetServerChannelsFail(String),
    GetUserServersFail(String),
    CreateServerFail(String),
    ListenWebSocketFail(String),
    JoinChannelFail(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
