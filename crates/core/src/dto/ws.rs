use serde::{Deserialize, Serialize};

use crate::{dto::message::MessageView, ids::ChannelId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerRecieve {
    Hello,
    JoinSuccess,
    MessageNew(MessageView),
    ChannelHistoryBefore(Vec<MessageView>),
    ChannelHistoryAfter(Vec<MessageView>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerSend {
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
    ChannelHistoryBefore {
        channel_id: ChannelId,
        #[serde(with = "time::serde::rfc3339")]
        timestamp: time::OffsetDateTime,
    },
    ChannelHistoryAfter {
        channel_id: ChannelId,
        #[serde(with = "time::serde::rfc3339")]
        timestamp: time::OffsetDateTime,
    },
}
