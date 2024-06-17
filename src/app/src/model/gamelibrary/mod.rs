use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;

use musiqlibrary::games;

use crate::model;

mod consoles;
mod images;
mod nameutil;

fn find_best_match(map: &BTreeMap<String, path::PathBuf>, key: String) -> Option<&path::PathBuf> {
    let mut best_so_far = (1000, None);

    for (iter_key, iter_value) in map.iter() {
        let iter_distance = model::functions::levenshtein(iter_key.as_str(), key.as_str());
        if iter_distance < best_so_far.0 {
            best_so_far = (iter_distance, Some(iter_value));
        }
    }

    best_so_far.1
}

fn get_game_image_bytes(
    image_map: &images::ConsoleGameImageMap,
    path: &path::PathBuf,
    game_console: consoles::GameConsole,
) -> Option<Vec<u8>> {
    let this_game_maybe_image_file = match (
        path.file_stem().map(|x| x.to_string_lossy().to_string()),
        image_map.get_console_map(&game_console),
    ) {
        (Some(game_path), Some(console_map)) => find_best_match(console_map, game_path),
        (_, _) => None,
    };

    match this_game_maybe_image_file {
        Some(image_path) => Some(fs::read(image_path).unwrap()),
        None => None,
    }
}

pub struct GBAGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Option<Vec<u8>>,
}

impl GBAGame {
    pub fn new(path: path::PathBuf, image_map: &images::ConsoleGameImageMap) -> Self {
        let loaded_image_bytes =
            get_game_image_bytes(image_map, &path, consoles::GameConsole::GameBoyAdvance);

        GBAGame {
            name: nameutil::clean_filename_to_game_name(&path),
            path: path.clone(),
            image: loaded_image_bytes,
        }
    }
}

pub struct SNESGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Option<Vec<u8>>,
}

impl SNESGame {
    pub fn new(path: path::PathBuf, image_map: &images::ConsoleGameImageMap) -> Self {
        let loaded_image_bytes =
            get_game_image_bytes(image_map, &path, consoles::GameConsole::SNES);

        SNESGame {
            name: nameutil::clean_filename_to_game_name(&path),
            path: path.clone(),
            image: loaded_image_bytes,
        }
    }
}

pub struct N64Game {
    pub name: String,
    pub path: path::PathBuf,
}

impl N64Game {
    pub fn new(path: path::PathBuf) -> Self {
        N64Game {
            name: nameutil::clean_filename_to_game_name(&path),
            path: path.clone(),
        }
    }
}

pub struct NDSGame {
    pub name: String,
    pub path: path::PathBuf,
}

impl NDSGame {
    pub fn new(path: path::PathBuf) -> Self {
        NDSGame {
            name: nameutil::clean_filename_to_game_name(&path),
            path: path.clone(),
        }
    }
}

pub struct GameCubeGame {
    pub name: String,
    pub code: String,
    pub path: path::PathBuf,
}

impl GameCubeGame {
    pub fn new(path: path::PathBuf, lookup_table: &BTreeMap<String, String>) -> Self {
        let code = extract_code_from_path(&path);
        let name = lookup_name_from_code(&code, lookup_table);

        GameCubeGame {
            name: name,
            code: code,
            path: path.clone(),
        }
    }
}

pub struct WiiGame {
    pub name: String,
    pub code: String,
    pub path: path::PathBuf,
}

impl WiiGame {
    pub fn new(path: path::PathBuf, lookup_table: &BTreeMap<String, String>) -> Self {
        let code = extract_code_from_path(&path);
        let name = lookup_name_from_code(&code, lookup_table);

        WiiGame {
            name: name,
            code: code,
            path: path.clone(),
        }
    }
}

fn extract_code_from_path(path: &path::PathBuf) -> String {
    path.file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("<unknown>".to_string())
}

fn lookup_name_from_code(code: &String, lookup_table: &BTreeMap<String, String>) -> String {
    let code_lookup = match code.strip_suffix("-disc2") {
        Some(v) => v.to_string(),
        None => code.clone(),
    };
    lookup_table.get(&code_lookup).unwrap().clone()
}

pub struct GameLibrary {
    inner: Option<InnerGameLibrary>,
}

