use std::ffi;
use std::process;

use iced::Command;

use crate::gui::message;

pub fn exec_cmd(
    game_library: &musiqcore::model::gl::GameLibraryState,
    cmd: message::ExternalSpawn,
) -> Command<message::Message> {
    match cmd {
        message::ExternalSpawn::Mpv(movie_path) => {
            let _wanted_to_be_detached = process::Command::new("mpv")
                .arg(movie_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
        message::ExternalSpawn::LaunchEmulator(console, game_rom_path) => {
            let cwd = game_library
                .get_console_prefix(&console)
                .expect("no prefix dir for console");

            let (spawn_command, raw_spawn_args) = console.get_spawn_command();

            let mut spawn_args: Vec<ffi::OsString> = raw_spawn_args
                .into_iter()
                .map(|x| ffi::OsStr::new(x).to_os_string())
                .collect();
            spawn_args.push(game_rom_path.into_os_string());

            let _wanted_to_be_detached = process::Command::new(spawn_command)
                .current_dir(cwd)
                .args(spawn_args)
                .spawn()
                .expect("Failed to execute command");

            Command::none()
        }
    }
}
