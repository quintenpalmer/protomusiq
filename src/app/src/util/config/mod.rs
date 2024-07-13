use std::fs;
use std::io;

pub fn get_default_config() -> musiqcore::model::app::AppConfigState {
    let file = fs::File::open(musiqcore::model::functions::get_default_config_path()).unwrap();
    let reader = io::BufReader::new(file);
    let raw_config_state: musiqcore::model::app::RawAppConfigState =
        serde_json::from_reader(reader).unwrap();

    raw_config_state.to_real(musiqcore::model::functions::get_default_data_path())
}
