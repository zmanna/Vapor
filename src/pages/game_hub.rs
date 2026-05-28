use eframe::{
    egui::{self, Color32, FontId, Pos2, Rect, Rounding, Sense, Shape, Stroke, Vec2},
    epaint::RectShape,
};
use std::fs;
use std::io::{BufRead, BufReader};
use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};
use walkdir::WalkDir;

//use crate::user_info::User;
use crate::{data_base_api::DbAPI, vapor::Vapor};

/// Metadata and launch behavior for one executable discovered in the game library.
#[derive(Clone)]
pub struct GameIcon {
    pub title: String,
    pub id: i16,
    pub rect: Shape,
    pub path: String,
}
impl Default for GameIcon {
    fn default() -> Self {
        Self {
            title: String::from(""),
            id: 0,
            rect: Shape::Noop,
            path: "".into(),
        }
    }
}

impl GameIcon {
    /// Starts the game process and reads a small amount of stdout for integration data.
    pub fn run_game(&self, _user_id: i32, _db_api: &DbAPI) {
        let mut game_instance = Command::new(&self.path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Game not found in Vapor Path...");

        thread::spawn(move || {
            {
                let Some(game_output) = &mut game_instance.stdout else {
                    let _ = game_instance.wait();
                    return;
                };

                let lines = BufReader::new(game_output).lines().enumerate().take(64);
                for (_, line) in lines {
                    eprint!("{:?}", &line);
                    let game_output = line.unwrap();
                    let mut split_output = game_output.split(' ');
                    let game_name = split_output.next().expect("Empty game data");

                    if game_name == "Word_Unscrambler" {
                        let score = split_output.next().expect("Missing Score Data");
                        let ratio = split_output
                            .next()
                            .expect("Missing Correct/Incorrect Ratio");
                        println!("{game_name}: Score: {score} Ratio: {ratio}");
                    } else if game_name == "Sudoku" {
                    }
                }
            }

            let _ = game_instance.wait();
        }); // End thread
    }

    /// Precomputes the simple rectangular icon shape used by the game hub.
    fn generate_icon_rect(&mut self) {
        let index = self.id as f32;
        let top_left = Pos2::from(((100.0 * index) + 5.0, 5.0));
        let bottom_right = Pos2::from(((100.0 * index) + 105.0, 55.0));

        self.rect = Shape::from(RectShape::new(
            Rect::from([top_left, bottom_right]),
            Rounding::ZERO,
            Color32::BLACK,
            Stroke::new(1.0, Color32::BLACK),
        ))
    }
}

pub trait DisplayLibrary {
    /// Renders the game library page.
    fn display_library(&mut self, ctx: &egui::Context);
}

impl DisplayLibrary for Vapor {
    fn display_library(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for game in self.game_library.iter() {
                let (game_rect, response) =
                    ui.allocate_exact_size(Vec2::new(200.0, 50.0), Sense::click());
                ui.painter().rect_filled(game_rect, 0.0, Color32::BLACK);
                ui.painter().text(
                    game_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    game.title.clone(),
                    FontId::default(),
                    Color32::WHITE,
                );
                if response.clicked() {
                    game.run_game(self.current_user.id, &self.db_api)
                }
            }
        });
    }
}

/// Scans the runtime `library` directory and returns launchable game metadata.
pub fn build_library() -> Vec<GameIcon> {
    let path = library_path_from_env_or_exe();
    eprint!("Path to Game Library: {:?}", &path);

    build_library_from_path(&path)
}

/// Scans a specific directory for launchable game metadata.
pub fn build_library_from_path(path: &Path) -> Vec<GameIcon> {
    let mut games: Vec<GameIcon> = Vec::new();
    if !fs::exists(path).expect("Error evaluating path to game library...") {
        fs::create_dir(path).expect("Could not create game library directory");
    }
    for result in WalkDir::new(path) {
        let entry = result.expect("No File...");
        if entry.file_type().is_file() {
            let filename = entry
                .file_name()
                .to_str()
                .expect("Error converting game file-name from osStr => &str");
            let path = entry.path().to_str().expect("Error unwrapping path");
            let mut icon = GameIcon {
                title: filename.into(),
                id: games.len() as i16,
                rect: Shape::Noop,
                path: path.into(),
            };
            icon.generate_icon_rect();
            eprint!("{:?}", icon.id);
            games.push(icon);
            eprint!("Icon for: {} created...", filename)
        }
    }

    games
}

fn library_path_from_env_or_exe() -> PathBuf {
    if let Ok(path) = env::var("VAPOR_LIBRARY_PATH") {
        return PathBuf::from(path);
    }

    let mut path = PathBuf::new();

    match env::current_exe() {
        Ok(vapor_path) => {
            path.push(vapor_path);
            path.pop();
            path.push("library");
        }
        Err(e) => eprint!("Error fetching path to Vapor exe: {e}"),
    };

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn build_library_from_path_creates_missing_directory() {
        let path = unique_temp_path("vapor-library-create");

        let games = build_library_from_path(&path);

        assert!(path.exists());
        assert!(games.is_empty());

        fs::remove_dir_all(path).expect("cleanup temp library");
    }

    #[test]
    fn build_library_from_path_discovers_files() {
        let path = unique_temp_path("vapor-library-discover");
        fs::create_dir_all(&path).expect("create temp library");
        File::create(path.join("example-game")).expect("create fake game");

        let games = build_library_from_path(&path);

        assert_eq!(games.len(), 1);
        assert_eq!(games[0].title, "example-game");

        fs::remove_dir_all(path).expect("cleanup temp library");
    }

    fn unique_temp_path(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before epoch")
            .as_nanos();
        env::temp_dir().join(format!("{prefix}-{nanos}"))
    }
}
