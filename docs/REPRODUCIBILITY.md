# Reproducibility

This document explains how to reproduce the current Vapor launcher state from the repository and identifies dependencies that are external to the repo.

## Reproducibility Scope

The repository can reproduce:

- the Rust launcher source tree
- dependency resolution through `Cargo.lock`
- the desktop UI structure
- local game discovery behavior
- API request construction in the client
- TCP chat-client behavior

The repository does not independently reproduce:

- the backend API service
- backend database contents
- the chat server
- connected game executables
- leaderboard data
- account data

## Environment

Recommended environment:

| Tool | Version |
|---|---|
| Rust | stable toolchain, pinned by `rust-toolchain.toml` |
| Cargo | bundled with Rust |
| OS | macOS, Linux, or Windows with native GUI support |
| Network | required for API-backed features |

## Fresh Clone Procedure

```sh
git clone https://github.com/zmanna/Vapor.git
cd Vapor
rustup default stable
cargo check
cargo run
```

## Optional Service Setup

### Backend API

The launcher currently points to a hosted API:

```text
https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api
```

Related backend repository:

```text
https://github.com/willbtty/rust_api.git
```

Override the API base URL for local backend testing:

```sh
VAPOR_API_BASE_URL="http://localhost:3000/api" cargo run
```

TODO: Add instructions for running the backend locally.

### Chat Server

The chat panel expects a TCP server at:

```text
127.0.0.1:8080
```

If no server is available, Vapor still opens and displays an unavailable chat status.

Override the chat address:

```sh
VAPOR_CHAT_SERVER_ADDR="127.0.0.1:9000" cargo run
```

TODO: Add the chat server implementation or protocol.

### Connected Games

Referenced game repositories:

| Game | Repository |
|---|---|
| Word Unscrambler | `https://github.com/zmanna/Word_Unscrambler.git` |
| Sudoku | `https://github.com/yung00se/Sudoku_app.git` |
| Rapid Math | `https://github.com/zmanna/rapid-math.git` |

Build each game separately and place the executable in Vapor's runtime `library` directory.

## Game Library Location

At runtime, Vapor computes the library path from the compiled executable location:

```text
<directory containing Vapor executable>/library
```

Examples:

```text
target/debug/library
target/release/library
```

The directory is created automatically if missing.

Override the library path:

```sh
VAPOR_LIBRARY_PATH="./library" cargo run
```

## Verification Checklist

Use this checklist after setup:

- `cargo check` completes.
- `cargo run` opens a native window.
- The app does not panic when optional services are absent, or known service requirements are documented.
- Login/signup requests return expected backend data.
- The Games page displays files placed in the runtime `library` directory.
- Clicking a game starts its executable.
- Leaderboard tabs fetch and render data for Word Scramble, Sudoku, and Rapid Math.
- Friends page can fetch users and submit friend requests.
- Chat sends and receives newline-delimited messages when a compatible server is running.

## Current Verification Status

The following commands pass in the cleanup environment:

```sh
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

The test suite currently contains deterministic unit tests for configuration, API URL construction, optional chat behavior, and game-library discovery.

## Reproducibility Risks

| Risk | Impact | Recommended mitigation |
|---|---|---|
| Hosted API dependency | Results depend on external service availability. | Add local backend setup instructions. |
| Missing chat server | Chat cannot send or receive messages. | Document or include the chat server protocol. |
| Missing game binaries | Game page may be empty. | Add release artifacts or build instructions for each game. |
| Undocumented backend schema | API changes can break deserialization. | Add OpenAPI schema or sample fixtures. |
| Narrow automated tests | Live workflows are difficult to verify after changes. | Add integration and smoke tests. |
