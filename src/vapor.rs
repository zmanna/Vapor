use std::time::Duration;

use crate::chat_bar::{Chat, ChatBar};
use crate::config::RuntimeConfig;
use crate::pages::{
    friends_page::DisplayFriends,
    game_hub::{build_library, build_library_from_path, DisplayLibrary, GameIcon},
    leaderboard_page::DisplayLeaderboard,
    navigator::NavBar,
};
use crate::user_info::User;
use crate::{
    data_base_api::{DbAPI, MakeRequest},
    pages::leaderboard_page::Leaderboard,
};
use eframe::{
    egui::{
        self, Align, Button, CentralPanel, Color32, Key, Label, RichText, Sense, TextEdit,
        TextStyle, TopBottomPanel,
    },
    App, Frame,
};

/// Top-level application state for the Vapor desktop launcher.
///
/// `Vapor` is the integration point between the immediate-mode UI, backend API
/// client, local game library, leaderboard view state, and chat panel.
pub struct Vapor {
    pub current_user: User,
    pub db_api: DbAPI,
    pub current_page: String,
    pub game_library: Vec<GameIcon>,
    pub add_friend_input: String,
    pub leaderboard: Leaderboard,
    pub chat: Chat,
    pub new_username: String,
    pub new_password: String,
}

impl Default for Vapor {
    fn default() -> Self {
        let config = RuntimeConfig::from_env();
        let game_library = config
            .library_path
            .as_deref()
            .map(build_library_from_path)
            .unwrap_or_else(build_library);

        Self {
            current_user: User::new("".into(), "".into(), -1),
            db_api: DbAPI::new_with_base_url(config.api_base_url),
            current_page: "login".to_string(),
            game_library,
            add_friend_input: "".to_string(),
            leaderboard: Leaderboard::default(),
            chat: Chat::connect(&config.chat_server_addr),
            new_username: "".to_string(),
            new_password: "".to_string(),
        }
    }
}

impl App for Vapor {
    /// Renders one GUI frame and advances any page-level workflows.
    ///
    /// API requests complete asynchronously and write into shared buffers owned
    /// by `DbAPI`; this method reads those buffers on later redraws.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if self.current_user.id == -1 {
            self.display_landing(ctx);
            if let Some(user_info) = self.db_api.user.lock().unwrap().pop() {
                self.current_user.id = user_info.UserID
            }
        } else {
            if self.chat.username.is_empty() {
                self.chat.username = self.current_user.name.clone();
            }
            self.show_nav_bar(ctx);
            self.display_friends(ctx);
            self.chat.display_chat_bar(ctx);
            if *self.db_api.update_indicator.lock().unwrap() {
                self.db_api.get_friends_list(self.current_user.id);
                *self.db_api.update_indicator.lock().unwrap() = false;
            }
        }
        //Draw the current page
        self.show_current_page(ctx);
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}

impl Vapor {
    /// Creates the launcher state and applies the project-specific `egui` theme.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let ctx = &cc.egui_ctx;
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgb(92, 30, 38);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(56, 18, 23);
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(252, 251, 182));
        style.visuals.dark_mode = true;
        ctx.set_style(style);

        Self::default()
    }

    /// Displays the login/signup selector shown before a user is authenticated.
    fn display_landing(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("login-or-signup").show(ctx, |ui| {
            //Login or  Signup Selection
            ui.horizontal(|ui| {
                if ui
                    .add(
                        Label::new(
                            RichText::new("Log In")
                                .text_style(TextStyle::Heading)
                                .color(Color32::from_rgb(0, 200, 200)),
                        )
                        .sense(Sense::click()),
                    )
                    .clicked()
                {
                    self.current_page = "login".into()
                }

                ui.add_space(75.0);

                if ui
                    .add(
                        Label::new(RichText::new("Signup").text_style(TextStyle::Heading))
                            .sense(Sense::click()),
                    )
                    .clicked()
                {
                    self.current_page = "signup".into()
                } /*End Login/Signup Buttons*/
            });
        });
    }

    /// Displays the login form and starts a backend login lookup on submit.
    fn display_login(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            //Username/Password entry fields
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.heading("Log In");

                ui.add_space(50.0);

                ui.label("Username:");
                ui.add(
                    TextEdit::singleline(&mut self.current_user.name)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center),
                );

                ui.add_space(10.0);

                ui.label("Password:");
                ui.add(
                    TextEdit::singleline(&mut self.current_user.password)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center)
                        .password(true),
                );

                ui.add_space(20.0);
                //ui.label(RichText::new("Test").color(Color32::RED));
                let button = ui.add(Button::new("Log In"));
                if ui.input(|i| i.key_pressed(Key::Enter)) || button.clicked() {
                    self.request_login()
                }

                if self.current_user.id != -1 {
                    self.current_page = "lib".into();
                    ctx.request_repaint();
                }
            });
        });
    }

    /// Displays the signup form and starts a backend account creation request.
    fn display_signup(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            //Username/Password entry fields
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.heading("Sign Up");

                ui.add_space(50.0);

                ui.label("Username:");
                ui.add(
                    TextEdit::singleline(&mut self.current_user.name)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center),
                );

                ui.add_space(10.0);

                ui.label("Password:");
                ui.add(
                    TextEdit::singleline(&mut self.current_user.password)
                        .desired_width(200.0)
                        .horizontal_align(Align::Center)
                        .password(true),
                );

                ui.add_space(20.0);
                //ui.label(RichText::new("Test").color(Color32::RED));
                let button = ui.add(Button::new("Sign Up"));
                if ui.input(|i| i.key_pressed(Key::Enter)) || button.clicked() {
                    self.request_signup()
                }

                if self.current_user.id != -1 {
                    self.current_page = "lib".into();
                }
            });
        });
    }

    /// Displays account settings backed by username and password update calls.
    fn display_settings(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Account Settings");

            // Change Username
            ui.label("Change Username:");
            ui.add(
                TextEdit::singleline(&mut self.new_username)
                    .hint_text("Enter a new Username...")
                    .desired_width(200.0)
                    .horizontal_align(Align::Center),
            );
            if ui.button("Update Username").clicked() {
                self.current_user.name = self.new_username.clone();
                self.db_api
                    .change_username(self.current_user.id, &self.new_username);
            }

            // Change Password
            ui.label("Change Password:");
            ui.add(
                TextEdit::singleline(&mut self.new_password)
                    .password(true)
                    .hint_text("Enter new password..."),
            );
            if ui.button("Update Password").clicked() {
                self.db_api
                    .change_password(self.current_user.id, &self.new_password);
            }
        });
    }
    /// Requests login data for the username currently entered in the login form.
    fn request_login(&mut self) {
        self.db_api.get_login(self.current_user.name.as_str());
    }

    /// Requests account creation for the username/password currently entered.
    fn request_signup(&mut self) {
        self.db_api.post_signup(
            self.current_user.name.as_str(),
            self.current_user.password.as_str(),
        );
    }

    /// Routes the central panel to the currently selected launcher page.
    pub fn show_current_page(&mut self, ctx: &egui::Context) {
        if self.current_page == "lib" {
            self.display_library(ctx)
        } else if self.current_page == "friends" {
            self.add_friends(ctx);
            self.display_users(ctx)
        } else if self.current_page == "leaderboards" {
            self.display_leaderboard(ctx);
        } else if self.current_page == "login" {
            self.display_login(ctx)
        } else if self.current_page == "signup" {
            self.display_signup(ctx)
        } else if self.current_page == "settings" {
            self.display_settings(ctx)
        }
    }
} //End Vapor Implementation
