use std::fs;
use std::io;

use crate::model;

pub fn get_default_config() -> model::AppConfigState {
    let file = fs::File::open(model::common::get_default_config_path()).unwrap();
    let reader = io::BufReader::new(file);
    let raw_config_state: model::RawAppConfigState = serde_json::from_reader(reader).unwrap();
    let config_state = raw_config_state.to_real(model::common::get_default_data_path());

    config_state
}
