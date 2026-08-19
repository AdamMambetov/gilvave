use gilvave_core::{
    dto::{
        channel::{ChannelType, ChannelView},
        server::{MemberView, ServerCreateInfo, ServerView},
        user::{AuthTokensResponse, LoginRequest, RegisterRequest, UpdateTokensRequest, UserView},
    },
    error::ErrorInfo,
    ids::{ChannelId, ServerId},
    settings::DeviceInfo,
};
use uuid::Uuid;

const TEST_UUID: &str = "550e8400-e29b-41d4-a716-446655440000";
const TEST_UUID_2: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";

fn test_uuid() -> Uuid {
    Uuid::parse_str(TEST_UUID).unwrap()
}

fn test_uuid_2() -> Uuid {
    Uuid::parse_str(TEST_UUID_2).unwrap()
}

#[test]
fn test_error_info_default() {
    let err = ErrorInfo::default("service unavailable".to_string());
    assert_eq!(err.0, 503);
    assert_eq!(err.1, "service unavailable");
}

#[test]
fn test_error_info_custom() {
    let err = ErrorInfo(404, "not found".to_string());
    assert_eq!(err.0, 404);
    assert_eq!(err.1, "not found");
}

#[test]
fn test_error_info_clone() {
    let err = ErrorInfo(500, "server error".to_string());
    let cloned = err.clone();
    assert_eq!(err, cloned);
}

#[test]
fn test_register_request_serialize() {
    let req = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("testuser"));
    assert!(json.contains("test@example.com"));
}

#[test]
fn test_register_request_deserialize() {
    let json = r#"{"username":"testuser","email":"test@example.com","password":"password123"}"#;
    let req: RegisterRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.username, "testuser");
    assert_eq!(req.email, "test@example.com");
    assert_eq!(req.password, "password123");
}

#[test]
fn test_login_request_serialize() {
    let req = LoginRequest {
        email: "user@test.com".to_string(),
        password: "secret".to_string(),
        device_info: DeviceInfo::default().to_json(),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("user@test.com"));
}

#[test]
fn test_login_request_deserialize() {
    let json = r#"{"email":"user@test.com","password":"secret"}"#;
    let req: LoginRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.email, "user@test.com");
    assert_eq!(req.password, "secret");
}

#[test]
fn test_auth_tokens_response_deserialize() {
    let json = r#"{"access_token":"abc123","refresh_token":"xyz789"}"#;
    let res: AuthTokensResponse = serde_json::from_str(json).unwrap();
    assert_eq!(res.access_token, "abc123");
    assert_eq!(res.refresh_token, "xyz789");
}

#[test]
fn test_auth_tokens_response_serialize_roundtrip() {
    let res = AuthTokensResponse {
        access_token: "access123".to_string(),
        refresh_token: "refresh456".to_string(),
    };
    let json = serde_json::to_string(&res).unwrap();
    let parsed: AuthTokensResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(res.access_token, parsed.access_token);
    assert_eq!(res.refresh_token, parsed.refresh_token);
}

#[test]
fn test_update_tokens_request_serialize() {
    let req = UpdateTokensRequest {
        refresh_token: "refresh123".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("refresh123"));
}

#[test]
fn test_user_view_deserialize() {
    let json = format!(
        r#"{{"id":"{}","username":"alice","email":"alice@example.com","is_active":true,"avatar":"https://example.com/avatar.png"}}"#,
        TEST_UUID
    );
    let user: UserView = serde_json::from_str(&json).unwrap();
    assert_eq!(user.id.0, test_uuid());
    assert_eq!(user.username, "alice");
    assert!(user.is_active);
}

#[test]
fn test_user_view_inactive() {
    let json = format!(
        r#"{{"id":"{}","username":"bob","email":"bob@test.com","is_active":false,"avatar":""}}"#,
        TEST_UUID
    );
    let user: UserView = serde_json::from_str(&json).unwrap();
    assert!(!user.is_active);
}

#[test]
fn test_server_view_deserialize() {
    let json = format!(
        r#"{{"id":"{}","name":"My Server","icon_url":"https://example.com/icon.png","created_at":"2024-01-15T10:30:00Z"}}"#,
        TEST_UUID
    );
    let server: ServerView = serde_json::from_str(&json).unwrap();
    assert_eq!(server.id.0, test_uuid());
    assert_eq!(server.name, "My Server");
    assert!(server.icon_url.contains("icon.png"));
}

#[test]
fn test_server_create_info_serialize() {
    let info = ServerCreateInfo {
        name: "New Server".to_string(),
        icon_url: Some("https://example.com/icon.png".to_string()),
        is_public: true,
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("New Server"));
    assert!(json.contains("true"));
}

#[test]
fn test_server_create_info_no_icon() {
    let info = ServerCreateInfo {
        name: "Private Server".to_string(),
        icon_url: None,
        is_public: false,
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("Private Server"));
    assert!(json.contains("null"));
}

#[test]
fn test_member_view_deserialize() {
    let json = format!(
        r#"{{"user_id":"{}","username":"bob","avatar":"https://example.com/bob.png"}}"#,
        TEST_UUID
    );
    let member: MemberView = serde_json::from_str(&json).unwrap();
    assert_eq!(member.user_id.0, test_uuid());
    assert_eq!(member.username, "bob");
}

