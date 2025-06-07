use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path;

use serde::{Deserialize, Serialize};

use musiqcore::datastore::jsonbacked::common;
use musiqlibrary::video;

use crate::datastore::loader;
use crate::model;

#[derive(Serialize, Deserialize, Default)]
struct CacheMetadataPayload {
    pub info: Vec<video::MovieMetadata>,
}

pub fn load_library_from_cache_and_scan(
    config_state: &musiqcore::model::app::AppConfigState,
    loader: &loader::Loader,
) -> model::VideoLibrary {
    match loader {
        loader::Loader::NoCache => model::VideoLibrary::new(&config_state.movie_path),
        loader::Loader::Json => {
            let (metadata_payload, metadata_json_file): (CacheMetadataPayload, path::PathBuf) =
                common::bootstrap_raw_data(
                    &config_state.app_data_path,
                    vec!["cache", "metadata", "movies.json"],
                );

            let cached_movie_file_names = metadata_payload.get_file_path_info();

            let filesystem_movie_file_names =
                video::find_movie_paths(config_state.movie_path.clone());

            let (only_cached, only_filesystem) =
                compute_diff(&cached_movie_file_names, &filesystem_movie_file_names);

            println!(
                "diffs are: {}, {}",
                only_cached.len(),
                only_filesystem.len()
            );

            println!("{:?}", only_cached);
            println!("{:?}", only_filesystem);

            let organized = if only_cached.len() > 0 || only_filesystem.len() > 0 {
                println!("found a diff so just loading entire video library");

                let ret = model::VideoLibrary::new(&config_state.movie_path);

                ret
            } else {
                println!("no diff seen, so loading video metadata from cache");

                let ret = model::VideoLibrary::from_metadata_list(
                    //Some(&config_state.movie_path),
                    metadata_payload.get_full_movie_list(),
                );

                ret
            };

            let new_metadata_payload: CacheMetadataPayload = arbitrary_serialize_sort(&organized);

            serde_json::to_writer(
                io::BufWriter::new(fs::File::create(metadata_json_file).unwrap()),
                &new_metadata_payload,
            )
            .unwrap();

            organized
        }
        _ => panic!("not prepared to load movie metadata from sqlite"),
    }
}

impl CacheMetadataPayload {
    fn get_file_path_info(&self) -> Vec<path::PathBuf> {
        let mut file_path_info = Vec::new();
        for movie in self.info.iter() {
            file_path_info.push(movie.path.clone());
        }
        file_path_info
    }

    fn get_full_movie_list(&self) -> Vec<video::MovieMetadata> {
        let mut full_track_list = Vec::new();
        for movie in self.info.iter() {
            full_track_list.push(movie.clone());
        }
        full_track_list
    }
}

fn arbitrary_serialize_sort(raw_library: &model::VideoLibrary) -> CacheMetadataPayload {
    let mut ret = Vec::new();

    for movie in raw_library.movies.values() {
        ret.push(movie.clone());
    }

    CacheMetadataPayload { info: ret }
}

fn compute_diff(
    input_left: &Vec<path::PathBuf>,
    input_right: &Vec<path::PathBuf>,
) -> (Vec<path::PathBuf>, Vec<path::PathBuf>) {
    let mut output_left = Vec::new();
    let mut output_right = Vec::new();

    let input_left_btree: BTreeSet<path::PathBuf> = input_left.clone().into_iter().collect();

    let input_right_btree: BTreeSet<path::PathBuf> = input_right.clone().into_iter().collect();

    for left in input_left.iter() {
        if !input_right_btree.contains(left) {
            output_left.push(left.clone());
        }
    }

    for right in input_right.iter() {
        if !input_left_btree.contains(right) {
            output_right.push(right.clone());
        }
    }

    (output_left, output_right)
}
