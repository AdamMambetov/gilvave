use serde::{Deserialize, Serialize};

use crate::{dto::message::MessageView, ids::ChannelId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerRecieve {
    Hello { heartbeat_interval: u64 },
    JoinSuccess,
    MessageNew(MessageView),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerSend {
    Heartbeat,
    MessageCreate {
        channel_id: ChannelId,
        content: String,
    },
    JoinChannel {
        channel_id: ChannelId,
    },
    LeftChannel {
        channel_id: ChannelId,
    },
    ChannelHistory {
        channel_id: ChannelId,
        #[serde(with = "time::serde::rfc3339")]
        from: time::OffsetDateTime,
    },
}
