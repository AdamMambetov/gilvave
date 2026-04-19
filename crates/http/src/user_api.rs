use gilvave_core::dto::user::{RegisterRequest, RegisterResponse};
use tauri_plugin_http::reqwest::Client;

async fn _register(
    username: String,
    email: String,
    password: String,
) -> anyhow::Result<RegisterResponse> {
    let json = RegisterRequest {
        username,
        email,
        password,
    };
    let client = Client::new();
    let res = client
        .post("http://26.186.139.15:3000/users/register")
        .json(&json)
        .send()
        .await?;
    Ok(res.json::<RegisterResponse>().await?)
}

#[tauri::command]
pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Result<RegisterResponse, String> {
    if let Ok(res) = _register(username, email, password).await {
        return Ok(res);
    }
    Err("error".to_string())
}
