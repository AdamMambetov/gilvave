# AGENTS.md — Gilvave

Tauri 2 + Sycamore (WASM) chat application. Rust workspace with 3 crates.

## Architecture

```
crates/
  core/      — DTOs, error types, ID generation (UUID v4/v7), validation, desktop security (keyring), settings
  ui/        — Sycamore WASM frontend (Trunk build, port 1420), direct HTTP/WS client, web security (cookies)
  src-tauri/ — Tauri backend entry point, platform command handler (`handle_command`), local SQLite db
```

**Data flow (HTTP & WebSocket)**: UI components (`crates/ui`) call `Api::*` (`crates/ui/src/http/api/`) and `WsService::*` (`crates/ui/src/gateway/`) directly in WASM without routing network requests through Tauri IPC. `Api::request_raw` uses `RequestCredentials::Include` and automatically retries on `401` by calling `Api::update_tokens` (`POST /users/refresh`).

**Command system (`invoke_command`)**: Only platform-dependent operations where Web and Desktop behavior differs (`GetAccessToken`, `GetRefreshToken`, `SetAccessToken`, `SetRefreshToken`, `GetDeviceInfo`, `WindowMinimize`, `WindowToggleMaximize`, `WindowClose`, `WindowStartDragging`, and future native features like system audio capture) pass through `CommandArgs` (`core/src/dto/command.rs`) → `invoke_command` (`ui/src/utils.rs`):
- **Desktop (`#[cfg(not(target_os = "unknown"))]`)**: `invoke_command` calls Tauri `handle_command` (`src-tauri/src/handler.rs`), accessing OS `keyring`, native `sysinfo` (`client: "desktop"`), and native window controls.
- **Web (`#[cfg(target_os = "unknown")]`)**: `invoke_command` calls `handle_command_web` (`ui/src/utils.rs`), accessing browser cookies (`ui/src/security.rs`) and `woothee` user-agent parsing (`client: "web"`).
- **Token storage split**: On Desktop, the server returns tokens in the JSON body (`AuthTokensResponse`) and the client stores them in the OS `keyring` via `invoke_command`. On Web, the server sets `HttpOnly` cookies via `Set-Cookie` (sent automatically with `RequestCredentials::Include`), with `ui/src/security.rs` providing cookie access when needed.

---

## Key commands

```bash
# Dev (starts Trunk + Tauri dev)
cargo tauri dev

# Dev frontend only (WASM hot-reload on port 1420)
trunk serve --config crates/ui/Trunk.toml

# Build frontend only (WASM release)
trunk build --release --config crates/ui/Trunk.toml

# Rust check & build (backend only, skips UI)
cargo check
cargo build

# Rust check UI (WASM target)
cargo check --package gilvave-ui --target wasm32-unknown-unknown

# Run tests
cargo test --package gilvave-core        # Core & DTO tests
cargo test --workspace                   # All workspace tests

# Lint
cargo clippy --workspace
```

---

## Crate dependency order

`core` → `ui` / `src-tauri` (`core` has no internal deps).
`ui` and `src-tauri` both depend on `core` (shared DTOs, IDs, validation, settings) and communicate exclusively via `invoke_command` (`utils.rs`) & event listeners.

---

## UI Architecture (`crates/ui`)

Sycamore 0.9 + Trunk single-page WASM application styled with modular SCSS.

### Directory Structure

