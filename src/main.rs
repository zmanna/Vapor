use eframe::NativeOptions;
use vapor::vapor::Vapor;

/// Starts the native Vapor desktop launcher.
///
/// The Tokio runtime is initialized here because API calls are spawned from the
/// GUI layer. `eframe` owns the native event loop after `run_native` starts.
#[tokio::main]
async fn main() {
    let native_options = NativeOptions::default();

    let _ = eframe::run_native(
        // Start Vapor
        "Vapor", // Set the app title
        native_options,
        Box::new(|cc| Ok(Box::new(Vapor::new(cc)))),
    );
}
