# Gilvave

Чат-приложение (типа Discord) на Tauri 2 + Sycamore (WASM).

## Стек

- **Backend**: Rust, Tauri 2, tokio-tungstenite (WebSocket), reqwest (HTTP)
- **Frontend**: Sycamore (WASM), компилируется через Trunk
- **Хранение токенов**: keyring (системный keychain)

## Возможности

- Авторизация (регистрация / вход)
- Серверы и каналы (текстовые + голосовые)
- Реалтайм-обмен сообщениями через WebSocket
- Список участников онлайн/офлайн

## Запуск

```bash
# Dev (Trunk + Tauri dev, горячая перезагрузка)
cargo tauri dev

# Dev только фронтенд (port 1420)
trunk serve --config ui/Trunk.toml

# Сборка
cargo tauri build
```

## Тесты

```bash
cargo test --workspace           # все тесты
cargo test --package gilvave-http # только HTTP
```

## Структура

```
crates/
  core/      — DTO, ошибки, ID (UUID v4/v7), keyring, настройки
  gateway/   — WebSocket сервис
  http/      — HTTP API клиент
  state/     — AppState (WebSocket sender + HTTP client)
  ui/        — Sycamore WASM фронтенд
  src-tauri/ — Tauri backend, диспетчер команд
```

## IDE

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
