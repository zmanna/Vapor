# Methodology

Vapor is best understood as a computational systems integration project. It does not currently implement a scientific model, machine learning pipeline, or numerical simulation. Its methodology is the engineering approach used to coordinate independent game executables, remote user data, social features, and desktop UI state.

## Research and Engineering Question

The central question is:

> How can separately developed game projects be integrated into one launcher that provides shared user identity, friend discovery, chat, and leaderboards?

The project answers this through a Rust client that combines GUI rendering, local process orchestration, asynchronous API calls, and TCP messaging.

## System Assumptions

| Assumption | Current implementation |
|---|---|
| Users are represented by backend records | `UserEntry` mirrors the JSON returned by the API. |
| Scores are stored remotely | Leaderboards are fetched from backend endpoints. |
| Games can run as standalone executables | Vapor scans a local `library` directory and spawns files as child processes. |
| Game results can be emitted over stdout | `GameIcon::run_game` reads child-process stdout and parses game-specific lines. |
| Chat is a local service | `Chat` connects to `127.0.0.1:8080`. |
| UI state can be redrawn from shared state | `egui` renders each frame from `Vapor` and shared API buffers. |

## Workflow Method

1. Initialize desktop state and style.
2. Authenticate or create a user through backend API calls.
3. Use a page-based UI to separate launcher workflows.
4. Fetch remote social and leaderboard state on demand.
5. Discover runnable games from the local runtime directory.
6. Launch selected games as child processes.
7. Read game output for score/status integration.
8. Use a TCP client for chat messages.

## Data Sources

The repository itself does not include a dataset. Runtime data comes from:

- user-entered credentials and settings values
- backend API responses
- local files in the game `library` directory
- stdout emitted by launched games
- messages received from a local chat server

TODO: Add backend schema documentation and endpoint response examples.

## Algorithms and Protocols

### Directory Traversal

The launcher uses `walkdir` to recursively inspect the runtime `library` path. Each file becomes a launchable `GameIcon`.

### Page Routing

The active page is selected through string identifiers such as `login`, `signup`, `lib`, `friends`, `leaderboards`, and `settings`. This is simple but should be replaced with a typed enum.

### API Synchronization

API calls are asynchronous tasks. They update shared `Arc<Mutex<_>>` response buffers, which the UI reads during later redraws.

### Game Output Parsing

The child-process stdout parser currently expects whitespace-delimited text and recognizes game names such as `Word_Unscrambler`. This protocol is not yet formally specified.

Recommended future protocol:

```json
{"game":"word_unscrambler","event":"score","user_id":1,"score":120}
```

## Evaluation Approach

Because no formal evaluation artifacts are included, the current evaluation should focus on reproducible engineering checks:

- app builds successfully with stable Rust
- app launches when optional services such as chat are unavailable
- login/signup request paths match backend contract
- friends and leaderboards render expected API responses
- game discovery creates launch controls for executable files
- launched games produce parseable score/status output
- chat gracefully handles unavailable servers

TODO: Add automated tests and record expected outputs for representative backend responses.

## Scientific and Technical Limitations

- No included dataset, experimental protocol, or benchmark results.
- No statistical evaluation of user behavior or system performance.
- No formal schema for backend data.
- No formal specification for game result output.
- External services are not version-pinned in this repository.
- Reproducibility depends on services and executables outside the repository.

## Recommended Evidence for Publication or Case Study

To support publication-quality claims, add:

- API schema and sample fixtures
- executable packaging instructions for each connected game
- a documented game-result protocol
- automated build and integration tests
- screenshots or screen recordings of representative workflows
- benchmark results for startup time and API interactions, if relevant
- author and contribution statements approved by all collaborators