impl GameLibrary {
    pub fn new(games: &Option<model::app::GameConfig>) -> Self {
        match games {
            Some(actual_games) => {
                let image_map = images::ConsoleGameImageMap::new(&actual_games.image_path);

                let (gba_prefix_dir, gba_rom_paths) = {
                    let rom_paths =
                        games::gba::scan_for_gba_rom_files(&actual_games.gba_path).unwrap();

                    let mut sorted_rom_paths: Vec<GBAGame> = rom_paths
                        .into_iter()
                        .map(|x| GBAGame::new(x, &image_map))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.gba_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (snes_prefix_dir, snes_rom_paths) = {
                    let rom_paths =
                        games::snes::scan_for_snes_rom_files(&actual_games.snes_path).unwrap();

                    let mut sorted_rom_paths: Vec<SNESGame> = rom_paths
                        .into_iter()
                        .map(|x| SNESGame::new(x, &image_map))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.snes_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (n64_prefix_dir, n64_rom_paths) = {
                    let rom_paths =
                        games::n64::scan_for_n64_rom_files(&actual_games.n64_path).unwrap();

                    let mut sorted_rom_paths: Vec<N64Game> =
                        rom_paths.into_iter().map(|x| N64Game::new(x)).collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.snes_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (nds_prefix_dir, nds_rom_paths) = {
                    let rom_paths =
                        games::nds::scan_for_nds_rom_files(&actual_games.nds_path).unwrap();

                    let mut sorted_rom_paths: Vec<NDSGame> =
                        rom_paths.into_iter().map(|x| NDSGame::new(x)).collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.snes_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let gamecube_metadata_path = actual_games.gamecube.metadata_path.clone();

                let gamecube_lookup_table = {
                    let file = fs::File::open(
                        gamecube_metadata_path.join("gamecube_wii_code_to_name_lookup_table.json"),
                    )
                    .unwrap();
                    let reader = io::BufReader::new(file);
                    let lookup_table: BTreeMap<String, String> =
                        serde_json::from_reader(reader).unwrap();

                    lookup_table
                };

                let (ngc_prefix_dir, ngc_rom_paths) = {
                    let rom_paths = games::gamecube::scan_for_gamecube_rom_files(
                        &actual_games.gamecube.gamecube_path,
                    )
                    .unwrap();

                    let mut sorted_rom_paths: Vec<GameCubeGame> = rom_paths
                        .into_iter()
                        .map(|x| GameCubeGame::new(x, &gamecube_lookup_table))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.snes_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (wii_prefix_dir, wii_rom_paths) = {
                    let rom_paths =
                        games::wii::scan_for_wii_rom_files(&actual_games.gamecube.wii_path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<WiiGame> = rom_paths
                        .into_iter()
                        .map(|x| WiiGame::new(x, &gamecube_lookup_table))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.snes_path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                GameLibrary {
                    inner: Some(InnerGameLibrary {
                        gba_prefix_dir: gba_prefix_dir,
                        gba_rom_paths: gba_rom_paths,
                        snes_prefix_dir: snes_prefix_dir,
                        snes_rom_paths: snes_rom_paths,
                        n64_prefix_dir: n64_prefix_dir,
                        n64_rom_paths: n64_rom_paths,
                        nds_prefix_dir: nds_prefix_dir,
                        nds_rom_paths: nds_rom_paths,
                        gamecube_prefix_dir: ngc_prefix_dir,
                        gamecube_rom_paths: ngc_rom_paths,
                        wii_prefix_dir: wii_prefix_dir,
                        wii_rom_paths: wii_rom_paths,
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

    pub snes_prefix_dir: path::PathBuf,
    pub snes_rom_paths: Vec<SNESGame>,

    pub n64_prefix_dir: path::PathBuf,
    pub n64_rom_paths: Vec<N64Game>,

    pub nds_prefix_dir: path::PathBuf,
    pub nds_rom_paths: Vec<NDSGame>,

    pub gamecube_prefix_dir: path::PathBuf,
    pub gamecube_rom_paths: Vec<GameCubeGame>,

    pub wii_prefix_dir: path::PathBuf,
    pub wii_rom_paths: Vec<WiiGame>,
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

    pub fn get_snes_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.snes_prefix_dir),
            None => None,
        }
    }

    pub fn get_snes_rom_paths(&self) -> Option<&Vec<SNESGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.snes_rom_paths),
            None => None,
        }
    }

    pub fn get_n64_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.n64_prefix_dir),
            None => None,
        }
    }

    pub fn get_n64_rom_paths(&self) -> Option<&Vec<N64Game>> {
        match self.games.inner {
            Some(ref v) => Some(&v.n64_rom_paths),
            None => None,
        }
    }

    pub fn get_nds_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.nds_prefix_dir),
            None => None,
        }
    }

    pub fn get_nds_rom_paths(&self) -> Option<&Vec<NDSGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.nds_rom_paths),
            None => None,
        }
    }

    pub fn get_ngc_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gamecube_prefix_dir),
            None => None,
        }
    }

    pub fn get_ngc_rom_paths(&self) -> Option<&Vec<GameCubeGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gamecube_rom_paths),
            None => None,
        }
    }

    pub fn get_wii_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.wii_prefix_dir),
            None => None,
        }
    }

    pub fn get_wii_rom_paths(&self) -> Option<&Vec<WiiGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.wii_rom_paths),
            None => None,
        }
    }
}
