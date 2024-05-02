use std::process;

use iced::Command;

use crate::model;

use crate::gui::message;

pub fn exec_cmd(
    game_library: &model::GameLibraryState,
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
        message::ExternalSpawn::MGBA(gba_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("mgba")
                .current_dir(game_library.get_gba_prefix_path().clone().unwrap())
                .arg(gba_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
        message::ExternalSpawn::ZSNES(snes_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("zsnes")
                .current_dir(game_library.get_snes_prefix_path().clone().unwrap())
                .arg(snes_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
        message::ExternalSpawn::Mupen64(n64_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("mupen64plus")
                .current_dir(game_library.get_n64_prefix_path().clone().unwrap())
                .arg(n64_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
        message::ExternalSpawn::Desmume(nds_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("desmume")
                .current_dir(game_library.get_nds_prefix_path().clone().unwrap())
                .arg(nds_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
    }
}
