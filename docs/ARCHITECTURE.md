# Architecture

## Purpose

Vapor is a collaborative Rust desktop launcher that connects multiple games through a shared interface for login, game navigation, friends, chat, and leaderboards.

## Runtime Shape

```text
main.rs
  -> eframe native app
  -> Vapor application state
  -> page modules
  -> database API client
  -> related game/back-end services
```

## Core Modules

| Module | Responsibility |
|---|---|
| `src/main.rs` | Starts the native `eframe` application. |
| `src/vapor.rs` | Owns application state, page routing, login/signup flow, account settings, and high-level UI composition. |
| `src/pages/navigator.rs` | Navigation controls for switching launcher pages. |
| `src/pages/game_hub.rs` | Game library and launcher display. |
| `src/pages/friends_page.rs` | Friends list and add-friend interface. |
| `src/pages/leaderboard_page.rs` | Leaderboard display across connected games. |
| `src/data_base_api.rs` | HTTP integration layer for login, friends, users, and leaderboards. |
| `src/chat_bar.rs` | Chat UI and local chat process integration. |
| `src/user_info.rs` | User state model. |
| `python_client/` | Supporting chat/client experiment. |

## State Model

The `Vapor` struct is the application state container. It stores:

- current user
- current page
- game library
- database API client
- friends state
- leaderboard state
- chat state
- settings form state

The UI redraws continuously with `ctx.request_repaint_after`, which keeps async-updated state visible in the native interface.

## Integration Model

`DbAPI` owns shared state containers wrapped in `Arc<Mutex<_>>`. Request methods spawn asynchronous work with `tokio::spawn`, update shared vectors, and allow UI pages to read the latest data.

This design kept the launcher responsive during team development, but it has clear improvement areas:

- move hardcoded API base URLs into configuration
- avoid sending passwords through query strings
- centralize response/error handling
- add typed request/response models per endpoint
- add integration tests around request construction

## Collaboration Context

This fork is portfolio evidence of team contribution. The upstream repository remains the original project source. Contribution claims should stay specific to writing, architecture planning, launcher design, and integration direction unless individual implementation ownership is later documented more precisely.