```
crates/ui/src/
  app.rs                    — Root App component, session check, view switching
  main.rs                   — WASM entry point (mounts to DOM, console error hook)
  security.rs               — Browser cookie helpers (get/set access & refresh tokens)
  utils.rs                  — Unified invoke_command (Tauri IPC vs Web), local timezone conversion
  http/                     — Direct WASM HTTP client (web-sys fetch, auto-401 refresh)
    api/                    — Domain endpoints (user, server, channel)
  gateway/                  — Direct WASM WebSocket client (ws_stream_wasm) & event dispatcher
  components/
    common/                 — Shared reactive contexts, mixins, variables, animations, reset
      contexts.rs           — ScreenWrapper, UserProfileContext, ServerContext, ChannelContext,
                              UiModalContext, CreateServerContext
      _variables.scss       — Color palette, typography, breakpoints ($bp-sm: 480px, $bp-md: 768px, etc.)
      _mixins.scss          — Media queries (@include respond-to), custom scrollbars, glassmorphism
      _animations.scss      — Fade, scale, slide, pulse keyframe animations
      _reset.scss           — CSS reset, font definitions, box-sizing
    layout/                 — App shell, window header
      header.rs             — Desktop drag region, custom window control buttons (minimize, close)
    pages/                  — Full-page screens
      home_panel.rs         — Authenticated workspace container (sidebar, channels, chat, members, modals)
      login_panel.rs        — Login form with validation & token persistence
      register_panel.rs     — Registration form with client-side field validation
    features/               — Feature domains
      servers/              — Server sidebar, server actions, create/join/settings modals
      channels/             — Channel list (text/voice), user status bar, create channel modal
      chat/                 — Message list, bubble items, day dividers, message input, scrolling
      members/              — Server member list (online/offline groups, roles, avatars)
      home/                 — Home dashboard (greeting hero, quick actions, DM chat, friends online)
      profile/              — User profile settings modal (preview, bio, credentials, security)
      auth/                 — Social buttons, auth form cards
    ui/                     — Atomic reusable UI elements (buttons, inputs, spinners, dividers, tooltips, icons)
```

### Reactive Contexts (`components/common/contexts.rs`)

1. **`ScreenWrapper`**: Controls active screen state (`ActiveScreen::Login`, `ActiveScreen::Register`, `ActiveScreen::Home`).
2. **`UserProfileContext`**: Active user's profile (`username`, `avatar`, `banner`, `bio`, `is_muted`, `is_deafened`).
3. **`ServerContext`**: Currently selected server (`current: Option<Server>`), joined servers list (`list: Vec<ServerSmallPart>`), and server members (`members: Vec<MemberView>`).
4. **`ChannelContext`**: Text channels (`text`), voice channels (`voice`), currently selected channel (`current: Option<ChannelView>`), and loaded message stream (`messages: Vec<MessageView>`).
5. **`UiModalContext`**: UI modal visibility signals (`is_server_settings_open`, `is_create_channel_open`, `create_channel_type`, `is_profile_settings_open`, `selected_dm_name`, `home_tab: Chats | Dashboard`).
6. **`CreateServerContext`**: Add/join server flow state (`modal_view: Home | Create | Join`, `is_modal_open: bool`, `from_dashboard: bool`).

---

## UI & Styling Guidelines

### Design System & Theme
- **Color Palette** (Discord / Telegram dark hybrid):
  - `$bg-darkest: #1e1f22` (Sidebar & window background)
  - `$bg-tertiary: #111214` (Deep background, textareas, inputs, search fields)
  - `$bg-secondary: #2b2d31` (Panels, channels sidebar, navigation)
  - `$bg-primary: #313338` (Chat area, main content canvas)
  - `$brand-primary: #5865f2` (Blurple primary accent)
  - `$brand-accent: #23a55a` (Green action accent, online status, server add hover)
  - `$danger: #da373c` (Destructive actions, logout, errors)
  - `$text-primary: #f2f3f5`, `$text-secondary: #949ba4`, `$text-muted: #6d6f78`
- **Typography**: Clean sans-serif (`gg sans`, `Noto Sans`, `Helvetica Neue`, sans-serif).
- **Glassmorphism & Overlays**: Backdrop filter `blur(12px)` on modals and sticky headers.
- **Scrollbars**: Thin, custom-styled scrollbars with rounded thumbs (`rgba(255, 255, 255, 0.15)`), hidden until hovered.

### Chat & Message System Nuances
- **Message Grouping (Chaining)**:
  - Consecutive messages from the same author sent within **5 minutes** on the **same calendar day** are grouped into a continuous bubble block.
  - The first message in a group (`is_first`) displays author avatar, username, and timestamp.
  - Chained messages (`.chained`) omit the avatar/author header, reduce vertical padding, and show an inline hover timestamp (`.chained-time`).
  - Corner rounding adapts dynamically: `.group-first` has rounded top corners, `.group-last` has rounded bottom corners, and middle messages have reduced corner radius.
