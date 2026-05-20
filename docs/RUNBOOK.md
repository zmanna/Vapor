# Runbook

## Prerequisites

- Rust toolchain
- Cargo
- Network access if testing live API-backed features

Install Rust with `rustup` if needed:

```sh
rustup default stable
```

## Local Run

```sh
cargo run
```

## Build Check

```sh
cargo check
```

## Known Runtime Dependencies

Some features expect related services or repositories:

| Dependency | Purpose |
|---|---|
| database API | login, user, score, friends, and leaderboard requests |
| Word Unscrambler | connected game |
| Sudoku app | connected game |
| Rapid Math | connected game |

## Known Issues

- The API base URL is hardcoded in `src/data_base_api.rs`.
- Some account operations pass credentials in query parameters.
- The launcher title in `src/main.rs` still says `Word Unscrambler`.
- There is no automated test suite in this fork.
- `cargo check` could not be run in the cleanup environment because no default Rust toolchain was configured.

