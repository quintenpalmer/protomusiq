use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;

use musiqlibrary::games;

pub mod consoles;
mod images;
mod nameutil;

pub enum ImageMode {
    BestMatch(path::PathBuf),
    ExactMatch,
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

    pub fn get_spawn_command(
        &self,
        console: &consoles::GameConsole,
    ) -> Option<(String, Vec<String>)> {
        match console {
            consoles::GameConsole::GameBoy => match self.games.inner {
                Some(ref v) => Some((v.gb.emu_cmd.clone(), v.gb.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::GameBoyColor => match self.games.inner {
                Some(ref v) => Some((v.gbc.emu_cmd.clone(), v.gbc.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::GameBoyAdvance => match self.games.inner {
                Some(ref v) => Some((v.gba.emu_cmd.clone(), v.gba.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::NintendoDS => match self.games.inner {
                Some(ref v) => Some((v.nds.emu_cmd.clone(), v.nds.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::SNES => match self.games.inner {
                Some(ref v) => Some((v.snes.emu_cmd.clone(), v.snes.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::Nintendo64 => match self.games.inner {
                Some(ref v) => Some((v.n64.emu_cmd.clone(), v.n64.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::GameCube => match self.games.inner {
                Some(ref v) => Some((v.gamecube.emu_cmd.clone(), v.gamecube.emu_cmd_args.clone())),
                None => None,
            },
            consoles::GameConsole::Wii => match self.games.inner {
                Some(ref v) => Some((v.wii.emu_cmd.clone(), v.wii.emu_cmd_args.clone())),
                None => None,
            },
        }
    }

    pub fn get_generic_game_and_prefix(
        &self,
        console: &consoles::GameConsole,
    ) -> Option<(Vec<Box<&dyn GenericGame>>, &path::PathBuf)> {
        match console {
            consoles::GameConsole::GameBoy => Some((
                self.get_gb_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_gb_prefix_path()?,
            )),
            consoles::GameConsole::GameBoyColor => Some((
                self.get_gbc_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_gbc_prefix_path()?,
            )),
            consoles::GameConsole::GameBoyAdvance => Some((
                self.get_gba_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_gba_prefix_path()?,
            )),
            consoles::GameConsole::NintendoDS => Some((
                self.get_nds_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_nds_prefix_path()?,
            )),
            consoles::GameConsole::SNES => Some((
                self.get_snes_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_snes_prefix_path()?,
            )),
            consoles::GameConsole::Nintendo64 => Some((
                self.get_n64_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_n64_prefix_path()?,
            )),
            consoles::GameConsole::GameCube => Some((
                self.get_ngc_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_ngc_prefix_path()?,
            )),
            consoles::GameConsole::Wii => Some((
                self.get_wii_rom_paths()?
                    .iter()
                    .map(|x| Box::new(x as &dyn GenericGame))
                    .collect(),
                self.get_wii_prefix_path()?,
            )),
        }
    }

    pub fn get_console_prefix(&self, console: &consoles::GameConsole) -> Option<&path::PathBuf> {
        match console {
            consoles::GameConsole::GameBoy => self.get_gb_prefix_path(),
            consoles::GameConsole::GameBoyColor => self.get_gbc_prefix_path(),
            consoles::GameConsole::GameBoyAdvance => self.get_gba_prefix_path(),
            consoles::GameConsole::NintendoDS => self.get_nds_prefix_path(),
            consoles::GameConsole::SNES => self.get_snes_prefix_path(),
            consoles::GameConsole::Nintendo64 => self.get_n64_prefix_path(),
            consoles::GameConsole::GameCube => self.get_ngc_prefix_path(),
            consoles::GameConsole::Wii => self.get_wii_prefix_path(),
        }
    }

    pub fn get_gb_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gb.prefix_dir),
            None => None,
        }
    }

    pub fn get_gb_rom_paths(&self) -> Option<&Vec<GBGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gb.rom_paths),
            None => None,
        }
    }

    pub fn get_gbc_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gbc.prefix_dir),
            None => None,
        }
    }

    pub fn get_gbc_rom_paths(&self) -> Option<&Vec<GBCGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gbc.rom_paths),
            None => None,
        }
    }

    pub fn get_gba_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gba.prefix_dir),
            None => None,
        }
    }

    pub fn get_gba_rom_paths(&self) -> Option<&Vec<GBAGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gba.rom_paths),
            None => None,
        }
    }

