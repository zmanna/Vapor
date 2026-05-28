use eframe::egui::{self, Button};
use emath::Align2;
use std::net::TcpStream;
use std::{
    io::{BufRead, BufReader, Write},
    sync::{Arc, Mutex},
};

/// TCP-backed chat state rendered by the launcher.
///
/// The current implementation opens local TCP streams during construction and
/// stores received newline-delimited messages for display in the chat window.
pub struct Chat {
    pub username: String,
    chat_input: String,
    message_list: Arc<Mutex<Vec<String>>>,
    write_stream: Option<TcpStream>,
    connection_status: String,
}

impl Chat {
    /// Connects to the local chat server and starts a reader thread.
    pub fn new() -> Self {
        Self::connect(crate::config::DEFAULT_CHAT_SERVER_ADDR)
    }

    /// Attempts to connect to the configured chat server.
    ///
    /// Connection failures are stored as UI state instead of panicking, which
    /// allows the launcher to run when chat is unavailable.
    pub fn connect(server_addr: &str) -> Self {
        let write_stream = TcpStream::connect(server_addr);

        let message_list = Arc::new(Mutex::new(Vec::new()));
        let mut connection_status = format!("Chat unavailable: could not connect to {server_addr}");

        let write_stream = match write_stream {
            Ok(write_stream) => {
                match TcpStream::connect(server_addr) {
                    Ok(read_stream) => {
                        let read_stream = Arc::new(Mutex::new(read_stream));
                        let read_stream_clone = Arc::clone(&read_stream);
                        let message_list_clone = Arc::clone(&message_list);
                        connection_status = format!("Connected to chat server at {server_addr}");

                        std::thread::spawn(move || {
                            let mut stream = read_stream_clone.lock().unwrap();
                            let reader = BufReader::new(&mut *stream);

                            for line_result in reader.lines() {
                                match line_result {
                                    Ok(line) => message_list_clone.lock().unwrap().push(line),
                                    Err(e) => {
                                        message_list_clone
                                            .lock()
                                            .unwrap()
                                            .push(format!("Chat read error: {e}"));
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        connection_status =
                            format!("Chat unavailable: could not open read stream: {e}");
                    }
                }

                Some(write_stream)
            }
            Err(e) => {
                connection_status = format!("Chat unavailable: {e}");
                None
            }
        };

        Self {
            username: "".into(),
            chat_input: "".into(),
            message_list,
            write_stream,
            connection_status,
        }
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ChatBar {
    //fn send_message_getter(&self) -> String;
    //fn message_list_getter(&self) -> Vec<String>;
    /// Renders the floating chat window and sends typed messages.
    fn display_chat_bar(&mut self, ctx: &egui::Context);
}

impl ChatBar for Chat {
    fn display_chat_bar(&mut self, ctx: &egui::Context) {
        egui::Window::new("Chat")
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(92, 30, 38),
                rounding: egui::Rounding::same(5.0),
                ..Default::default()
            })
            .min_height(200.0)
            .max_height(300.0)
            .collapsible(true)
            .scroll([false, true]) //Horizontal Scrolling: False, Vertical Scrolling: True
            .anchor(Align2::LEFT_BOTTOM, [10.0, 0.0])
            .pivot(Align2::LEFT_BOTTOM)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    egui::Frame::none() //Blank frame for styling the text edit box
                        .fill(egui::Color32::from_rgb(56, 18, 23))
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.chat_input)
                                    .return_key(None)
                                    .frame(false) //Override default text edit style
                                    .text_color(egui::Color32::from_rgb(252, 251, 182))
                                    .desired_width(ui.available_width())
                                    .hint_text("Type here..."),
                            );

                            let send_button = ui.add(Button::new("Send"));
                            if send_button.clicked() {
                                if let Some(write_stream) = &mut self.write_stream {
                                    match write_stream.write_all(
                                        format!("{}: {}\n", self.username, self.chat_input)
                                            .as_bytes(),
                                    ) {
                                        Ok(()) => self.chat_input = String::new(),
                                        Err(e) => {
                                            self.connection_status =
                                                format!("Chat send failed: {e}");
                                            self.write_stream = None;
                                        }
                                    }
                                }
                            }

                            ui.label(&self.connection_status);
                            let messages_result = self.message_list.lock().unwrap();
                            for message in messages_result.iter() {
                                ui.label(message);
                            }
                        }); //End text input
                }); //End bottom_up display area
            }); //End Window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_chat_server_does_not_panic() {
        let chat = Chat::connect("127.0.0.1:9");

        assert!(chat.write_stream.is_none());
        assert!(chat.connection_status.contains("Chat unavailable"));
    }
}
