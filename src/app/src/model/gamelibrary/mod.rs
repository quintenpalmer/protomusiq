use std::path;

use musiqlibrary::games;

use crate::model;

pub struct GBAGame {
    pub name: String,
    pub path: path::PathBuf,
}

impl GBAGame {
    pub fn new(path: path::PathBuf) -> Self {
        GBAGame {
            name: clean_filename_to_game_name(&path),
            path: path.clone(),
        }
    }
}

fn clean_filename_to_game_name(path: &path::PathBuf) -> String {
    let mut unstripped = path
        .file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("<unknown>".to_string());

    unstripped = unstripped
        .strip_suffix("(Rev 1)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());
    unstripped = unstripped
        .strip_suffix("(Rev 2)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());
    unstripped = unstripped
        .strip_suffix("(Rev 3)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    unstripped = unstripped
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    unstripped = unstripped
        .strip_suffix("(USA)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());
    unstripped = unstripped
        .strip_suffix("(USA, Europe)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());
    unstripped = unstripped
        .strip_suffix("(USA, Australia)")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    unstripped = unstripped
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    unstripped
}

pub struct GameLibrary {
    inner: Option<InnerGameLibrary>,
}

impl GameLibrary {
    pub fn new(games: &Option<model::app::GameConfig>) -> Self {
        match games {
            Some(actual_games) => {
                let gba_rom_paths =
                    games::gba::scan_for_gba_rom_files(&actual_games.gba_path).unwrap();

                let mut sorted_rom_paths: Vec<GBAGame> =
                    gba_rom_paths.into_iter().map(|x| GBAGame::new(x)).collect();

                sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                let prefix_dir = actual_games.gba_path.clone();

                GameLibrary {
                    inner: Some(InnerGameLibrary {
                        gba_prefix_dir: prefix_dir,
                        gba_rom_paths: sorted_rom_paths,
                    }),
                }
            }
            None => GameLibrary { inner: None },
        }
    }
}

struct InnerGameLibrary {
    pub gba_prefix_dir: path::PathBuf,
    pub gba_rom_paths: Vec<GBAGame>,
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

    pub fn get_gba_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gba_prefix_dir),
            None => None,
        }
    }

    pub fn get_gba_rom_paths(&self) -> Option<&Vec<GBAGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gba_rom_paths),
            None => None,
        }
    }
}