- **Local Timezone Conversion**:
  - The backend stores and emits all timestamps in UTC (`time::OffsetDateTime`).
  - In WASM (`crates/ui/src/utils.rs`), `get_local_offset()` retrieves the client browser's local timezone offset via `js_sys::Date::new_0().get_timezone_offset()`.
  - All display times (`MessageItem` timestamps) and calendar day dividers are converted using `to_local_datetime(dt)` before formatting.
- **Date Dividers**:
  - Date dividers (`.chat-date-divider`) appear whenever a message's local date differs from the preceding message.
  - Formatted in Russian:
    - Current day: `"Сегодня, 21 сентября"`
    - Previous day: `"Вчера, 20 сентября"`
    - Current year: `"18 сентября, четверг"`
    - Older years: `"15 мая 2024 г."`
- **Avatar Fallback**:
  - For the current user's messages, avatars are retrieved directly from `UserProfileContext.avatar` (with fallback to `ServerContext.members`).
  - For other members, avatars are retrieved from `ServerContext.members`. If unavailable, a circular initial placeholder is rendered with an author-based color.
- **Keyed Memo Key**:
  - Sycamore `Keyed` message list must include `(message.id, is_first, is_last, date_divider.clone(), avatar_url.clone())` so reactive changes to avatar loading or group status re-render cleanly without recreating entire message elements.

### Mobile & Responsive Layout
- **Breakpoints**:
  - `$bp-sm: 480px` (Smartphones)
  - `$bp-md: 768px` (Tablets / small screens)
  - `$bp-lg: 1024px` (Laptops / desktops)
- **Single-Active-Panel Strategy on Mobile (`<= 768px`)**:
  - Mobile layout transforms the multi-column desktop layout into a single-panel flow.
  - Server sidebar remains accessible on the left.
  - Channels / Home nav panel occupies full remaining width when no channel or DM is active.
  - When a channel is clicked, `ChannelContext.current` is set, sliding the chat area into view.
  - A back button (`.chat-back-btn`) in `chat-header` allows clearing `ChannelContext.current` to return to the channel navigation list.
  - Modals (Profile Settings, Server Settings, Create Channel, Join Server) adapt on mobile:
    - Sidebar tabs convert to a horizontal scrollable tab strip (`.profile-sidebar-tabs`).
    - Full-screen height with scrollable content bodies and full-width stacked action buttons.
    - Top title bar includes an explicit `✕` close icon.

---

## Critical Gotchas & Pitfalls

1. **Sycamore 0.9 Modal Mounting Gotcha**:
   - **NEVER** use static class rules like `ClassRule::IfTrue("hidden", { !is_open.get() }.into())` for modals or overlays. Sycamore 0.9 evaluates `{ !is_open.get() }` once into a non-reactive `bool`, leaving overlays mounted with `position: fixed; inset: 0; z-index: 1000; pointer-events: auto;`. This intercepts all clicks and deadlocks the entire UI.
   - **Always** render modals conditionally using Sycamore view expressions:
     ```rust
     (if is_open.get() {
         view! { ModalContent(...) }
     } else {
         view! {}
     })
     ```
2. **Context Lifecycle & Early Mount**:
   - `HomePanel` is rendered in `App` (`app.rs`) from the moment the web app loads, even while the user is still on `ActiveScreen::Login`.
   - Any data fetching that requires authentication (such as `GetProfile` or `GetUserServers`) must **NOT** be executed as an unmanaged `spawn_local_scoped` at the root of `HomePanel`.
   - **Always** place authenticated startup tasks inside `create_effect(move || { if screen_wrapper.is_home() { ... } })` so they fire immediately upon login or token-restoration.
3. **Async Race Conditions on Shared Context**:
   - Do **NOT** reset shared context state (such as `context.members.set(vec![])`) inside the async callback of an unrelated command (e.g. `GetServerById`). If `GetServerById` completes after `GetMembers`, it wipes the member list out of existence.
   - Reset shared collections synchronously before kicking off concurrent fetch operations, or isolate state within dedicated effect triggers.
4. **Timezone Offset in WASM**:
   - Do not call `.date()` or format timestamps directly on raw UTC `OffsetDateTime` from the server.
   - For users in UTC+3 (or other timezones), midnight arrives at different UTC times; comparing raw UTC dates results in messages from the same night being misclassified as different days or grouped incorrectly.
   - Always run timestamps through `to_local_datetime(dt)` before date comparison or UI display.
