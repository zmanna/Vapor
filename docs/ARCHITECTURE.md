# Architecture

This document describes Vapor's current architecture, module boundaries, data flow, and extension points. It is written for future contributors and evaluators who need to understand how the launcher is assembled.

## System Purpose

Vapor is a Rust desktop launcher that coordinates multiple game executables through a shared interface for accounts, friends, leaderboards, chat, and local launch workflows. The repository contains the launcher client only; the backend API, chat server, and connected games are external dependencies.

## Runtime Components

| Component | Location | Responsibility |
|---|---|---|
| Native app entry point | `src/main.rs` | Starts the `eframe` native window and constructs `Vapor`. |
| Application state | `src/vapor.rs` | Owns login state, selected page, game library, API client, leaderboard model, chat model, and settings form state. |
| Navigation | `src/pages/navigator.rs` | Renders the top navigation controls and triggers page transitions or data fetches. |
| Game hub | `src/pages/game_hub.rs` | Discovers local game executables, renders launch controls, and starts selected games. |
| Friends page | `src/pages/friends_page.rs` | Renders friend/user lists and sends add-friend requests. |
| Leaderboard page | `src/pages/leaderboard_page.rs` | Renders leaderboard tabs and top-score lists for supported games. |
| Backend API client | `src/data_base_api.rs` | Sends HTTP requests for login, signup, users, friends, account settings, and leaderboard data. |
| Chat panel | `src/chat_bar.rs` | Opens TCP streams to a local chat server and renders messages. |
| User model | `src/user_info.rs` | Stores current user identity and associated UI-facing collections. |
| Python client experiment | `python_client/main.py` | Experimental stdin/stdout client scaffold; not part of the main Rust runtime. |

## Control Flow

1. `main` starts `eframe::run_native`.
2. `Vapor::new` applies a custom `egui` style and returns default app state.
3. `Vapor::update` runs every GUI frame.
4. Unauthenticated users see the login/signup flow.
5. Authenticated users see the navigation bar, friends window, chat panel, and selected main page.
6. Page modules render UI and call `DbAPI` or `GameIcon` methods in response to user actions.
7. API requests update shared state containers that are read during later redraws.

## State Ownership

`Vapor` is the top-level state container. It owns:

- `current_user`: active account state
- `db_api`: backend integration client and shared API response buffers
- `current_page`: active page identifier
- `game_library`: discovered game executable metadata
- `add_friend_input`: friend form state
- `leaderboard`: selected leaderboard tab state
- `chat`: chat connection and message buffers
- `new_username` and `new_password`: account-settings form state

The current implementation uses strings for page names. A typed enum would make invalid pages impossible and improve maintainability.

## Backend API Model

`DbAPI` stores response data in `Arc<Mutex<_>>` containers:

| Field | Data |
|---|---|
| `user` | login/signup response records |
| `friends_list` | current user's friend names |
| `user_list` | global user list for friend search |
| `leaderboard` | Word Scramble score records |
| `sudoku_leaderboard` | Sudoku score records |
| `math_leaderboard` | Rapid Math score records |
| `update_indicator` | flag used to refresh friends after add-friend operations |

Request methods spawn asynchronous tasks with `tokio::spawn`. The UI continues rendering while requests complete, then reads updated data on subsequent frames.

### API Endpoints Used

The API base URL defaults to:

```text
https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api
```

It can be overridden with `VAPOR_API_BASE_URL`.

Observed endpoint patterns:

| Operation | Endpoint pattern |
|---|---|
| Login lookup | `/User/LookForUser?username={username}` |
| Signup | `/User/AddUser?username={username}&password={password}` |
| All users | `/User/GetAllUsers` |
| Friends list | `/Friend/GetAllFriends/{UserID}` |
| Add friend | `/Friend/SendFriendRequest?userId={id}&friendUsername={username}` |
| Word leaderboard | `/User/GetScoresDescending` |
| Sudoku leaderboard | `/User/GetScoresDescendingSudoku` |
| Rapid Math leaderboard | `/User/GetScoresDescendingMath` |
| Change username | `/User/ChangeUsername?UserID={id}&NewUsername={username}` |
| Change password | `/User/ChangePassword?UserID={id}&NewPassword={password}` |

Design note: credentials and account changes should be moved out of query strings before production use.

## Game Library Model

`build_library` determines the directory next to the running Vapor executable, appends `library`, creates it if missing, and scans files using `walkdir`. `VAPOR_LIBRARY_PATH` can override that directory. Each file becomes a `GameIcon` with:

