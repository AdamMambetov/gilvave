use serde::{Deserialize, Serialize};

use crate::ids::UserId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub device_info: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTokensRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserView {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub avatar: String,
}
