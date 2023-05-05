use std::fs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::super::localfs;

pub fn _create_new_raw_data<T: Serialize, S: Into<String>>(
    app_data_path: &PathBuf,
    child_list: Vec<S>,
    raw: T,
) -> (T, PathBuf) {
    let final_path = localfs::build_tree_for_file(&app_data_path, child_list);

    serde_json::to_writer(fs::File::create(final_path.clone()).unwrap(), &raw).unwrap();

    (raw, final_path)
}

pub fn maybe_get_existing_raw_data<T: DeserializeOwned>(json_db_path: &PathBuf) -> Option<T> {
    let maybe_raw: Option<T> = match fs::File::open(json_db_path.clone()) {
        Ok(reader) => match serde_json::from_reader(reader) {
            Ok(tracker) => Some(tracker),
            Err(_) => {
                println!(
                    "could not deserialize data from path: {:?}",
                    json_db_path.display()
                );
                None
            }
        },
        Err(_) => {
            println!("could not load file: {:?}", json_db_path.display());
            None
        }
    };

    maybe_raw
}

pub fn bootstrap_raw_data<T: Default + DeserializeOwned + Serialize, S: Into<String>>(
    app_data_path: &PathBuf,
    child_list: Vec<S>,
) -> (T, PathBuf) {
    let json_db_path = localfs::build_tree_for_file(&app_data_path, child_list);

    let maybe_raw = maybe_get_existing_raw_data(&json_db_path);

    let raw = match maybe_raw {
        None => {
            let empty = T::default();
            serde_json::to_writer(fs::File::create(json_db_path.clone()).unwrap(), &empty).unwrap();
            empty
        }
        Some(x) => x,
    };

    (raw, json_db_path)
}