    pub fn get_snes_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.snes.prefix_dir),
            None => None,
        }
    }

    pub fn get_snes_rom_paths(&self) -> Option<&Vec<SNESGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.snes.rom_paths),
            None => None,
        }
    }

    pub fn get_n64_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.n64.prefix_dir),
            None => None,
        }
    }

    pub fn get_n64_rom_paths(&self) -> Option<&Vec<N64Game>> {
        match self.games.inner {
            Some(ref v) => Some(&v.n64.rom_paths),
            None => None,
        }
    }

    pub fn get_nds_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.nds.prefix_dir),
            None => None,
        }
    }

    pub fn get_nds_rom_paths(&self) -> Option<&Vec<NDSGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.nds.rom_paths),
            None => None,
        }
    }

    pub fn get_ngc_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.gamecube.prefix_dir),
            None => None,
        }
    }

    pub fn get_ngc_rom_paths(&self) -> Option<&Vec<GameCubeGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.gamecube.rom_paths),
            None => None,
        }
    }

    pub fn get_wii_prefix_path(&self) -> Option<&path::PathBuf> {
        match self.games.inner {
            Some(ref v) => Some(&v.wii.prefix_dir),
            None => None,
        }
    }

    pub fn get_wii_rom_paths(&self) -> Option<&Vec<WiiGame>> {
        match self.games.inner {
            Some(ref v) => Some(&v.wii.rom_paths),
            None => None,
        }
    }
}

pub struct GameLibrary {
    inner: Option<InnerGameLibrary>,
}

