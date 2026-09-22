use futures_util::AsyncReadExt;
use gilvave_core::error::{ErrorInfo, ErrorMessage};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_streams::ReadableStream;

pub mod channel;
pub mod server;
pub mod user;

#[derive(Clone, Default)]
pub struct Client;

pub struct Api;

impl Api {
    pub async fn request_raw(
        method: &str,
        url: &str,
        body: Option<String>,
        auth_token: Option<&str>,
    ) -> Result<web_sys::Response, ErrorInfo> {
        let window =
            web_sys::window().ok_or_else(|| ErrorInfo(1, "No window available".to_string()))?;
        let opts = web_sys::RequestInit::new();
        opts.set_method(method);

        let headers = web_sys::Headers::new().map_err(|e| ErrorInfo(1, format!("{e:?}")))?;
        headers
            .set("Accept", "application/json")
            .map_err(|e| ErrorInfo(1, format!("{e:?}")))?;
        if body.is_some() {
            headers
                .set("Content-Type", "application/json")
                .map_err(|e| ErrorInfo(1, format!("{e:?}")))?;
        }
        if let Some(token) = auth_token {
            if !token.is_empty() {
                headers
                    .set("Authorization", &format!("Bearer {token}"))
                    .map_err(|e| ErrorInfo(1, format!("{e:?}")))?;
            }
        }
        opts.set_headers(&headers);

        if let Some(ref b) = body {
            opts.set_body(&wasm_bindgen::JsValue::from_str(b));
        }

        let request = web_sys::Request::new_with_str_and_init(url, &opts)
            .map_err(|e| ErrorInfo(1, format!("{e:?}")))?;

        let promise = window.fetch_with_request(&request);
        let resp_value = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|e| ErrorInfo(1, format!("Fetch error: {e:?}")))?;

        let response: web_sys::Response = resp_value
            .dyn_into()
            .map_err(|e| ErrorInfo(1, format!("Invalid response: {e:?}")))?;

        Ok(response)
    }

    pub async fn response_to_bytes(response: web_sys::Response) -> Result<Vec<u8>, ErrorInfo> {
        if let Some(raw_stream) = response.body() {
            let stream = ReadableStream::from_raw(raw_stream);
            let mut async_reader = stream.into_async_read();
            let mut bytes = Vec::new();
            async_reader
                .read_to_end(&mut bytes)
                .await
                .map_err(|e| ErrorInfo(1, format!("Stream read error: {e}")))?;
            Ok(bytes)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn response_to_text(response: web_sys::Response) -> Result<String, ErrorInfo> {
        let bytes = Self::response_to_bytes(response).await?;
        String::from_utf8(bytes).map_err(|e| ErrorInfo(1, format!("UTF-8 decode error: {e}")))
    }

    pub async fn response_to<T: DeserializeOwned>(
        response: web_sys::Response,
    ) -> Result<T, ErrorInfo> {
        let status = response.status();
        if (200..=299).contains(&status) {
            let text = Self::response_to_text(response).await?;
            serde_json::from_str::<T>(&text)
                .map_err(|e| ErrorInfo(1, format!("JSON decode error: {e}")))
        } else {
            Err(Self::response_to_error(response).await)
        }
    }

    pub async fn response_to_empty(response: web_sys::Response) -> Result<(), ErrorInfo> {
        let status = response.status();
        if (200..=299).contains(&status) {
            Ok(())
        } else {
            Err(Self::response_to_error(response).await)
        }
    }

    async fn response_to_error(response: web_sys::Response) -> ErrorInfo {
        let status = response.status();
        let text = Self::response_to_text(response).await.unwrap_or_default();
        if let Ok(err_msg) = serde_json::from_str::<ErrorMessage>(&text) {
            ErrorInfo(status, err_msg.error)
        } else {
            ErrorInfo(status, text)
        }
    }
}
