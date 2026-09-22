use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc};
use gilvave_core::{dto::ws::ServerSend, ids::ChannelId};
use gilvave_state::AppState;
use rusqlite::Connection;
use tauri_plugin_http::reqwest::Client;

#[tokio::test]
async fn test_app_state_initialization() {
    let state = AppState {
        sender: Arc::new(RwLock::new(None)),
        http_client: Client::new(),
        db: Arc::new(Mutex::new(None)),
    };

    assert!(state.sender.read().await.is_none());
    assert!(state.db.lock().await.is_none());
}

#[tokio::test]
async fn test_app_state_sender_flow() {
    let state = AppState {
        sender: Arc::new(RwLock::new(None)),
        http_client: Client::new(),
        db: Arc::new(Mutex::new(None)),
    };

    let (tx, mut rx) = mpsc::unbounded_channel::<ServerSend>();
    {
        let mut sender_lock = state.sender.write().await;
        *sender_lock = Some(tx);
    }

    let channel_id = ChannelId::default();
    {
        let sender_lock = state.sender.read().await;
        let sender = sender_lock.as_ref().unwrap();
        sender.send(ServerSend::JoinChannel { channel_id }).unwrap();
    }

    let msg = rx.recv().await.unwrap();
    match msg {
        ServerSend::JoinChannel { channel_id: c } => assert_eq!(c, channel_id),
        _ => panic!("Expected JoinChannel"),
    }
}

#[tokio::test]
async fn test_app_state_db_access() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("CREATE TABLE test (val TEXT)", ()).unwrap();
    conn.execute("INSERT INTO test (val) VALUES ('hello')", ()).unwrap();

    let state = AppState {
        sender: Arc::new(RwLock::new(None)),
        http_client: Client::new(),
        db: Arc::new(Mutex::new(Some(conn))),
    };

    let db_lock = state.db.lock().await;
    let conn_ref = db_lock.as_ref().unwrap();
    let val: String = conn_ref.query_row("SELECT val FROM test", (), |r| r.get(0)).unwrap();
    assert_eq!(val, "hello");
}
