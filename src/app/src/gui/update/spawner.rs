use std::ffi;
use std::process;

use iced::Command;

use crate::model;

use crate::gui::message;

pub fn exec_cmd(
    game_library: &model::gl::GameLibraryState,
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
            let _wanted_to_be_detached = process::Command::new("snes9x")
                .current_dir(game_library.get_snes_prefix_path().clone().unwrap())
                .args(vec![
                    ffi::OsStr::new("-fullscreen"),
                    ffi::OsStr::new("-xvideo"),
                    snes_rom_path.into_os_string().as_os_str(),
                ])
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
        message::ExternalSpawn::DolphinGC(ngc_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("dolphin-emu-nogui")
                .current_dir(game_library.get_ngc_prefix_path().clone().unwrap())
                .arg(ngc_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
        message::ExternalSpawn::DolphinWii(wii_rom_path) => {
            let _wanted_to_be_detached = process::Command::new("dolphin-emu-nogui")
                .current_dir(game_library.get_wii_prefix_path().clone().unwrap())
                .arg(wii_rom_path.into_os_string())
                .spawn()
                .expect("Failed to execute command");
            Command::none()
        }
    }
}
