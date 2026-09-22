use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use gilvave_core::{
    dto::ws::ServerSend,
    ids::ChannelId,
};
use gilvave_gateway::service::WsService;
use time::OffsetDateTime;

#[tokio::test]
async fn test_join_channel_with_sender() {
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerSend>();
    let sender_ptr = Arc::new(RwLock::new(Some(tx)));
    let channel_id = ChannelId::default();

    let res = WsService::join_channel(sender_ptr, channel_id).await;
    assert!(res.is_ok());

    let received = rx.recv().await.expect("Expected message");
    match received {
        ServerSend::JoinChannel { channel_id: c } => assert_eq!(c, channel_id),
        _ => panic!("Expected JoinChannel variant"),
    }
}

#[tokio::test]
async fn test_join_channel_without_sender_fails() {
    let sender_ptr = Arc::new(RwLock::new(None));
    let channel_id = ChannelId::default();

    let res = WsService::join_channel(sender_ptr, channel_id).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().1, "Read websocket fail!");
}

#[tokio::test]
async fn test_left_channel_with_sender() {
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerSend>();
    let sender_ptr = Arc::new(RwLock::new(Some(tx)));
    let channel_id = ChannelId::default();

    let res = WsService::left_channel(sender_ptr, channel_id).await;
    assert!(res.is_ok());

    let received = rx.recv().await.expect("Expected message");
    match received {
        ServerSend::LeftChannel { channel_id: c } => assert_eq!(c, channel_id),
        _ => panic!("Expected LeftChannel variant"),
    }
}

#[tokio::test]
async fn test_message_create_with_sender() {
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerSend>();
    let sender_ptr = Arc::new(RwLock::new(Some(tx)));
    let channel_id = ChannelId::default();

    let res = WsService::message_create(sender_ptr, channel_id, "hello world".to_string()).await;
    assert!(res.is_ok());

    let received = rx.recv().await.expect("Expected message");
    match received {
        ServerSend::MessageCreate { channel_id: c, content } => {
            assert_eq!(c, channel_id);
            assert_eq!(content, "hello world");
        }
        _ => panic!("Expected MessageCreate variant"),
    }
}

#[tokio::test]
async fn test_channel_history_before_and_after() {
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerSend>();
    let sender_ptr = Arc::new(RwLock::new(Some(tx)));
    let channel_id = ChannelId::default();
    let now = OffsetDateTime::now_utc();

    let res_before = WsService::channel_history_before(sender_ptr.clone(), channel_id, now).await;
    assert!(res_before.is_ok());
    let received_before = rx.recv().await.expect("Expected message");
    match received_before {
        ServerSend::ChannelHistoryBefore { channel_id: c, timestamp } => {
            assert_eq!(c, channel_id);
            assert_eq!(timestamp, now);
        }
        _ => panic!("Expected ChannelHistoryBefore variant"),
    }

    let res_after = WsService::channel_history_after(sender_ptr.clone(), channel_id, now).await;
    assert!(res_after.is_ok());
    let received_after = rx.recv().await.expect("Expected message");
    match received_after {
        ServerSend::ChannelHistoryAfter { channel_id: c, timestamp } => {
            assert_eq!(c, channel_id);
            assert_eq!(timestamp, now);
        }
        _ => panic!("Expected ChannelHistoryAfter variant"),
    }
}
