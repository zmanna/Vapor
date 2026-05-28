# Vapor

**A Rust desktop launcher for coordinating small games, shared user accounts, friends, chat, and cross-game leaderboards.**

Vapor is a collaborative `eframe`/`egui` desktop application that acts as a central launcher for several game projects. It provides a unified interface for authentication, game discovery, friend management, leaderboard viewing, and chat-oriented social features.

> Project status: academic/team project prototype. The repository is suitable for study, portfolio review, and extension work, but several production-readiness items are intentionally documented as limitations.

## Overview

Vapor explores how a desktop launcher can integrate multiple independently developed games behind one user experience. The app is written in Rust and uses `egui` for immediate-mode desktop UI. Runtime state is held in the main `Vapor` application struct, while API-backed features use asynchronous HTTP requests to a related backend service.

The launcher currently focuses on:

- account login and signup through a remote API
- local game executable discovery from a `library` directory
- game launch from the desktop UI
- friends list and add-friend workflows
- leaderboards for Word Scramble, Sudoku, and Rapid Math
- a TCP-based chat panel intended to connect to a local chat service

## Problem Statement

Small game projects are often built as isolated executables with separate scorekeeping, user flows, and distribution assumptions. Vapor addresses the integration problem: how can multiple games share a common user identity, social layer, and leaderboard surface while remaining separate runnable programs?

## Motivation

The project demonstrates practical systems integration rather than a single algorithmic model. It is useful for evaluating:

- Rust GUI application structure
- multi-module UI organization
- API boundary design between a client launcher and backend service
- local process orchestration for external game executables
- team-oriented architecture and documentation practices

## Key Features

| Area | Current behavior |
|---|---|
| Desktop UI | Native Rust GUI using `eframe` and `egui`. |
| Authentication | Login and signup requests against a backend user API. |
| Game library | Scans a runtime `library` directory next to the Vapor executable and displays runnable files. |
| Game launch | Starts selected games as child processes. |
| Leaderboards | Retrieves ranked score lists for Word Scramble, Sudoku, and Rapid Math. |
| Friends | Lists friends, displays users, and sends friend requests through API endpoints. |
| Chat | Connects to a local TCP chat server at `127.0.0.1:8080`. |
| Portfolio context | Includes explicit contribution framing for a collaborative project. |

## Technical Background

Vapor is built around an immediate-mode GUI model. Each frame redraws the current page from the latest application state. User actions trigger page transitions, process launches, or API calls. API calls are spawned asynchronously and write results into shared `Arc<Mutex<_>>` state containers that the UI reads on later redraws.

This architecture is appropriate for a student/team prototype because it keeps integration points easy to inspect. For production use, the same boundaries should be tightened with typed error handling, explicit configuration, request cancellation, and automated integration tests.

## Repository Structure

```text
.
|-- Cargo.toml                  # Rust package metadata and dependencies
|-- Cargo.lock                  # Locked dependency graph
|-- LICENSE                     # MIT license
|-- README.md                   # Project overview and evaluator-facing documentation
|-- docs/
|   |-- ARCHITECTURE.md         # System architecture and extension guide
|   |-- CONTRIBUTIONS.md        # Accurate collaborative contribution framing
|   |-- METHODOLOGY.md          # Engineering methodology and assumptions
|   |-- REPRODUCIBILITY.md      # Reproduction checklist and verification notes
|   |-- RUNBOOK.md              # Operational commands and runtime notes
|   |-- TROUBLESHOOTING.md      # Common failures and fixes
|   `-- UML.md                  # Mermaid diagrams for architecture and workflows
|-- images/
|   `-- W-icon.png              # Launcher image asset
|-- python_client/
|   `-- main.py                 # Experimental stdin/stdout Python client
`-- src/
    |-- main.rs                 # Native application entry point
    |-- lib.rs                  # Library module exports
    |-- vapor.rs                # Main application state and page routing
    |-- data_base_api.rs        # HTTP API client and shared response state
    |-- chat_bar.rs             # TCP chat client UI panel
    |-- user_info.rs            # User/account state model
    `-- pages/
        |-- friends_page.rs     # Friend and user listing UI
        |-- game_hub.rs         # Local executable discovery and launch UI
        |-- leaderboard_page.rs # Leaderboard tabs and rendering
        |-- mod.rs              # Page module exports
        `-- navigator.rs        # Top navigation bar
```

## Architecture Overview

Vapor has five primary layers:

| Layer | Responsibility |
|---|---|
| Application shell | Starts `eframe`, initializes the `Vapor` state, and drives redraws. |
| Page modules | Render specific UI workflows such as games, friends, settings, and leaderboards. |
| API integration | Sends requests to the backend service and stores response data for UI consumption. |
| Local process integration | Discovers executable games and starts them as child processes. |
| Chat integration | Connects to a local TCP service and renders incoming messages. |

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/UML.md](docs/UML.md) for diagrams and contributor notes.

