# Plan: Reorganize crates/ui/src/components/

## Current Problems

1. **Context leakage** — `MemberContext` and `ChannelContext` are defined in `pages/home_panel.rs` but consumed by 4 feature components (`server_sidebar`, `channel_panel`, `members_panel`, `channel_item`). Features depend on pages — wrong direction.
2. **Mixed concerns in `common/`** — Navigation state (`ActiveScreen`, `ScreenWrapper`) mixed with CSS utilities (`ClassRule`, `classes()`).
3. **Auth components scattered** — `features/form_tabs.rs` and `features/social_buttons.rs` are auth-only but sit in the generic `features/` root.
4. **`header.rs` misplaced** — App-specific (Tauri window controls, drag handling) but lives in `ui/` alongside generic primitives.
5. **`templates/` underused** — Only `auth_form.rs`, SCSS index empty.

## Target Structure

```
components/
├── common/
│   ├── mod.rs              — re-exports all sub-modules
│   ├── navigation.rs       — ActiveScreen, ScreenWrapper (extracted from mod.rs)
│   ├── contexts.rs         — MemberContext, ChannelContext (moved from home_panel.rs)
│   ├── helpers.rs          — ClassRule, ClassRuleVariant, classes() (extracted from mod.rs)
│   ├── _index.scss         — unchanged
│   ├── _variables.scss     — unchanged
│   ├── _reset.scss         — unchanged
│   └── _animations.scss    — unchanged
│
├── ui/
│   ├── mod.rs              — remove `header` module
│   ├── spinner.rs          — unchanged
│   ├── divider.rs          — unchanged
│   ├── input_group.rs      — unchanged
│   ├── checkbox.rs         — unchanged
│   ├── submit_button.rs    — unchanged
│   ├── social_button.rs    — unchanged
│   ├── tab_button.rs       — unchanged
│   ├── icons/              — unchanged
│   ├── _index.scss         — unchanged
│   ├── _spinner.scss       — unchanged
│   ├── _button.scss        — unchanged
│   ├── _input.scss         — unchanged
│   ├── _checkbox.scss      — unchanged
│   ├── _divider.scss       — unchanged
│   └── _header.scss        — keep styles here (SCSS stays, .rs moves)
│
├── layout/
│   ├── mod.rs              — NEW, declares `header` module
│   ├── header.rs           — MOVED from ui/header.rs
│   ├── _index.scss         — unchanged
│   ├── _body.scss          — unchanged
│   └── _container.scss     — unchanged
│
├── features/
│   ├── mod.rs              — re-exports all sub-modules
│   ├── auth/
│   │   ├── mod.rs          — re-exports form_tabs, social_buttons
│   │   ├── form_tabs.rs    — MOVED from features/form_tabs.rs
│   │   └── social_buttons.rs — MOVED from features/social_buttons.rs
│   ├── chat/
│   │   ├── mod.rs          — re-exports messages_area, message_item
│   │   ├── messages_area.rs — MOVED from features/messages_area.rs
│   │   └── message_item.rs — MOVED from features/message_item.rs
│   ├── channels/
│   │   ├── mod.rs          — re-exports channel_panel, channel_item
│   │   ├── channel_panel.rs — MOVED from features/channel_panel.rs
│   │   └── channel_item.rs — MOVED from features/channel_item.rs
│   ├── members/
│   │   ├── mod.rs          — re-exports members_panel, member_item
│   │   ├── members_panel.rs — MOVED from features/members_panel.rs
│   │   └── member_item.rs  — MOVED from features/member_item.rs
│   └── servers/
│       ├── mod.rs          — re-exports server_sidebar
│       └── server_sidebar.rs — MOVED from features/server_sidebar.rs
│   ├── _index.scss         — unchanged
│   └── all _*.scss         — unchanged
│
├── pages/
│   ├── mod.rs              — unchanged
│   ├── login_panel.rs      — update imports only
│   ├── register_panel.rs   — update imports only
│   ├── home_panel.rs       — remove context definitions, update imports
│   └── _*.scss             — unchanged
│
├── templates/
│   ├── mod.rs              — unchanged
│   ├── auth_form.rs        — update imports only
│   └── _index.scss         — unchanged
│
└── main.scss               — unchanged
```

## Steps

### Step 1: Extract `common/` sub-modules

Split `common/mod.rs` (99 lines) into 3 focused files:

**`common/navigation.rs`** — cut `ActiveScreen` enum + `ScreenWrapper` struct (lines 1-38)
**`common/helpers.rs`** — cut `ClassRule` enum + `From` impls + `classes()` fn (lines 40-99)
**`common/mod.rs`** — becomes re-exports: `mod navigation; mod helpers; pub use navigation::*; pub use helpers::*;`

