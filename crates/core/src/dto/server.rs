use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::{ServerId, UserId};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServerView {
    pub id: ServerId,
    pub name: String,
    pub icon_url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub member_count: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MemberView {
    pub user_id: UserId,
    pub username: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServerCreateInfo {
    pub name: String,
    pub icon_url: Option<String>,
    pub is_public: bool,
}
