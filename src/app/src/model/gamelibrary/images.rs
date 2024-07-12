use std::collections::BTreeMap;
use std::fs;
use std::path;

use super::consoles;
use super::nameutil;

pub struct ConsoleGameImageMap {
    preferred_region: String,
    consoles: BTreeMap<consoles::GameConsole, BTreeMap<String, path::PathBuf>>,
}

impl ConsoleGameImageMap {
    pub fn new(image_parent_path: &path::PathBuf, preferred_region: String) -> Self {
        let mut btreemap = BTreeMap::new();

        let gba_games =
            load_all_games_for_console(&image_parent_path, consoles::GameConsole::GameBoyAdvance);
        btreemap.insert(consoles::GameConsole::GameBoyAdvance, gba_games);

        let ds_games =
            load_all_games_for_console(&image_parent_path, consoles::GameConsole::NintendoDS);
        btreemap.insert(consoles::GameConsole::NintendoDS, ds_games);

        let snes_games =
            load_all_games_for_console(&image_parent_path, consoles::GameConsole::SNES);
        btreemap.insert(consoles::GameConsole::SNES, snes_games);

        let n64_games =
            load_all_games_for_console(&image_parent_path, consoles::GameConsole::Nintendo64);
        btreemap.insert(consoles::GameConsole::Nintendo64, n64_games);

        let gamecube_games =
            load_all_games_for_console(&image_parent_path, consoles::GameConsole::GameCube);
        btreemap.insert(consoles::GameConsole::GameCube, gamecube_games);

        let wii_games = load_all_games_for_console(&image_parent_path, consoles::GameConsole::Wii);
        btreemap.insert(consoles::GameConsole::Wii, wii_games);

        ConsoleGameImageMap {
            preferred_region: preferred_region,
            consoles: btreemap,
        }
    }

    pub fn get_console_map(
        &self,
        console: &consoles::GameConsole,
    ) -> Option<&BTreeMap<String, path::PathBuf>> {
        self.consoles.get(console)
    }

    pub fn get_preferred_region(&self) -> String {
        self.preferred_region.clone()
    }
}

fn load_all_games_for_console(
    parent_path: &path::PathBuf,
    console: consoles::GameConsole,
) -> BTreeMap<String, path::PathBuf> {
    let console_sub_path = console.full_name();

    let console_path = parent_path.clone().join(console_sub_path);

    let mut game_map = BTreeMap::new();

    for game_path in fs::read_dir(console_path).unwrap() {
        let game_path = game_path.unwrap().path();
        let game_name = clean_game_image_filename(&game_path, &console);
        game_map.insert(game_name, game_path);
    }

    game_map
}

fn clean_game_image_filename(game_path: &path::PathBuf, console: &consoles::GameConsole) -> String {
    let cleaned_game_name = match console {
        consoles::GameConsole::GameBoyAdvance => nameutil::clean_filename_to_game_name(game_path),
        consoles::GameConsole::SNES => nameutil::clean_filename_to_game_name(game_path),
        consoles::GameConsole::NintendoDS => nameutil::clean_filename_to_game_name(game_path),
        consoles::GameConsole::Nintendo64 => nameutil::clean_filename_to_game_name(game_path),
        consoles::GameConsole::GameCube => name_passthrough(game_path),
        consoles::GameConsole::Wii => name_passthrough(game_path),
    };

    cleaned_game_name
}

fn name_passthrough(game_path: &path::PathBuf) -> String {
    let unstripped = game_path
        .file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("<unknown>".to_string());

    let stripped = unstripped
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    stripped
}
