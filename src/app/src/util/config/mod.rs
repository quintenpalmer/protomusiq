use std::fs;
use std::io;

use crate::model;

pub fn get_default_config() -> musiqcore::model::app::AppConfigState {
    let file = fs::File::open(model::functions::get_default_config_path()).unwrap();
    let reader = io::BufReader::new(file);
    let raw_config_state: musiqcore::model::app::RawAppConfigState =
        serde_json::from_reader(reader).unwrap();

    raw_config_state.to_real(model::functions::get_default_data_path())
}
