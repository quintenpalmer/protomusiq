use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io;

//use chrono::{DateTime, Local};
use serde::de::DeserializeOwned;

use musiqcore::model::jsonbacked::tracker;

#[derive(Debug)]
pub enum Error {
    WrongNumberOfArguments,
}

pub fn entry_point() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_bin_name, _reconcile_flag, first_history_file, second_history_file] => {
            merge_two_files(first_history_file, second_history_file)
        }
        _ => {
            println!("must supply two history files to merge");
            Err(Error::WrongNumberOfArguments)
        }
    }
}

fn merge_two_files(first_history_file: &str, second_history_file: &str) -> Result<(), Error> {
    let first_history_raw_json: tracker::RawTrackedPayload = load_json(first_history_file).unwrap();
    let first_history_btree = first_history_raw_json.to_btree_map();

    let second_history_raw_json: tracker::RawTrackedPayload =
        load_json(second_history_file).unwrap();
    let second_history_btree = second_history_raw_json.to_btree_map();

    let mut merged_btree_of_listened_map = BTreeMap::new();
    for (track_id, listened_vec) in first_history_btree.into_iter() {
        let listened_map = merged_btree_of_listened_map
            .entry(track_id)
            .or_insert(BTreeSet::new());
        for listen in listened_vec.into_iter() {
            listened_map.insert(listen);
        }
    }
    for (track_id, listened_vec) in second_history_btree.into_iter() {
        let listened_map = merged_btree_of_listened_map
            .entry(track_id)
            .or_insert(BTreeSet::new());
        for listen in listened_vec.into_iter() {
            listened_map.insert(listen);
        }
    }

    let mut merged_listened_vec = BTreeMap::new();
    for (track_id, listened_btree_map) in merged_btree_of_listened_map.into_iter() {
        merged_listened_vec.insert(track_id, listened_btree_map.into_iter().collect::<Vec<_>>());
    }

    let merged_raw_payload = tracker::RawTrackedPayload::from_btree_map(&merged_listened_vec);

    println!(
        "{}",
        serde_json::to_string_pretty(&merged_raw_payload).unwrap()
    );

    Ok(())
}

fn load_json<T: DeserializeOwned>(filename: &str) -> Option<T> {
    match fs::File::open(filename) {
        Ok(reader) => match serde_json::from_reader(io::BufReader::new(reader)) {
            Ok(tracker) => Some(tracker),
            Err(e) => {
                println!("error!? {:?}", e);
                println!("could not deserialize data from path: {}", filename);
                None
            }
        },
        Err(e) => {
            println!("error!? {:?}", e);
            println!("could not load file: {}", filename);
            None
        }
    }
}
