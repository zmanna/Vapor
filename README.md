# Vapor

**Collaborative Rust game launcher app**

Vapor is a team-built game launcher for connecting multiple small games, user accounts, leaderboards, friends, and chat-oriented launcher features in one desktop experience.

## Project Context

This was a collaborative project. I was part of the team and contributed to the writing, architecture planning, launcher design, and integration direction across the connected games and backend services. This fork is included on my GitHub as portfolio evidence of that team contribution, not as a claim of sole ownership over the full codebase.

## What This Project Demonstrates

- Rust desktop application development with `eframe` and `egui`.
- Launcher architecture for organizing multiple games behind one user interface.
- Page-based UI structure for login, navigation, game hub, friends, chat, and leaderboards.
- Backend/API integration for users, scores, friends, and game data.
- Collaborative software design across multiple connected repositories.

## Project Structure

| Path | Purpose |
|---|---|
| `src/main.rs` | Application entry point. |
| `src/vapor.rs` | Main launcher application state and UI flow. |
| `src/pages/` | Page modules for navigation, game hub, friends, and leaderboards. |
| `src/data_base_api.rs` | API integration layer for user, score, and leaderboard data. |
| `src/chat_bar.rs` | Chat interface and local chat process integration. |
| `src/user_info.rs` | User/account state model. |
| `python_client/` | Supporting Python client experiment. |
| `images/` | Launcher image assets. |

## Documentation

- `docs/ARCHITECTURE.md`: launcher structure, state model, and integration model.
- `docs/RUNBOOK.md`: setup, run commands, dependencies, and known issues.
- `docs/CONTRIBUTIONS.md`: accurate team-contribution framing for portfolio use.

---

### Database API
https://github.com/willbtty/rust_api.git

---
### Word Game
https://github.com/zmanna/Word_Unscrambler.git

---
### Sudoku Game
https://github.com/yung00se/Sudoku_app.git

---
### Rapid Math
https://github.com/zmanna/rapid-math.git
