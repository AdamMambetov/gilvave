use gilvave_core::dto::command::CommandResult;
use serde_json::Value;
use tauri_sys::core::invoke;

pub async fn invoke_command(args: Value) -> CommandResult {
    invoke::<CommandResult>("handle_command", args).await
}

pub fn get_local_offset() -> time::UtcOffset {
    #[cfg(target_arch = "wasm32")]
    {
        let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i32;
        time::UtcOffset::from_whole_seconds(-offset_minutes * 60).unwrap_or(time::UtcOffset::UTC)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        time::UtcOffset::UTC
    }
}

pub fn to_local_datetime(dt: time::OffsetDateTime) -> time::OffsetDateTime {
    dt.to_offset(get_local_offset())
}
