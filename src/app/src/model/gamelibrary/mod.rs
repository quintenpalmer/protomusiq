use std::path;

use musiqlibrary::games;

pub struct GameLibrary {
    pub gba_rom_paths: Vec<path::PathBuf>,
}

impl GameLibrary {
    pub fn new<P: AsRef<path::Path>>(gba_path: &Option<P>) -> Self {
        let gba_rom_paths = match gba_path {
            Some(actual_gba_path) => games::gba::scan_for_gba_rom_files(&actual_gba_path).unwrap(),
            None => Vec::new(),
        };

        GameLibrary {
            gba_rom_paths: gba_rom_paths,
        }
    }
}

pub struct GameLibraryState {
    pub games: GameLibrary,
}

impl GameLibraryState {
    pub fn new(game_library: GameLibrary) -> Self {
        GameLibraryState {
            games: game_library,
        }
    }

    pub fn get_gba_rom_paths(&self) -> &Vec<path::PathBuf> {
        &self.games.gba_rom_paths
    }
}
