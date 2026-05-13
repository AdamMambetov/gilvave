use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CoreError {
    Ok,
    LoginFail,
    RegisterFail,
}

impl From<String> for CoreError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "LoginFail" => CoreError::LoginFail,
            _ => CoreError::Ok,
        }
    }
}
