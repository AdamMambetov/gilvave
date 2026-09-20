# AGENTS.md — Gilvave

Tauri 2 + Sycamore (WASM) chat application. Rust workspace with 6 crates.

## Architecture

```
crates/
  core/      — DTOs, error types, ID generation (UUID v4/v7), security (keyring), settings
  gateway/   — WebSocket service (tokio-tungstenite), message handling, auto-reconnect
  http/      — HTTP API client (reqwest via tauri-plugin-http)
  state/     — AppState: shared WebSocket sender + HTTP client
  ui/        — Sycamore WASM frontend (Trunk build, port 1420)
  src-tauri/ — Tauri backend entry point, command dispatch handler, local SQLite db
```

**Data flow**: UI (WASM) → `invoke("handle_command")` → `handler::handle_command` → `dispatch!` macro → `http`/`gateway` services → response back to UI.

**Command system**: All frontend commands pass through `CommandArgs` enum (`core/src/dto/command.rs`) → `handler::handle_command` → `dispatch!` macro. The macro auto-retries on 401 by calling `Api::update_tokens`.

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
cargo test --package gilvave-http        # HTTP crate tests (DTO serialize/deserialize)
cargo test --package gilvave-core        # Core crate tests
cargo test --workspace                   # All workspace tests

# Lint
cargo clippy --workspace
```

---

## Crate dependency order

`core` → `state` → `gateway` / `http` → `src-tauri` (bottom-up; `core` has no internal deps).
`ui` depends only on `core` (shared DTOs) and communicates with `src-tauri` exclusively via Tauri IPC invokes & event listeners.

---

## UI Architecture (`crates/ui`)

Sycamore 0.9 + Trunk single-page WASM application styled with modular SCSS.

### Directory Structure

```
crates/ui/src/
  app.rs                    — Root App component, session check, view switching
  main.rs                   — WASM entry point (mounts to DOM, console error hook)
  utils.rs                  — Tauri IPC invoke helper, browser local timezone conversion
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

---

## Testing & Quality Assurance

- **Unit tests**: Inline `#[cfg(test)]` modules.
- **Integration tests**: In `crates/<name>/tests/` (e.g., `crates/http/tests/api_tests.rs`).
- Dev-dependencies `uuid` and `time` in `http/Cargo.toml` enable mock DTO test construction without server connectivity.
- Verify workspace integrity before pushing:
  ```bash
  cargo check --package gilvave-ui --target wasm32-unknown-unknown
  cargo test --workspace
  trunk build --config crates/ui/Trunk.toml
  ```

---

## Conventions

- Edition 2024, rust-version 1.96, resolver 3.
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