### Step 2: Create `common/contexts.rs`

Extract `MemberContext` and `ChannelContext` from `pages/home_panel.rs` (currently defined there, lines ~varies). Place them in `common/contexts.rs` with their own `mod.rs`-friendly exports.

### Step 3: Move `ui/header.rs` → `layout/header.rs`

- Move the file
- Create `layout/mod.rs` with `pub mod header;`
- Update all imports that reference `crate::components::ui::header::AppHeader` → `crate::components::layout::header::AppHeader`
- Remove `pub mod header;` from `ui/mod.rs`

### Step 4: Group auth features

- Create `features/auth/mod.rs` with `pub mod form_tabs; pub mod social_buttons;`
- Move `features/form_tabs.rs` → `features/auth/form_tabs.rs`
- Move `features/social_buttons.rs` → `features/auth/social_buttons.rs`

### Step 5: Group remaining features

Create sub-module directories:
- `features/chat/mod.rs` — move `messages_area.rs`, `message_item.rs`
- `features/channels/mod.rs` — move `channel_panel.rs`, `channel_item.rs`
- `features/members/mod.rs` — move `members_panel.rs`, `member_item.rs`
- `features/servers/mod.rs` — move `server_sidebar.rs`

Update `features/mod.rs` to declare the 5 sub-modules.

### Step 6: Update all imports

Files that need import updates:
- `app.rs` — `ui::header` → `layout::header`
- `pages/home_panel.rs` — remove context defs, import from `common::contexts`
- `features/channels/channel_item.rs` — `pages::home_panel::ChannelContext` → `common::contexts::ChannelContext`
- `features/channels/channel_panel.rs` — same pattern
- `features/members/members_panel.rs` — `pages::home_panel::MemberContext` → `common::contexts::MemberContext`
- `features/servers/server_sidebar.rs` — both contexts + `ActiveScreen`/`ScreenWrapper` from `common`
- `features/auth/form_tabs.rs` — `ui::tab_button` → `crate::components::ui::tab_button`
- `features/auth/social_buttons.rs` — `ui::icons` / `ui::social_button` → same (no change needed)
- `templates/auth_form.rs` — `features::form_tabs` → `features::auth::form_tabs`
- `pages/login_panel.rs` — `features::social_buttons` → `features::auth::social_buttons`
- `pages/register_panel.rs` — same

### Step 7: Verify

```bash
cargo check --package gilvave-ui
cargo clippy --package gilvave-ui
```

## Files Modified (summary)

| Action | File |
|--------|------|
| EDIT | `common/mod.rs` — rewrite as re-exports |
| CREATE | `common/navigation.rs` — ActiveScreen, ScreenWrapper |
| CREATE | `common/helpers.rs` — ClassRule, classes() |
| CREATE | `common/contexts.rs` — MemberContext, ChannelContext |
| MOVE | `ui/header.rs` → `layout/header.rs` |
| CREATE | `layout/mod.rs` |
| EDIT | `ui/mod.rs` — remove header |
| CREATE | `features/auth/mod.rs` |
| MOVE | `features/form_tabs.rs` → `features/auth/form_tabs.rs` |
| MOVE | `features/social_buttons.rs` → `features/auth/social_buttons.rs` |
| CREATE | `features/chat/mod.rs` |
| MOVE | `features/messages_area.rs` → `features/chat/messages_area.rs` |
| MOVE | `features/message_item.rs` → `features/chat/message_item.rs` |
| CREATE | `features/channels/mod.rs` |
| MOVE | `features/channel_panel.rs` → `features/channels/channel_panel.rs` |
| MOVE | `features/channel_item.rs` → `features/channels/channel_item.rs` |
| CREATE | `features/members/mod.rs` |
| MOVE | `features/members_panel.rs` → `features/members/members_panel.rs` |
| MOVE | `features/member_item.rs` → `features/members/member_item.rs` |
| CREATE | `features/servers/mod.rs` |
| MOVE | `features/server_sidebar.rs` → `features/servers/server_sidebar.rs` |
| EDIT | `features/mod.rs` — update sub-module declarations |
| EDIT | `app.rs` — update imports |
| EDIT | `pages/home_panel.rs` — remove context defs, update imports |
| EDIT | `pages/login_panel.rs` — update imports |
| EDIT | `pages/register_panel.rs` — update imports |
| EDIT | `templates/auth_form.rs` — update imports |
