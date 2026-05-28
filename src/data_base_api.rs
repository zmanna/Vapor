use crate::config::DEFAULT_API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio;

/// Backend operations required by the launcher UI.
///
/// Implementations are currently fire-and-forget: each method starts a request
/// and writes the response into shared state owned by `DbAPI`.
pub trait MakeRequest {
    fn get_login(&self, username: &str);
    fn get_user_list(&self);
    fn get_friends_list(&self, user_id: i32);
    fn get_leaderboard(&self);
    fn get_user_stats(&self, user_id: &str);
    fn post_signup(&self, username: &str, password: &str);
    fn add_friend(&self, username: i32, friend: &str);
    fn change_password(&self, user_id: i32, new_password: &str);
    fn change_username(&self, user_id: i32, new_username: &str);
}

/*
pub enum ReturnType{
    IsValid(bool),
    Users(Vec<User>),
    Error(Option<String>),
    CurrentUser(User),
}
*/

/// User record returned by the backend API.
///
/// Field names intentionally mirror the current JSON contract. A future cleanup
/// should use idiomatic Rust names with `serde(rename = "...")` attributes.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserEntry {
    pub UserID: i32,
    pub Username: String,
    pub Password: String,
    pub HighScoreWord: i32,
    pub HighScoreSudoku: i32,
    pub HighScoreMath: i32,
}

impl Default for UserEntry {
    fn default() -> Self {
        Self {
            UserID: -1,
            Username: "".to_string(),
            Password: "".to_string(),
            HighScoreWord: 0,
            HighScoreSudoku: 0,
            HighScoreMath: 0,
        }
    }
}

impl UserEntry {
    pub fn new(id: i32, name: String, pass: String, score: i32) -> Self {
        Self {
            UserID: id,
            Username: name,
            Password: pass,
            HighScoreWord: score,
            HighScoreSudoku: 0,
            HighScoreMath: 0,
        }
    }
}

/// HTTP client and shared response buffers for backend-backed launcher data.
pub struct DbAPI {
    pub client: Client,
    pub api_base_url: String,
    pub user: Arc<Mutex<Vec<UserEntry>>>,
    pub friends_list: Arc<Mutex<Vec<String>>>,
    pub user_list: Arc<Mutex<Vec<UserEntry>>>,
    pub leaderboard: Arc<Mutex<Vec<UserEntry>>>,
    pub sudoku_leaderboard: Arc<Mutex<Vec<UserEntry>>>,
    pub math_leaderboard: Arc<Mutex<Vec<UserEntry>>>,

    pub update_indicator: Arc<Mutex<bool>>,
}

impl DbAPI {
    /// Creates an API client with empty response buffers.
    pub fn new() -> Self {
        Self::new_with_base_url(DEFAULT_API_BASE_URL)
    }

    /// Creates an API client using a caller-provided backend base URL.
    pub fn new_with_base_url(api_base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_base_url: normalize_base_url(api_base_url.into()),
            user: Arc::new(Mutex::new(Vec::new())),
            friends_list: Arc::new(Mutex::new(Vec::new())),
            user_list: Arc::new(Mutex::new(Vec::new())),
            leaderboard: Arc::new(Mutex::new(Vec::new())),
            sudoku_leaderboard: Arc::new(Mutex::new(Vec::new())),
            math_leaderboard: Arc::new(Mutex::new(Vec::new())),
            update_indicator: Arc::new(Mutex::new(false)),
        }
    }

    /// Builds a full API URL from an endpoint path.
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.api_base_url, ensure_leading_slash(path))
    }
}

impl Default for DbAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MakeRequest for DbAPI {
    /// Looks up a user by username and stores returned records in `self.user`.
    fn get_login(&self, username: &str) {
        let url = self.endpoint(&format!("/User/LookForUser?username={}", username));
        //eprint!("{}", url);
        let user_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> =
                        resp.json().await.expect("Error Logging in");
                    *user_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    /// Fetches all users for the friend-discovery UI.
    fn get_user_list(&self) {
        let url = self.endpoint("/User/GetAllUsers");
        eprint!("{}", url);
        // /Friend/GetAllFriends/{UserID}
        let user_list_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user_list);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> =
                        resp.json().await.expect("Error getting friends list");
                    *user_list_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    /// Fetches the current user's friend list.
    fn get_friends_list(&self, user_id: i32) {
        let url = self.endpoint(&format!("/Friend/GetAllFriends/{}", user_id));
        // /Friend/GetAllFriends/{UserID}
        let friends_list_arc: Arc<Mutex<Vec<String>>> = Arc::clone(&self.friends_list);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<String> =
                        resp.json().await.expect("Error getting friends list");
                    *friends_list_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    /// Fetches leaderboards for all currently supported games.
    fn get_leaderboard(&self) {
        let url = self.endpoint("/User/GetScoresDescending");
        let leaderboard_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.leaderboard);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> =
                        resp.json().await.expect("Error getting leaderboard");
                    //let response_body: String = resp.text().await.expect("Error getting leaderboard");
                    //eprint!("{}\n", response_body);
                    *leaderboard_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });

