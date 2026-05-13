use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTokensRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
}
