use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::ids::ServerId;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServerView {
    pub id: ServerId,
    pub name: String,
    pub icon_url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