        let url = self.endpoint("/User/GetScoresDescendingSudoku");
        let leaderboard_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.sudoku_leaderboard);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> =
                        resp.json().await.expect("Error getting leaderboard");
                    //let response_body: String = resp.text().await.expect("Error getting leaderboard");
                    //eprint!("{}\n", response_body);
                    *leaderboard_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });

        let url = self.endpoint("/User/GetScoresDescendingMath");
        let leaderboard_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.math_leaderboard);
        tokio::spawn(async move {
            let response = reqwest::get(url).await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> =
                        resp.json().await.expect("Error getting leaderboard");
                    //let response_body: String = resp.text().await.expect("Error getting leaderboard");
                    //eprint!("{}\n", response_body);
                    *leaderboard_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    eprint!("{}", e);
                }
            }
        });
    }

    /// Placeholder for future per-user statistics retrieval.
    fn get_user_stats(&self, _user_id: &str) {}

    /// Creates a user account and stores the returned user record.
    fn post_signup(&self, username: &str, password: &str) {
        //username: &str, password: &str) {
        let url = self.endpoint(&format!(
            "/User/AddUser?username={}&password={}",
            username, password
        ));
        // User/AddUser?username=paul&password=firefire"
        let response_arc: Arc<Mutex<Vec<UserEntry>>> = Arc::clone(&self.user);
        let client_clone = self.client.clone();
        tokio::spawn(async move {
            let response = client_clone.post(url).body("").send().await;
            match response {
                Ok(resp) => {
                    let response_body: Vec<UserEntry> = resp.json().await.unwrap();
                    *response_arc.lock().unwrap() = response_body;
                }
                Err(e) => {
                    if e.status().unwrap() == 400 {
                        eprintln!("Username is taken");
                    } else {
                        eprint!("{}", e);
                    }
                }
            }
        });
    }

    /// Sends a friend request and marks the friends list for refresh on success.
    fn add_friend(&self, user_id: i32, friend: &str) {
        let url = self.endpoint(&format!(
            "/Friend/SendFriendRequest?userId={}&friendUsername={}",
            user_id, friend
        ));

        let update_notifier = self.update_indicator.clone();
        let client_clone = self.client.clone();
        tokio::spawn(async move {
            let response = client_clone.post(url).body("").send().await;
            match response {
                Ok(_) => *update_notifier.lock().unwrap() = true,
                Err(e) => eprint!("Error sending friend request: {e}"),
            }
        });
    }

    /// Requests a password change for the current account.
    fn change_password(&self, user_id: i32, new_password: &str) {
        let url = self.endpoint(&format!(
            "User/ChangePassword?UserID={}&NewPassword={}",
            user_id, new_password
        ));
        let client_clone = self.client.clone();
        tokio::spawn(async move {
            let response = client_clone.put(url).body("").send().await;
            match response {
                Ok(_) => {
                    eprint!("password changed");
                }
                Err(e) => eprint!("Error changing password: {e}"),
            }
        });
    }

    /// Requests a username change for the current account.
    fn change_username(&self, user_id: i32, new_username: &str) {
        let url = self.endpoint(&format!(
            "/User/ChangeUsername?UserID={}&NewUsername={}",
            user_id, new_username
        ));
        let client_clone = self.client.clone();
        let new_username = new_username.to_string();
        tokio::spawn(async move {
            let response = client_clone.put(url).body("").send().await;
            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("Username successfully changed to '{}'", new_username);
                    } else {
                        eprintln!("Failed to change username: {}", resp.status());
                    }
                }
                Err(e) => eprintln!("Error changing username: {}", e),
            }
        });
    }
}

fn normalize_base_url(base_url: String) -> String {
    base_url.trim_end_matches('/').to_string()
}

fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_api_base_url() {
        let api = DbAPI::new_with_base_url("https://example.test/api/");

        assert_eq!(api.api_base_url, "https://example.test/api");
    }

    #[test]
    fn endpoint_accepts_paths_with_or_without_leading_slash() {
        let api = DbAPI::new_with_base_url("https://example.test/api/");

        assert_eq!(
            api.endpoint("/User/GetAllUsers"),
            "https://example.test/api/User/GetAllUsers"
        );
        assert_eq!(
            api.endpoint("User/GetAllUsers"),
            "https://example.test/api/User/GetAllUsers"
        );
    }
}
