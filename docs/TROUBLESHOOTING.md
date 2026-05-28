# Troubleshooting

This guide lists common setup, runtime, and integration issues for Vapor.

## Rust Toolchain Not Configured

Symptom:

```text
rustup could not choose a version of cargo to run
```

Cause:

No default Rust toolchain is installed or selected.

Fix:

```sh
rustup default stable
cargo check
```

## Chat Server Is Unavailable

Symptom:

```text
Chat unavailable
```

Cause:

No compatible TCP chat server is listening at the configured address.

Fix options:

- Start a compatible TCP chat server before launching Vapor.
- Set `VAPOR_CHAT_SERVER_ADDR` to the correct host and port.

Vapor should continue running without chat; the chat window displays the connection status.

## Login or Signup Does Not Work

Likely causes:

- backend API is unavailable
- network access is blocked
- API response shape changed
- username does not exist
- signup username is already taken

Checks:

```sh
curl "https://word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net/api/User/GetAllUsers"
```

Recommended fix:

Add structured error messages in `DbAPI` and expose them in the login/signup UI.

## Leaderboard Is Empty

Likely causes:

- leaderboard endpoints are unavailable
- backend has no score data
- request failed silently
- deserialization failed

Relevant endpoints:

```text
/User/GetScoresDescending
/User/GetScoresDescendingSudoku
/User/GetScoresDescendingMath
```

Recommended fix:

Add request status state for loading, success, empty, and error cases.

## Games Page Is Empty

Cause:

No files were found in the runtime `library` directory.

Fix:

1. Run `cargo build` or `cargo run` once.
2. Locate the executable directory, usually `target/debug` or `target/release`.
3. Place game executables in the `library` directory next to the Vapor executable.

Example:

```text
target/debug/library/word_unscrambler
```

## Game Launch Fails

Likely causes:

- selected file is not executable
- game binary was built for another platform
- file path contains unexpected permissions or signing restrictions
- game requires assets not present next to its binary

Checks:

```sh
ls -la target/debug/library
```

On Unix-like systems:

```sh
chmod +x target/debug/library/<game_binary>
```

## Game Runs But Scores Do Not Integrate

Cause:

Vapor currently parses child-process stdout with a prototype whitespace-delimited format.

Known recognized prefix:

```text
Word_Unscrambler
```

Recommended fix:

Define a stable JSON-lines protocol for all games and add parser tests.

## Friend Requests Do Not Appear Immediately

Possible causes:

- request failed
- backend did not create the relationship
- `update_indicator` was not set or consumed
- UI refreshed before backend state changed

Recommended fix:

Represent friend request state explicitly instead of relying on a boolean refresh flag.

## Account Settings Fail Silently

Cause:

Username and password changes print status to stdout/stderr but do not expose success or failure in the UI.

Recommended fix:

Store API operation results in `DbAPI` state and render user-facing status messages.

## Security Warnings

The current code passes credentials and account-change data in query strings for some operations. This is acceptable only for a prototype or controlled academic demonstration.

Recommended production changes:

- send credentials in HTTPS request bodies
- avoid logging passwords
- clear password fields after use
- do not store passwords in long-lived UI state
- add server-side password hashing and authentication tokens