## Data Pipeline and Workflow

1. The user starts the Vapor desktop app with `cargo run`.
2. `src/main.rs` launches a native `eframe` window and creates `Vapor`.
3. The login or signup page sends user credentials to the backend API.
4. The returned user identifier unlocks the authenticated launcher pages.
5. The navigation bar switches between the game library, friends, leaderboards, and account settings.
6. The game library scans the runtime `library` directory and renders one launch control per executable file.
7. Leaderboard and friends pages request data from API endpoints and render the latest shared state.
8. The chat panel attempts to connect to a local TCP chat server at `127.0.0.1:8080`.

## Installation

### Prerequisites

- Rust toolchain with Cargo, preferably stable Rust
- Network access for API-backed features
- Optional: a compatible chat server listening on `127.0.0.1:8080`
- Optional: compiled game executables placed in Vapor's runtime `library` directory

Install Rust with `rustup`:

```sh
rustup default stable
```

Clone and enter the repository:

```sh
git clone https://github.com/zmanna/Vapor.git
cd Vapor
```

Build dependencies:

```sh
cargo check
```

Run the app:

```sh
cargo run
```

## Environment Setup

The launcher has defaults for external services and supports environment-variable overrides:

| Service | Location | Used by |
|---|---|---|
| Backend API | `https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api` | login, signup, users, friends, leaderboards, settings |
| Chat server | `127.0.0.1:8080` | chat panel |
| Game library | `library` directory next to the compiled Vapor executable | game discovery and launch |

## Configuration

Configuration is loaded from environment variables in `src/config.rs`:

| Variable | Default | Purpose |
|---|---|---|
| `VAPOR_API_BASE_URL` | hosted Azure API URL | Backend user/friend/leaderboard requests. |
| `VAPOR_CHAT_SERVER_ADDR` | `127.0.0.1:8080` | TCP chat server address. |
| `VAPOR_LIBRARY_PATH` | `library` next to the Vapor executable | Directory scanned for game executables. |

Example:

```sh
VAPOR_API_BASE_URL="http://localhost:3000/api" \
VAPOR_CHAT_SERVER_ADDR="127.0.0.1:8080" \
VAPOR_LIBRARY_PATH="./library" \
cargo run
```

Page identifiers and game protocol names are still represented as source-level strings.

## Usage Examples

Run Vapor locally:

```sh
cargo run
```

Create a release build:

```sh
cargo build --release
```

Place game executables in the runtime library directory:

```text
target/debug/library/
target/release/library/
```

The app creates the `library` directory automatically if it does not exist.

## Input Requirements

| Input | Format | Notes |
|---|---|---|
| Username | Text | Used for login, signup, friend search, and account settings. |
| Password | Text | Used for signup and password update. Current API calls place credentials in query strings; see security limitations. |
| Game executable | Local executable file | Must be placed in the runtime `library` directory. |
| Game output | Whitespace-delimited stdout lines | `GameIcon::run_game` currently parses game name and score-like fields from stdout. |
| API response | JSON | Expected to deserialize into `UserEntry` or string lists depending on endpoint. |
| Chat messages | Newline-delimited TCP stream | Rendered in the chat window. |

## Output Files and Results

Vapor does not currently write structured output files. Observable outputs are:

- the native desktop launcher UI
- child game processes launched from the game library
- API-side mutations such as signup, friend requests, username changes, and password changes
- stdout/stderr diagnostic output from the launcher and launched games
- leaderboard and friends data rendered in the UI

TODO: Add an exported results format for game sessions if reproducible analysis or publication artifacts are required.

## Methodology

Vapor uses a modular GUI architecture:

- `Vapor` owns the global state and current page.
- Page modules expose traits implemented for `Vapor`, keeping page rendering separate from the main app loop.
- `DbAPI` wraps remote API interactions and stores response data in thread-safe containers.
- `GameIcon` represents a discovered game executable and its launch metadata.
- `Chat` owns TCP streams and message buffers for the chat panel.

The project's engineering methodology is documented in [docs/METHODOLOGY.md](docs/METHODOLOGY.md).

## Model and Algorithm Details

Vapor does not implement a machine learning model or numerical scientific algorithm. The core computational behavior is systems-oriented:

- directory traversal for game discovery via `walkdir`
- event-driven GUI rendering through `egui`
- asynchronous HTTP requests through `tokio` and `reqwest`
- shared-state synchronization with `Arc<Mutex<_>>`
- process spawning through `std::process::Command`
- TCP stream reading/writing for chat

## Evaluation Metrics

No formal benchmark or user study is included in the repository. Appropriate future evaluation metrics include:

