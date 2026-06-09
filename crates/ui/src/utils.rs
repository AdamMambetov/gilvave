use gilvave_core::dto::command::CommandResult;
use serde_json::Value;
use tauri_sys::core::invoke;

pub async fn invoke_command(args: Value) -> CommandResult {
    invoke::<CommandResult>("handle_command", args).await
}