impl GameLibrary {
    pub fn new(image_mode: &ImageMode, games: &Option<crate::model::app::GameConfig>) -> Self {
        match games {
            Some(actual_games) => {
                let source_image_path = match image_mode {
                    ImageMode::BestMatch(best_match_source_path) => best_match_source_path,
                    ImageMode::ExactMatch => &actual_games.image_path,
                };

                let image_map = images::ConsoleGameImageMap::new(
                    source_image_path,
                    actual_games.preferred_region.clone(),
                );

                let (gb_prefix_dir, gb_rom_paths) = {
                    let rom_paths =
                        games::gb::scan_for_gb_rom_files(&actual_games.consoles.gb.path).unwrap();

                    let mut sorted_rom_paths: Vec<GBGame> = rom_paths
                        .into_iter()
                        .map(|x| GBGame::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.gb.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (gbc_prefix_dir, gbc_rom_paths) = {
                    let rom_paths =
                        games::gbc::scan_for_gbc_rom_files(&actual_games.consoles.gbc.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<GBCGame> = rom_paths
                        .into_iter()
                        .map(|x| GBCGame::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.gbc.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (gba_prefix_dir, gba_rom_paths) = {
                    let rom_paths =
                        games::gba::scan_for_gba_rom_files(&actual_games.consoles.gba.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<GBAGame> = rom_paths
                        .into_iter()
                        .map(|x| GBAGame::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.gba.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (snes_prefix_dir, snes_rom_paths) = {
                    let rom_paths =
                        games::snes::scan_for_snes_rom_files(&actual_games.consoles.snes.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<SNESGame> = rom_paths
                        .into_iter()
                        .map(|x| SNESGame::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.snes.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (n64_prefix_dir, n64_rom_paths) = {
                    let rom_paths =
                        games::n64::scan_for_n64_rom_files(&actual_games.consoles.n64.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<N64Game> = rom_paths
                        .into_iter()
                        .map(|x| N64Game::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.n64.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (nds_prefix_dir, nds_rom_paths) = {
                    let rom_paths =
                        games::nds::scan_for_nds_rom_files(&actual_games.consoles.nds.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<NDSGame> = rom_paths
                        .into_iter()
                        .map(|x| NDSGame::new(x, &image_map, image_mode, &actual_games))
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.nds.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let gamecube_metadata_path = actual_games.metadata_path.clone();

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
                        &actual_games.consoles.gamecube.path,
                    )
                    .unwrap();

                    let mut sorted_rom_paths: Vec<GameCubeGame> = rom_paths
                        .into_iter()
                        .map(|x| {
                            GameCubeGame::new(
                                x,
                                &gamecube_lookup_table,
                                &image_map,
                                image_mode,
                                &actual_games,
                            )
                        })
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.gamecube.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                let (wii_prefix_dir, wii_rom_paths) = {
                    let rom_paths =
                        games::wii::scan_for_wii_rom_files(&actual_games.consoles.wii.path)
                            .unwrap();

                    let mut sorted_rom_paths: Vec<WiiGame> = rom_paths
                        .into_iter()
                        .map(|x| {
                            WiiGame::new(
                                x,
                                &gamecube_lookup_table,
                                &image_map,
                                image_mode,
                                &actual_games,
                            )
                        })
                        .collect();

                    sorted_rom_paths.sort_by_key(|x| x.name.clone().to_lowercase());

                    let prefix_dir = actual_games.consoles.wii.path.clone();

                    (prefix_dir, sorted_rom_paths)
                };

                GameLibrary {
                    inner: Some(InnerGameLibrary {
                        gb: ConsoleInfo {
                            prefix_dir: gb_prefix_dir,
                            rom_paths: gb_rom_paths,
                            emu_cmd: actual_games.consoles.gb.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .gb
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        gbc: ConsoleInfo {
                            prefix_dir: gbc_prefix_dir,
                            rom_paths: gbc_rom_paths,
                            emu_cmd: actual_games.consoles.gbc.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .gbc
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        gba: ConsoleInfo {
                            prefix_dir: gba_prefix_dir,
                            rom_paths: gba_rom_paths,
                            emu_cmd: actual_games.consoles.gba.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .gba
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        snes: ConsoleInfo {
                            prefix_dir: snes_prefix_dir,
                            rom_paths: snes_rom_paths,
                            emu_cmd: actual_games.consoles.snes.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .snes
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        n64: ConsoleInfo {
                            prefix_dir: n64_prefix_dir,
                            rom_paths: n64_rom_paths,
                            emu_cmd: actual_games.consoles.n64.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .n64
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        nds: ConsoleInfo {
                            prefix_dir: nds_prefix_dir,
                            rom_paths: nds_rom_paths,
                            emu_cmd: actual_games.consoles.nds.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .nds
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        gamecube: ConsoleInfo {
                            prefix_dir: ngc_prefix_dir,
                            rom_paths: ngc_rom_paths,
                            emu_cmd: actual_games.consoles.gamecube.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .gamecube
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                        wii: ConsoleInfo {
                            prefix_dir: wii_prefix_dir,
                            rom_paths: wii_rom_paths,
                            emu_cmd: actual_games.consoles.wii.emu_cmd.clone(),
                            emu_cmd_args: actual_games
                                .consoles
                                .wii
                                .emu_cmd_args
                                .clone()
                                .unwrap_or(Vec::new()),
                        },
                    }),
                }
            }
            _ => GameLibrary { inner: None },
        }
    }
}

struct InnerGameLibrary {
    pub gb: ConsoleInfo<GBGame>,
    pub gbc: ConsoleInfo<GBCGame>,
    pub gba: ConsoleInfo<GBAGame>,
    pub snes: ConsoleInfo<SNESGame>,
    pub n64: ConsoleInfo<N64Game>,
    pub nds: ConsoleInfo<NDSGame>,
    pub gamecube: ConsoleInfo<GameCubeGame>,
    pub wii: ConsoleInfo<WiiGame>,
}

struct ConsoleInfo<T> {
    pub prefix_dir: path::PathBuf,
    pub rom_paths: Vec<T>,
    pub emu_cmd: String,
    pub emu_cmd_args: Vec<String>,
}

pub trait GenericGame {
    fn get_name(&self) -> String;
    fn get_rom_path(&self) -> &path::PathBuf;
    fn get_matched_source_image_path(&self) -> &path::PathBuf;
}

pub struct GBGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl GBGame {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::GameBoy,
        );

        GBGame {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for GBGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct GBCGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl GBCGame {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::GameBoyColor,
        );

        GBCGame {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for GBCGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct GBAGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl GBAGame {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::GameBoyAdvance,
        );

        GBAGame {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for GBAGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct SNESGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl SNESGame {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::SNES,
        );

        SNESGame {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for SNESGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct N64Game {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl N64Game {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::Nintendo64,
        );

        N64Game {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for N64Game {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct NDSGame {
    pub name: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl NDSGame {
    pub fn new(
        path: path::PathBuf,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let name = nameutil::clean_filename_to_game_name(&path);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::NintendoDS,
        );

        NDSGame {
            name,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for NDSGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct GameCubeGame {
    pub name: String,
    pub code: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl GameCubeGame {
    pub fn new(
        path: path::PathBuf,
        lookup_table: &BTreeMap<String, String>,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let code = extract_code_from_path(&path);
        let name = lookup_name_from_code(&code, lookup_table);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::GameCube,
        );

        GameCubeGame {
            name: name,
            code: code,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for GameCubeGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
    }
}

pub struct WiiGame {
    pub name: String,
    pub code: String,
    pub path: path::PathBuf,
    pub image: Vec<u8>,
    pub image_path: path::PathBuf,
}

impl WiiGame {
    pub fn new(
        path: path::PathBuf,
        lookup_table: &BTreeMap<String, String>,
        image_map: &images::ConsoleGameImageMap,
        image_mode: &ImageMode,
        game_config: &crate::model::app::GameConfig,
    ) -> Self {
        let code = extract_code_from_path(&path);
        let name = lookup_name_from_code(&code, lookup_table);

        let (loaded_image_bytes, loaded_image_path) = get_game_image_bytes(
            image_map,
            &path,
            image_mode,
            game_config,
            consoles::GameConsole::Wii,
        );

        WiiGame {
            name: name,
            code: code,
            path: path.clone(),
            image: loaded_image_bytes,
            image_path: loaded_image_path,
        }
    }
}

impl GenericGame for WiiGame {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_rom_path(&self) -> &path::PathBuf {
        &self.path
    }

    fn get_matched_source_image_path(&self) -> &path::PathBuf {
        &self.image_path
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

fn get_game_image_bytes(
    image_map: &images::ConsoleGameImageMap,
    path: &path::PathBuf,
    image_mode: &ImageMode,
    game_config: &crate::model::app::GameConfig,
    game_console: consoles::GameConsole,
) -> (Vec<u8>, path::PathBuf) {
    let console_map = image_map.get_console_map(&game_console);

    match image_mode {
        ImageMode::BestMatch(_) => {
            let name = nameutil::clean_filename_to_game_name(&path);

            let this_game_image_file =
                find_best_match(console_map, name, image_map.get_preferred_region());

            (
                fs::read(this_game_image_file.clone()).unwrap(),
                this_game_image_file,
            )
        }
        ImageMode::ExactMatch => {
            let dest_prefix = &game_config.image_path;

            let dest_console_piece = path::PathBuf::from(game_console.full_name());

            let dest_base_name =
                path::PathBuf::from(path.file_name().unwrap()).with_extension("png");

            let dest_image_path = dest_prefix.join(dest_console_piece).join(dest_base_name);

            (fs::read(dest_image_path.clone()).unwrap(), dest_image_path)
        }
    }
}

fn find_best_match(
    map: &BTreeMap<path::PathBuf, String>,
    key: String,
    preferred_region: String,
) -> path::PathBuf {
    let mut best_so_far = (1000, Vec::new());

    for (iter_value, iter_key) in map.iter() {
        let iter_key = nameutil::clean_filename_to_game_name(&path::PathBuf::from(iter_key));
        let iter_distance = crate::model::functions::levenshtein(iter_key.as_str(), key.as_str());
        if iter_distance < best_so_far.0 {
            best_so_far = (iter_distance, vec![iter_value]);
        } else if iter_distance == best_so_far.0 {
            best_so_far.1.push(iter_value);
        }
    }

    match best_so_far.1.as_slice() {
        [] => panic!("there should always be at least one match, even if it was really bad"),
        matches @ [_, ..] => {
            let mut ret = matches[0].clone();
            for m in matches.into_iter() {
                if nameutil::get_game_region_info(m).contains(&preferred_region) {
                    ret = m.to_path_buf();
                }
            }
            ret
        }
    }
}
