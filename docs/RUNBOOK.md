# Runbook

This runbook contains the short operational commands for building and running Vapor. For a fuller reproduction guide, see `docs/REPRODUCIBILITY.md`.

## Prerequisites

- Rust toolchain with Cargo
- Network access for API-backed features
- Optional chat server on `127.0.0.1:8080`
- Optional connected game executables in the runtime `library` directory

Install or select stable Rust:

```sh
rustup default stable
```

## Local Development Commands

Check compilation:

```sh
cargo check
```

Run the desktop app:

```sh
cargo run
```

Format source:

```sh
cargo fmt
```

Run lints:

```sh
cargo clippy --all-targets --all-features
```

Run tests:

```sh
cargo test
```

## Runtime Dependencies

| Dependency | Purpose | Required for startup |
|---|---|---|
| Rust/Cargo | Build and run the launcher | Yes |
| Backend API | Login, signup, users, friends, account settings, leaderboards | No, but related features fail without it |
| Chat server | Chat panel messages | No; unavailable chat is shown in the UI |
| Game executables | Launchable games | No |

## Environment Variables

| Variable | Default |
|---|---|
| `VAPOR_API_BASE_URL` | hosted Azure API URL |
| `VAPOR_CHAT_SERVER_ADDR` | `127.0.0.1:8080` |
| `VAPOR_LIBRARY_PATH` | `library` next to the Vapor executable |

## Connected Repositories

| Repository | Role |
|---|---|
| `https://github.com/willbtty/rust_api.git` | Backend database/API service |
| `https://github.com/zmanna/Word_Unscrambler.git` | Word game |
| `https://github.com/yung00se/Sudoku_app.git` | Sudoku game |
| `https://github.com/zmanna/rapid-math.git` | Rapid Math game |

## Runtime Library Directory

Vapor scans for games in:

```text
<directory containing Vapor executable>/library
```

Typical development path:

```text
target/debug/library
```

Typical release path:

```text
target/release/library
```

## Known Issues

- Some account operations pass credentials through query strings.
- Backend, chat server, and connected game implementations are external to this repository.
- Test coverage is intentionally small and does not cover live backend workflows.
