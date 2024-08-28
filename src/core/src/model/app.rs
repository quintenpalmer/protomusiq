use std::fs;
use std::io;
use std::path;

use serde::Deserialize;

use crate::model::shared::SinkMode;
use crate::model::LoadMode;

#[derive(Debug, Clone)]
pub struct AppConfigState {
    pub library_path: path::PathBuf,
    pub movie_path: path::PathBuf,

    pub games: Option<GameConfig>,

    pub compressed_library_path: Option<path::PathBuf>,

    pub app_data_path: path::PathBuf,
    pub hostname: String,
    pub load_mode: Option<LoadMode>,
    pub sink_mode: Option<SinkMode>,

    pub split_ratio_left: u16,
    pub split_ratio_right: u16,

    pub grid_layout_width: u32,
    pub grid_layout_height: u32,
    pub grid_layout_track_multiplier: u32,

    pub scale_factor: f64,

    pub allowed_tracker_files: Option<Vec<path::PathBuf>>,
    pub allowed_prehistory_files: Option<Vec<path::PathBuf>>,
}

impl AppConfigState {
    pub fn get_safe_load_mode(&self) -> LoadMode {
        match self.load_mode {
            Some(ref v) => v.clone(),
            None => LoadMode::Json,
        }
    }

    pub fn get_safe_sink_mode(&self) -> SinkMode {
        match self.sink_mode {
            Some(ref v) => v.clone(),
            None => SinkMode::Local,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawAppConfigState {
    pub library_path: path::PathBuf,
    pub movie_path: path::PathBuf,

    pub games: Option<GameConfig>,

    pub compressed_library_path: Option<path::PathBuf>,

    pub hostname: String,
    pub load_mode: Option<LoadMode>,
    pub sink_mode: Option<SinkMode>,

    pub split_ratio_left: Option<u16>,
    pub split_ratio_right: Option<u16>,

    pub grid_layout_width: u32,
    pub grid_layout_height: u32,
    pub grid_layout_track_multiplier: u32,

    pub scale_factor: Option<f64>,

    pub allowed_tracker_files: Option<Vec<path::PathBuf>>,
    pub allowed_prehistory_files: Option<Vec<path::PathBuf>>,
}

impl RawAppConfigState {
    pub fn to_real<P: AsRef<path::Path>>(self, app_data_path: P) -> AppConfigState {
        AppConfigState {
            library_path: self.library_path,
            movie_path: self.movie_path,
            games: self.games,
            compressed_library_path: self.compressed_library_path,
            app_data_path: app_data_path.as_ref().to_path_buf(),
            hostname: self.hostname,
            load_mode: self.load_mode,
            sink_mode: self.sink_mode,
            split_ratio_left: self.split_ratio_left.unwrap_or(3),
            split_ratio_right: self.split_ratio_right.unwrap_or(2),
            grid_layout_width: self.grid_layout_width,
            grid_layout_height: self.grid_layout_height,
            grid_layout_track_multiplier: self.grid_layout_track_multiplier,
            scale_factor: self.scale_factor.unwrap_or(1.0),
            allowed_tracker_files: self.allowed_tracker_files,
            allowed_prehistory_files: self.allowed_prehistory_files,
        }
    }
}

impl AppConfigState {
    pub fn get_default() -> Self {
        let file = fs::File::open(crate::model::functions::get_default_config_path()).unwrap();
        let reader = io::BufReader::new(file);
        let raw_config_state: crate::model::app::RawAppConfigState =
            serde_json::from_reader(reader).unwrap();

        raw_config_state.to_real(crate::model::functions::get_default_data_path())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub preferred_region: String,
    pub image_path: path::PathBuf,
    pub gbc_path: path::PathBuf,
    pub gba_path: path::PathBuf,
    pub snes_path: path::PathBuf,
    pub n64_path: path::PathBuf,
    pub nds_path: path::PathBuf,
    pub gamecube: GameCubeWiiConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameCubeWiiConfig {
    pub metadata_path: path::PathBuf,
    pub gamecube_path: path::PathBuf,
    pub wii_path: path::PathBuf,
}
