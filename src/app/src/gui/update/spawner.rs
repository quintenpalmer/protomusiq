use std::process;

use iced::Command;

use crate::gui::message;

pub fn exec_cmd(cmd: message::ExternalSpawn) -> Command<message::Message> {
    match cmd {
        message::ExternalSpawn::Mpv(movie_path) => {
            let _wanted_to_be_detached = process::Command::new("mpv")
                .arg(movie_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
    }
}