#[test]
fn test_channel_view_deserialize_text() {
    let json = format!(
        r#"{{"id":"{}","name":"general","type":"TEXT","position":0}}"#,
        TEST_UUID
    );
    let channel: ChannelView = serde_json::from_str(&json).unwrap();
    assert_eq!(channel.id.0, test_uuid());
    assert_eq!(channel.name, "general");
    assert_eq!(channel.r#type, ChannelType::TEXT);
    assert_eq!(channel.position, 0);
}

#[test]
fn test_channel_view_deserialize_voice() {
    let json = format!(
        r#"{{"id":"{}","name":"voice","type":"VOICE","position":1}}"#,
        TEST_UUID
    );
    let channel: ChannelView = serde_json::from_str(&json).unwrap();
    assert_eq!(channel.r#type, ChannelType::VOICE);
    assert_eq!(channel.position, 1);
}

#[test]
fn test_channel_type_invalid() {
    let json = format!(
        r#"{{"id":"{}","name":"bad","type":"INVALID","position":2}}"#,
        TEST_UUID
    );
    let result = serde_json::from_str::<ChannelView>(&json);
    assert!(result.is_err());
}

#[test]
fn test_user_view_debug() {
    let json = format!(
        r#"{{"id":"{}","username":"test","email":"test@test.com","is_active":false,"avatar":""}}"#,
        TEST_UUID
    );
    let user: UserView = serde_json::from_str(&json).unwrap();
    let debug_str = format!("{:?}", user);
    assert!(debug_str.contains("test"));
}

#[test]
fn test_server_view_eq() {
    let json = format!(
        r#"{{"id":"{}","name":"Server","icon_url":"","created_at":"2024-01-01T00:00:00Z"}}"#,
        TEST_UUID
    );
    let s1: ServerView = serde_json::from_str(&json).unwrap();
    let s2: ServerView = serde_json::from_str(&json).unwrap();
    assert_eq!(s1, s2);
}

#[test]
fn test_channel_view_eq() {
    let c1 = ChannelView {
        id: ChannelId(test_uuid()),
        name: "test".to_string(),
        r#type: ChannelType::TEXT,
        position: 0,
    };
    let c2 = ChannelView {
        id: ChannelId(test_uuid()),
        name: "test".to_string(),
        r#type: ChannelType::TEXT,
        position: 0,
    };
    assert_eq!(c1, c2);
}

#[test]
fn test_channel_view_neq() {
    let c1 = ChannelView {
        id: ChannelId(test_uuid()),
        name: "general".to_string(),
        r#type: ChannelType::TEXT,
        position: 0,
    };
    let c2 = ChannelView {
        id: ChannelId(test_uuid_2()),
        name: "general".to_string(),
        r#type: ChannelType::TEXT,
        position: 0,
    };
    assert_ne!(c1, c2);
}

#[test]
fn test_server_view_different_names() {
    let s1 = ServerView {
        id: ServerId(test_uuid()),
        name: "Server A".to_string(),
        icon_url: "".to_string(),
        created_at: time::OffsetDateTime::now_utc(),
        description: String::new(),
        member_count: 0,
    };
    let s2 = ServerView {
        id: ServerId(test_uuid()),
        name: "Server B".to_string(),
        icon_url: "".to_string(),
        created_at: time::OffsetDateTime::now_utc(),
        description: String::new(),
        member_count: 0,
    };
    assert_ne!(s1, s2);
}

#[test]
fn test_error_info_serialize() {
    let err = ErrorInfo(400, "bad request".to_string());
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("400"));
    assert!(json.contains("bad request"));
}

#[test]
fn test_error_info_deserialize() {
    let json = r#"[422,"validation failed"]"#;
    let err: ErrorInfo = serde_json::from_str(json).unwrap();
    assert_eq!(err.0, 422);
    assert_eq!(err.1, "validation failed");
}

#[test]
fn test_register_request_empty_fields() {
    let req = RegisterRequest {
        username: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };
    let json = serde_json::to_string(&req).unwrap();
    let parsed: RegisterRequest = serde_json::from_str(&json).unwrap();
    assert!(parsed.username.is_empty());
    assert!(parsed.email.is_empty());
    assert!(parsed.password.is_empty());
}

#[test]
fn test_login_request_special_characters() {
    let req = LoginRequest {
        email: "user+tag@domain.com".to_string(),
        password: "p@$$w0rd!#%".to_string(),
        device_info: DeviceInfo::default().to_json(),
    };
    let json = serde_json::to_string(&req).unwrap();
    let parsed: LoginRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.email, req.email);
    assert_eq!(parsed.password, req.password);
}

#[test]
fn test_auth_tokens_empty() {
    let res = AuthTokensResponse {
        access_token: "".to_string(),
        refresh_token: "".to_string(),
    };
    let json = serde_json::to_string(&res).unwrap();
    let parsed: AuthTokensResponse = serde_json::from_str(&json).unwrap();
    assert!(parsed.access_token.is_empty());
    assert!(parsed.refresh_token.is_empty());
}

#[test]
fn test_channel_view_positions() {
    let positions = vec![0, 1, 5, 100, -1];
    for pos in positions {
        let json = format!(
            r#"{{"id":"{}","name":"ch","type":"TEXT","position":{}}}"#,
            TEST_UUID, pos
        );
        let ch: ChannelView = serde_json::from_str(&json).unwrap();
        assert_eq!(ch.position, pos);
    }
}
