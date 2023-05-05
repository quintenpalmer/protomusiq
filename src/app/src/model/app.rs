use std::path;

use serde::Deserialize;

use super::common::LoadMode;

#[derive(Debug, Clone)]
pub struct AppConfigState {
    pub library_path: path::PathBuf,
    pub app_data_path: path::PathBuf,
    pub hostname: String,
    pub load_mode: Option<LoadMode>,
}

impl AppConfigState {
    pub fn get_safe_load_mode(&self) -> LoadMode {
        match self.load_mode {
            Some(ref v) => v.clone(),
            None => LoadMode::JSON,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawAppConfigState {
    pub library_path: path::PathBuf,
    pub hostname: String,
    pub load_mode: Option<LoadMode>,
}

impl RawAppConfigState {
    pub fn to_real<P: AsRef<path::Path>>(self, app_data_path: P) -> AppConfigState {
        AppConfigState {
            library_path: self.library_path,
            app_data_path: app_data_path.as_ref().to_path_buf(),
            hostname: self.hostname,
            load_mode: self.load_mode,
        }
    }
}