5. **Textarea Resize & Form Theme**:
   - Multi-line textareas in modals must always have `resize: none;`, `background: $bg-tertiary;`, and `color: $text-primary;` to prevent breaking modal boundaries and preserve dark-theme consistency.
6. **Trunk Config Path**:
   - `Trunk.toml` is located at `crates/ui/Trunk.toml` (not `ui/Trunk.toml`).
   - Run commands using `--config crates/ui/Trunk.toml`.
7. **Linux WebKit Env Vars**:
   - Set in `crates/src-tauri/src/main.rs`: `WEBKIT_DISABLE_DMABUF_RENDERER`, `WEBKIT_DISABLE_COMPOSITING_MODE`, `GDK_BACKEND=wayland`. Required for Tauri to render properly on Linux.
8. **IDs and Types**:
   - IDs are UUIDs (v4 for `ServerId`/`ChannelId`, v7 for `UserId`/`MessageId`). Use `uuid::Uuid` comparisons.
   - `Cargo.lock` is gitignored (intentional workspace configuration).
   - `BASE_HTTP_URL` / `BASE_WS_URL` are hardcoded in `core/src/settings.rs`.
9. **Disposed Signal Access Panic in WASM**:
   - In Sycamore 0.9, setting a parent signal (such as `modal_context.is_open.set(false)`) that unmounts a component immediately runs the reactive graph and destroys the component's reactive scope and all local child signals.
   - **NEVER** call `.set()`, `.update()`, or `.replace()` on a component's local signal *after* triggering its parent unmount action. Accessing a disposed signal panics in Sycamore (`panic!("{}", self.get_disposed_panic_message())`), crashing the entire WASM runtime and making the UI completely unresponsive to clicks.
10. **Strict `invoke_command` Usage & Direct HTTP/WS Calls**:
   - **NEVER** call bare `tauri_sys::core::invoke` anywhere outside `crates/ui/src/utils.rs`. All communication with Tauri (`src-tauri`) must go through `invoke_command(CommandArgs::*.to_json())`.
   - Use `invoke_command` **ONLY** where behavior differs between Web and Desktop (token storage in keyring vs cookies, `GetDeviceInfo`, window controls, future system audio/noise suppression).
   - Call HTTP (`Api::*`) and WebSocket (`WsService::*`) methods directly inside UI components — do not route network requests through `invoke_command`.
11. **`collect_device_info()` in WASM vs Desktop**:
   - Because `crates/ui` is always compiled to `wasm32-unknown-unknown`, calling `gilvave_core::settings::collect_device_info()` directly inside UI components will **always** hit `#[cfg(target_arch = "wasm32")]` and report `client: "web"`, even when running inside the Tauri Desktop app.
   - Always retrieve `DeviceInfo` via `invoke_command(CommandArgs::GetDeviceInfo.to_json()).await` so Desktop fetches `collect_desktop()` (`client: "desktop"`) from `src-tauri` and Web fetches `collect_web()` (`client: "web"`).

---

## Testing & Quality Assurance

- **Unit tests**: Inline `#[cfg(test)]` modules (e.g. `crates/core/src/validation.rs`, `crates/src-tauri/src/database.rs`, `crates/ui/src/utils.rs`).
- **Integration & DTO tests**: In `crates/core/tests/` (`core_tests.rs`, `dto_tests.rs`).
- Verify workspace integrity before pushing:
  ```bash
  cargo check --package gilvave-ui --target wasm32-unknown-unknown
  cargo test --workspace
  trunk build --config crates/ui/Trunk.toml
  ```

---

## Conventions

- Edition 2024, rust-version 1.98, resolver 3.
- Workspace dependencies declared in root `Cargo.toml`; crates reference via `.workspace = true`.
- Internal crates use `path = "../<name>"` dependencies.
- Standard Rust formatting (`cargo fmt`).
- Git commit message format:
  ```
  [add] Description of additions...
  [fix] Description of fixes...
  [new] Description of new features...
  [refactor] Description of refactored components...
  ```
