# AGENTS.md — Gilvave

Tauri 2 + Sycamore (WASM) chat application. Rust workspace with 6 crates.

## Architecture

```
crates/
  core/      — DTOs, error types, ID generation (UUID v4/v7), security (keyring), settings
  gateway/   — WebSocket service (tokio-tungstenite), message handling
  http/      — HTTP API client (reqwest via tauri-plugin-http)
  state/     — AppState: shared WebSocket sender + HTTP client
  ui/        — Sycamore WASM frontend (Trunk build, port 1420)
  src-tauri/ — Tauri backend entry point, command dispatch handler
```

**Data flow**: UI (WASM) → `invoke("handle_command")` → handler.rs dispatch macro → http/gateway services → response back to UI.

**Command system**: All frontend commands go through `CommandArgs` enum → `handler::handle_command` → `dispatch!` macro. The macro auto-retries on 401 by calling `Api::update_tokens`.

## Key commands

```bash
# Dev (starts Trunk + Tauri dev)
cargo tauri dev

# Dev frontend only (WASM hot-reload on port 1420)
trunk serve --config ui/Trunk.toml

# Build frontend only
trunk build --release --config ui/Trunk.toml

# Rust build/check (backend only, skips UI)
cargo check
cargo build

# Run tests
cargo test --package gilvave-http        # HTTP crate tests
cargo test --package gilvave-core        # Core crate tests
cargo test --workspace                   # All workspace tests

# Lint
cargo clippy --workspace
```

## Crate dependency order

`core` → `state` → `gateway` / `http` → `src-tauri` (bottom-up; `core` has no internal deps).

## Gotchas

- **Linux WebKit env vars** are set in `main.rs`: `WEBKIT_DISABLE_DMABUF_RENDERER`, `WEBKIT_DISABLE_COMPOSITING_MODE`, `GDK_BACKEND=wayland`. Required for Tauri to render on Linux.
- **Trunk watch** ignores `src-tauri` and `core` (configured in `Trunk.toml`).
- **`.taurignore`** excludes `/src`, `/public`, `/Cargo.toml` from Tauri bundling.
- **`Cargo.lock` is gitignored** (workspace, not a library — unusual but intentional).
- **IDs are UUIDs** (v4 for ServerId/ChannelId, v7 for UserId/MessageId). Use `uuid::Uuid` for comparisons, not integers.
- **keyring** is used for token storage. `get_access_token()` / `get_refresh_token()` may panic if keyring backend is unavailable in test environments.
- **`BASE_HTTP_URL` / `BASE_WS_URL`** are hardcoded constants in `core/src/settings.rs` — no env var override.

## Testing notes

- Integration tests live in `crates/<name>/tests/`. Unit tests can be `#[cfg(test)]` modules inline.
- HTTP crate tests (`crates/http/tests/api_tests.rs`) test DTO serialization/deserialization only — no live server calls.
- `uuid` and `time` are dev-dependencies in `http/Cargo.toml` for test ID construction.

## Conventions

- Edition 2024, rust-version 1.96, resolver 3.
- All workspace deps declared in root `Cargo.toml`; crates reference with `.workspace = true`.
- Internal crates use `path = "../<name>"` dependencies.
- No `.github/` CI, no pre-commit hooks, no formatter config (uses rustfmt defaults).
