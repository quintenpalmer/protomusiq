use std::collections::BTreeMap;
use std::fs;
use std::path;

use serde::{Deserialize, Serialize};

use crate::model;

use super::common;

use musiqlibrary;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrehistoryPlayCountEntry {
    pub track: musiqlibrary::FullTrackMetadata,
    pub play_count: u32,
}

pub struct Reporter {
    pub historical_play_count_records: BTreeMap<musiqlibrary::TrackUniqueIdentifier, u32>,
}

impl Reporter {
    pub fn new(
        app_data_path: &path::PathBuf,
        allowed_prehistory_files: &Option<Vec<path::PathBuf>>,
    ) -> Self {
        let historical_play_count_vec =
            compute_historical_map(&app_data_path, allowed_prehistory_files);

        let mut historical_play_count_records = BTreeMap::new();

        for record in historical_play_count_vec.into_iter() {
            let existing_value = historical_play_count_records
                .get(&record.key)
                .unwrap_or(&0)
                .clone();

            historical_play_count_records.insert(record.key, existing_value + record.count);
        }

        Reporter {
            historical_play_count_records,
        }
    }

    pub fn get_track_count(&self, track_identifier: &musiqlibrary::TrackUniqueIdentifier) -> usize {
        match self.historical_play_count_records.get(track_identifier) {
            Some(v) => {
                //println!("I found a value for {:?}: {}", track_identifier, v);
                *v as usize
            }
            None => 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct PreHistoryContainer {
    name: String,
    records: Vec<PrehistoryPlayCountEntry>,
}

pub fn compute_historical_map(
    app_data_path: &path::PathBuf,
    allowed_prehistory_files: &Option<Vec<path::PathBuf>>,
) -> Vec<model::PrehistoryRecord> {
    let allowed_files = common::FileAllower::new(allowed_prehistory_files);

    let mut records = Vec::new();

    for historical_file in fs::read_dir(app_data_path.join("data").join("prehistory")).unwrap() {
        let historical_file = historical_file.unwrap().path();
        println!("prehistory play count file: {:?}", historical_file);
        if allowed_files.is_allowed(&historical_file) {
            match historical_file.extension().map(|x| x.to_str()) {
                Some(Some("json")) => {
                    let container: PreHistoryContainer =
                        common::maybe_get_existing_raw_data(&historical_file).unwrap();

                    let current_tracks = container.records;

                    for entry in current_tracks.into_iter() {
                        let track_unique_id =
                            musiqlibrary::TrackUniqueIdentifier::from_track(&entry.track);

                        records.push(model::PrehistoryRecord {
                            source: container.name.clone(),
                            key: track_unique_id,
                            count: entry.play_count,
                        });
                    }
                }
                Some(Some(ext)) => println!("skipping non json file ({}) in data/tracker/", ext),
                Some(None) => println!("skipping non json file in data/tracker/"),
                None => println!("skipping non json file in data/tracker/"),
            }
        } else {
            println!("skipping prehistory file, per config");
        }
    }
    records
}