- successful clean build rate across supported platforms
- login/signup/friend/leaderboard API integration success rate
- time from app launch to interactive UI
- number of supported games using a documented game-output protocol
- UI task completion rate for launching a game and viewing scores
- test coverage for request construction, parsing, and page-state transitions

TODO: Add benchmark results or usability evaluation data if this project is used in a paper, case study, or senior-project report.

## Results Summary

Supported by the current repository:

- Rust desktop app skeleton with modular page organization
- backend API client for users, friends, settings, and leaderboards
- local executable discovery and launch flow
- leaderboards for three game categories
- optional TCP chat panel integration point
- environment-variable configuration for API, chat, and library paths
- CI workflow for formatting, checking, linting, and tests
- unit tests for configuration, API endpoint construction, chat failure handling, and game-library discovery
- project documentation framing for collaborative contribution

Not yet supported or not verifiable from this repository alone:

- fully self-contained backend reproduction
- documented chat server implementation
- published benchmark results
- formal dataset or academic citation

## Reproducibility

Minimum reproduction path:

1. Install stable Rust.
2. Clone the repository.
3. Run `cargo check`.
4. Run `cargo run`.
5. Optionally start a compatible local chat server on `127.0.0.1:8080`; the app still opens without it.
6. Optionally place compatible game executables into the runtime `library` directory.
7. Verify login, friends, and leaderboard features against the configured backend API.

Detailed instructions are in [docs/REPRODUCIBILITY.md](docs/REPRODUCIBILITY.md).

## Testing

The repository includes a small automated test suite for local, deterministic behavior. Recommended commands:

```sh
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Current tests cover:

- default configuration values
- API base URL normalization and endpoint construction
- optional chat startup when no server is available
- creation and scanning of the game-library directory

## Troubleshooting

Common issues:

| Symptom | Likely cause | Fix |
|---|---|---|
| `rustup could not choose a version of cargo` | No default Rust toolchain configured | Run `rustup default stable`. |
| Chat shows unavailable status | No TCP chat server is listening on `127.0.0.1:8080` | Start the chat server or set `VAPOR_CHAT_SERVER_ADDR`. |
| Login or leaderboard data does not appear | Backend API unavailable or response shape changed | Check network access and API endpoint compatibility. |
| No games appear | Runtime `library` directory is empty | Add executable files to the library directory next to the Vapor binary. |
| Game launch fails | File is not executable or path is invalid | Build the game binary and ensure execute permission. |

See [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for more detail.

## Limitations

- The app still depends on an external backend for account, friends, and leaderboard behavior.
- Credentials are passed through query strings for some API operations.
- Passwords are stored in the in-memory `User` struct as plain strings.
- Error handling often uses `expect`/`unwrap`, which can terminate the app on recoverable failures.
- Page routing uses string literals instead of a typed enum.
- The repository does not include the backend, chat server, or connected game source code as submodules.

## Future Work

- Replace string page names with a `Page` enum.
- Move credentials out of query strings and use secure request bodies over HTTPS.
- Add structured request/response types per API endpoint.
- Expand unit and integration tests around page routing, request failures, and game-output parsing.
- Define a formal game-launch and score-reporting protocol.
- Package connected games and backend deployment instructions.
- Document individual team contributions with commit-level evidence where available.

## Citation and Academic Use

No formal citation is provided by the repository. If used in academic writing, cite the GitHub repository and include an access date.

Suggested informal citation:

```text
Zutshi, A. and collaborators. Vapor: A Rust desktop launcher for integrated game accounts, friends, chat, and leaderboards. GitHub repository, 2024. https://github.com/zmanna/Vapor
```

TODO: Confirm author list, project date, course or institution context, and preferred citation format before publication.

## Contributing

Contributions should preserve the modular structure:

- put page-specific UI in `src/pages/`
- put backend request logic in `src/data_base_api.rs` or a future `api/` module
- put launcher-level state transitions in `src/vapor.rs`
- document new runtime services in `docs/ARCHITECTURE.md` and `docs/REPRODUCIBILITY.md`

Before opening a pull request, run:

```sh
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

See [docs/CONTRIBUTIONS.md](docs/CONTRIBUTIONS.md) for current contribution-context guidance.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).

## Contact

Primary repository: [https://github.com/zmanna/Vapor](https://github.com/zmanna/Vapor)

Related repositories referenced by the project:

| Repository | Role |
|---|---|
| [willbtty/rust_api](https://github.com/willbtty/rust_api) | Backend database/API service |
| [zmanna/Word_Unscrambler](https://github.com/zmanna/Word_Unscrambler) | Connected word game |
| [yung00se/Sudoku_app](https://github.com/yung00se/Sudoku_app) | Connected Sudoku game |
| [zmanna/rapid-math](https://github.com/zmanna/rapid-math) | Connected math game |
