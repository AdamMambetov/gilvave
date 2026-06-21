use gilvave_core::error::{ErrorInfo, ErrorMessage};
use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest::Response;

pub mod channel;
pub mod server;
pub mod user;

pub struct Api;

impl Api {
    pub async fn response_to<T: DeserializeOwned>(response: Response) -> Result<T, ErrorInfo> {
        let status = response.status();
        if status.is_success() {
            return response
                .json::<T>()
                .await
                .map_err(|e| ErrorInfo(1u16, e.without_url().to_string()));
        }
        Err(Api::response_to_error(response).await)
    }

    pub async fn response_to_empty(response: Response) -> Result<(), ErrorInfo> {
        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        Err(Api::response_to_error(response).await)
    }

    async fn response_to_error(response: Response) -> ErrorInfo {
        let status = response.status();
        if status.is_redirection() {
            ErrorInfo(status.as_u16(), response.text().await.unwrap_or_default())
        } else {
            let err = response
                .json::<ErrorMessage>()
                .await
                .map_err(|e| ErrorInfo(1u16, e.without_url().to_string()));
            match err {
                Ok(e) => ErrorInfo(status.as_u16(), e.error),
                Err(e) => e,
            }
        }
    }
}
