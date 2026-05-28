# UML and Diagrams

This document contains Mermaid diagrams that GitHub can render directly. The diagrams describe the current launcher architecture, runtime workflow, data movement, and main object relationships.

## High-Level System Architecture

```mermaid
flowchart LR
    User["Desktop user"] --> VaporApp["Vapor desktop app<br/>Rust + eframe/egui"]

    VaporApp --> UI["Page modules<br/>games, friends, leaderboards, settings"]
    VaporApp --> DbAPI["DbAPI<br/>HTTP client"]
    VaporApp --> Chat["Chat panel<br/>TCP client"]
    VaporApp --> GameHub["Game hub<br/>local executable launcher"]

    DbAPI --> Backend["Remote backend API<br/>users, friends, scores"]
    Chat --> ChatServer["Local chat server<br/>127.0.0.1:8080"]
    GameHub --> Library["Runtime library directory"]
    Library --> Games["External game executables"]
    Games --> GameStdout["stdout score/status lines"]
    GameStdout --> GameHub
```

## Main Workflow Sequence

```mermaid
sequenceDiagram
    actor User
    participant Main as main.rs
    participant Vapor as Vapor
    participant DbAPI as DbAPI
    participant Backend as Backend API
    participant GameHub as Game Hub
    participant Game as Game executable
    participant Chat as Chat
    participant ChatServer as Chat server

    User->>Main: cargo run
    Main->>Vapor: Vapor::new()
    Vapor->>Chat: Chat::connect(config.chat_server_addr)
    Chat->>ChatServer: open TCP connections
    Vapor-->>User: render login/signup
    User->>Vapor: submit username/password
    Vapor->>DbAPI: get_login() or post_signup()
    DbAPI->>Backend: HTTP request
    Backend-->>DbAPI: JSON user response
    DbAPI-->>Vapor: update shared user buffer
    Vapor-->>User: render authenticated launcher
    User->>Vapor: open Games page
    Vapor->>GameHub: display_library()
    User->>GameHub: click game
    GameHub->>Game: spawn process
    Game-->>GameHub: stdout score/status data
```

## Data Flow Diagram

```mermaid
flowchart TD
    Credentials["Username/password input"] --> LoginSignup["Login/signup UI"]
    LoginSignup --> APIRequests["DbAPI request methods"]
    APIRequests --> RemoteAPI["Backend API"]
    RemoteAPI --> UserEntries["UserEntry JSON records"]
    UserEntries --> SharedState["Arc<Mutex<_>> response buffers"]
    SharedState --> Pages["egui page renderers"]

    LibraryDir["Runtime library directory"] --> WalkDir["walkdir scan"]
    WalkDir --> GameIcons["Vec<GameIcon>"]
    GameIcons --> GamePage["Game hub page"]
    GamePage --> ChildProcess["Child process"]
    ChildProcess --> StdoutParser["stdout parser"]

    ChatInput["Chat input"] --> TcpWrite["TCP write stream"]
    TcpWrite --> ChatService["Chat server"]
    ChatService --> TcpRead["TCP read stream"]
    TcpRead --> MessageBuffer["message_list buffer"]
    MessageBuffer --> ChatWindow["Chat window"]
```

## Module Dependency Diagram

```mermaid
flowchart LR
    main["src/main.rs"] --> vapor["src/vapor.rs"]
    lib["src/lib.rs"] --> vapor
    lib --> pages["src/pages/mod.rs"]
    lib --> user_info["src/user_info.rs"]
    lib --> data_base_api["src/data_base_api.rs"]
    lib --> chat_bar["src/chat_bar.rs"]

    vapor --> user_info
    vapor --> data_base_api
    vapor --> chat_bar
    vapor --> navigator["src/pages/navigator.rs"]
    vapor --> game_hub["src/pages/game_hub.rs"]
    vapor --> friends_page["src/pages/friends_page.rs"]
    vapor --> leaderboard_page["src/pages/leaderboard_page.rs"]

    navigator --> data_base_api
    game_hub --> data_base_api
    friends_page --> data_base_api
    leaderboard_page --> data_base_api
```

## Class/Object Model

```mermaid
classDiagram
    class Vapor {
        +User current_user
        +DbAPI db_api
        +String current_page
        +Vec~GameIcon~ game_library
        +String add_friend_input
        +Leaderboard leaderboard
        +Chat chat
        +String new_username
        +String new_password
        +new(cc) Vapor
        +show_current_page(ctx)
    }

    class User {
        +String name
        +String password
        +i32 id
        +Vec~GameIcon~ library
        +Vec~UserEntry~ friends
        +Vec~UserEntry~ leaderboard
        +String current_page
        +new(name, password, id) User
    }

    class DbAPI {
        +Client client
        +Arc~Mutex~ user
        +Arc~Mutex~ friends_list
        +Arc~Mutex~ user_list
        +Arc~Mutex~ leaderboard
        +Arc~Mutex~ sudoku_leaderboard
        +Arc~Mutex~ math_leaderboard
        +Arc~Mutex~ update_indicator
        +new() DbAPI
    }

    class MakeRequest {
        <<trait>>
        +get_login(username)
        +get_user_list()
        +get_friends_list(user_id)
        +get_leaderboard()
        +post_signup(username, password)
        +add_friend(user_id, friend)
        +change_password(user_id, new_password)
        +change_username(user_id, new_username)
    }

    class UserEntry {
        +i32 UserID
        +String Username
        +String Password
        +i32 HighScoreWord
        +i32 HighScoreSudoku
        +i32 HighScoreMath
    }

    class GameIcon {
        +String title
        +i16 id
        +Shape rect
        +String path
        +run_game(user_id, db_api)
    }

    class Leaderboard {
        -String current_page
        +DbAPI db_api
        +display_leaderboard(ctx)
    }

    class Chat {
        +String username
        -String chat_input
        -Arc~Mutex~ read_buffer
        -Arc~Mutex~ message_list
        -TcpStream write_stream
        +new() Chat
    }

    class ChatBar {
        <<trait>>
        +display_chat_bar(ctx)
    }

    Vapor --> User
    Vapor --> DbAPI
    Vapor --> Chat
    Vapor --> Leaderboard
    Vapor --> GameIcon
    DbAPI ..|> MakeRequest
    Chat ..|> ChatBar
    DbAPI --> UserEntry
    User --> UserEntry
```

## Page Routing State

```mermaid
stateDiagram-v2
    [*] --> Login
    Login --> Library: successful login
    Signup --> Library: successful signup
    Login --> Signup: user selects signup
    Signup --> Login: user selects login

    Library --> Friends: nav click
    Library --> Leaderboards: nav click
    Library --> Settings: nav click
    Friends --> Library: nav click
    Friends --> Leaderboards: nav click
    Friends --> Settings: nav click
    Leaderboards --> Library: nav click
    Leaderboards --> Friends: nav click
    Leaderboards --> Settings: nav click
    Settings --> Library: nav click
    Settings --> Friends: nav click
    Settings --> Leaderboards: nav click
```