- display title
- numeric id
- precomputed icon rectangle shape
- executable path

When a game is clicked, `GameIcon::run_game` starts it as a child process and reads up to 64 stdout lines. The parser currently recognizes `Word_Unscrambler` output and has a placeholder branch for `Sudoku`.

Recommended future protocol:

```json
{"game":"word_unscrambler","user_id":12,"score":420,"metadata":{"ratio":"8/10"}}
```

JSON lines would be easier to validate and extend than whitespace-delimited text.

## Chat Model

`Chat::connect` attempts to open two TCP connections to the configured chat server, one for writing and one for reading. The default is `127.0.0.1:8080`, and `VAPOR_CHAT_SERVER_ADDR` can override it. A reader thread collects newline-delimited messages into `message_list`. `display_chat_bar` renders a floating chat window and sends messages as:

```text
{username}: {message}
```

Connection failures are rendered as chat status instead of panicking at startup.

## Design Decisions

| Decision | Rationale | Tradeoff |
|---|---|---|
| Immediate-mode GUI with `egui` | Fast iteration for a desktop prototype. | Requires careful state ownership and redraw behavior. |
| Page traits implemented for `Vapor` | Keeps page rendering in separate files while sharing app state. | Traits are UI-focused and tightly coupled to `Vapor`. |
| Shared `Arc<Mutex<_>>` API buffers | Simple async-to-UI communication pattern. | Limited error visibility and potential lock coupling. |
| Local executable discovery | Keeps games independently buildable and launchable. | Requires documented binary placement and output protocol. |
| Environment-variable configuration | Keeps the prototype simple while allowing local service overrides. | A typed config file would be more discoverable for packaged releases. |

## Naming Conventions

Current conventions are mixed because the code reflects a collaborative prototype:

- Rust modules use snake_case.
- Some API response fields mirror backend JSON using PascalCase, such as `UserID` and `HighScoreWord`.
- Page names are lowercase strings such as `login`, `signup`, and `leaderboards`.
- Connected game labels use user-facing names such as `Word Scramble`, `Sudoku`, and `Rapid Math`.

Recommended cleanup:

- preserve backend field names with `#[serde(rename = "...")]`
- expose idiomatic Rust fields such as `user_id` internally
- replace page strings with a `Page` enum
- centralize game identifiers in a `GameKind` enum

## Folder Responsibilities

| Folder | Responsibility |
|---|---|
| `src/` | Core launcher source code. |
| `src/pages/` | UI page modules and page-specific interactions. |
| `docs/` | Architecture, reproduction, methodology, troubleshooting, and contribution documentation. |
| `images/` | Static visual assets. |
| `python_client/` | Experimental or supporting client code. |

## Extension Points

### Add a New Page

1. Create a new module under `src/pages/`.
2. Define a display trait for the page if following existing conventions.
3. Implement the trait for `Vapor`.
4. Export the module in `src/pages/mod.rs`.
5. Add a navigation control in `src/pages/navigator.rs`.
6. Add routing in `Vapor::show_current_page`.

### Add a New Backend Endpoint

1. Add a method to the `MakeRequest` trait.
2. Implement it for `DbAPI`.
3. Add a typed response struct if the response shape is new.
4. Store response state in `DbAPI` or return a typed future in a future refactor.
5. Document the endpoint in this file and in reproducibility notes.

### Add a New Game

1. Build the game as an executable for the target platform.
2. Place the executable in the runtime `library` directory.
3. Ensure the game accepts or can infer the current user context if needed.
4. Emit score data using the documented protocol.
5. Add leaderboard API support if the game has persistent scoring.

### Improve Chat

1. Add reconnect controls.
2. Add richer connection-state UI.
3. Replace raw strings with typed message structures if persistence or moderation is needed.

## Known Technical Debt

- Credentials passed in query strings.
- No request timeout or retry policy.
- Sparse structured error handling.
- Test coverage is still narrow and does not exercise live API behavior.
- Some unused imports and prototype-era debug output.
- Duplicate leaderboard rendering logic.
- Page routing is stringly typed.

## Recommended Refactor Sequence

1. Replace page strings with enums.
2. Add typed API request/response models and error states.
3. Define and test a game score-reporting protocol.
4. Add request timeout and retry behavior.
5. Split `data_base_api.rs` into smaller modules once endpoint coverage grows.
