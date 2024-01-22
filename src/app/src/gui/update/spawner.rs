use std::process;

use iced::Command;

use crate::gui::message;

pub fn exec_cmd(cmd: message::ExternalSpawn) -> Command<message::Message> {
    match cmd {
        message::ExternalSpawn::Mpv(movie_path) => {
            process::Command::new("mpv")
                .arg(movie_path.into_os_string())
                .output()
                .expect("Failed to execute command");
            Command::none()
        }
    }
}
