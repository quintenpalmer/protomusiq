use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datastore::loader;
use crate::datastore::sqlitebacked;
use crate::model;
use crate::util::logging;

use super::common;

#[derive(Serialize, Deserialize, Default)]
struct CacheMetadataPayload {
    pub info: Vec<musiqlibrary::SortedArtistAlbums<musiqlibrary::FullTrackMetadata>>,
}

impl CacheMetadataPayload {
    fn from_raw_library(raw_library: &musiqlibrary::RawLibrary) -> Self {
        arbitrary_serialize_sort(raw_library)
    }

    fn get_file_path_info(&self) -> Vec<musiqlibrary::TrackPathInfo> {
        let mut file_path_info = Vec::new();
        for artist in self.info.iter() {
            for album in artist.albums.iter() {
                for disc in album.discs.iter() {
                    for track in disc.tracks.iter() {
                        file_path_info.push(musiqlibrary::TrackPathInfo {
                            path: track.path.clone(),
                            relative_path: track.relative_path.clone(),
                            last_modified: track.last_modified,
                        });
                    }
                }
            }
        }
        file_path_info
    }

    fn get_full_track_list(&self) -> Vec<musiqlibrary::FullTrackMetadata> {
        let mut full_track_list = Vec::new();
        for artist in self.info.iter() {
            for album in artist.albums.iter() {
                for disc in album.discs.iter() {
                    for track in disc.tracks.iter() {
                        full_track_list.push(track.clone());
                    }
                }
            }
        }
        full_track_list
    }
}

impl loader::Loader {
    fn into_inner(&self) -> InnerLoadMode {
        match self {
            loader::Loader::NoCache => InnerLoadMode::NoCache,
            loader::Loader::Latest(conn) => {
                InnerLoadMode::Cached(CacheMode::Latest(conn.spawn_connection()))
            }
            loader::Loader::Json => {
                InnerLoadMode::Cached(CacheMode::Specified(SpecifiedCacheMode::Json))
            }
            loader::Loader::Sqlite(conn) => InnerLoadMode::Cached(CacheMode::Specified(
                SpecifiedCacheMode::Sqlite(conn.spawn_connection()),
            )),
        }
    }
}

enum InnerLoadMode {
    NoCache,
    Cached(CacheMode),
}

enum CacheMode {
    Latest(sqlitebacked::Connections),
    Specified(SpecifiedCacheMode),
}

enum SpecifiedCacheMode {
    Json,
    Sqlite(sqlitebacked::Connections),
}

enum Callback {
    Json(PathBuf),
    Sqlite(sqlitebacked::Connections),
    NoCache,
}

impl Callback {
    fn done(&mut self, organized: &musiqlibrary::RawLibrary) {
        match self {
            Callback::Json(metadata_json_file) => {
                let new_metadata_payload: CacheMetadataPayload =
                    arbitrary_serialize_sort(organized);

                serde_json::to_writer(
                    io::BufWriter::new(fs::File::create(metadata_json_file).unwrap()),
                    &new_metadata_payload,
                )
                .unwrap();
            }
            Callback::Sqlite(conn) => conn.repopulate_tracks(organized),
            Callback::NoCache => (),
        };
    }
}

pub fn load_library_from_cache_and_scan(
    config_state: &model::app::AppConfigState,
    loader: &loader::Loader,
    //public_load_mode: model::LoadMode,
) -> musiqlibrary::RawLibrary {
    let load_mode = loader.into_inner();

    let lib_path = config_state.library_path.clone();

    match load_mode {
        InnerLoadMode::NoCache => {
            println!("loading with no cache");
            musiqlibrary::RawLibrary::new(lib_path.clone()).unwrap()
        }
        InnerLoadMode::Cached(mode) => {
            println!("loading with a cache");
            let mut logger = logging::Logger::new(logging::LogType::Timing, "library load");

            logger.print_elapsed("starting loading (should be 0)");

            let (metadata_payload, mut callback): (CacheMetadataPayload, Callback) = match mode {
                CacheMode::Latest(conn) => {
                    println!("loading with auto (trying sqlite)");
                    let needs_migration = conn.needs_tracks_seeded();

                    match needs_migration {
                        false => {
                            println!("already has tracks migrated");
                            resolve_specified_cache_mode(
                                config_state,
                                SpecifiedCacheMode::Sqlite(conn),
                            )
                        }
                        true => {
                            println!("needs tracks migrated");
                            let raw_library =
                                musiqlibrary::RawLibrary::new(lib_path.clone()).unwrap();

                            let payload = CacheMetadataPayload::from_raw_library(&raw_library);

                            let tracks = payload.get_full_track_list();

                            sqlitebacked::Connections::bootstrap_tracks(
                                config_state.clone(),
                                &tracks,
                            );

                            // TODO is `Callback::NoCache` correct here?
                            // there shouldn't be any reason to anything fresh after seeding
                            // everything
                            (payload, Callback::NoCache)
                        }
                    }
                }
                CacheMode::Specified(specified_cache_mode) => {
                    println!("loading with specified");
                    resolve_specified_cache_mode(config_state, specified_cache_mode)
                }
            };

            logger.print_elapsed("scanning for metadata");

            println!(
                "found cached metadata with this many entries: {}",
                metadata_payload.info.len()
            );

            let cached_track_file_path_info = metadata_payload.get_file_path_info();

            logger.print_elapsed("getting file path info");

            println!(
                "length of cached metadata as file path info: {}",
                cached_track_file_path_info.len()
            );

            let storage_track_file_paths =
                musiqlibrary::find_only_files(&lib_path.clone()).unwrap();

            logger.print_elapsed("scanning library for files");

            println!(
                "length of stored file from hard drive: {}",
                storage_track_file_paths.len()
            );

            let (left, right) =
                compute_diff(&cached_track_file_path_info, &storage_track_file_paths);

            logger.print_elapsed("computing the diff");

            println!("diffs are: {}, {}", left.len(), right.len());

            let organized = if left.len() > 0 || right.len() > 0 {
                println!("found a diff so just loading entire library");

                let ret = musiqlibrary::RawLibrary::new(lib_path.clone()).unwrap();

                logger.print_elapsed("loading the whole library with cache mismatch");

                ret
            } else {
                println!("no diff seen, so loading metadata from cache");

                logger.print_elapsed("deciding to load from metadata");

                let ret = musiqlibrary::RawLibrary::from_track_list(
                    Some(&lib_path.clone()),
                    metadata_payload.get_full_track_list(),
                )
                .unwrap();

                logger.print_elapsed("loading the library from cache");

                ret
            };

            logger.print_elapsed("no longer serialize sorting");

            callback.done(&organized);

            logger.print_elapsed("writing the json back");

            organized
        }
    }
}

fn resolve_specified_cache_mode(
    config_state: &model::app::AppConfigState,
    specified_cache_mode: SpecifiedCacheMode,
) -> (CacheMetadataPayload, Callback) {
    match specified_cache_mode {
        SpecifiedCacheMode::Json => {
            println!("loading with JSON");
            let (inner_metadata_payload, metadata_json_file): (CacheMetadataPayload, PathBuf) =
                common::bootstrap_raw_data(
                    &config_state.app_data_path,
                    vec!["cache", "metadata", "tracks.json"],
                );
            (inner_metadata_payload, Callback::Json(metadata_json_file))
        }
        SpecifiedCacheMode::Sqlite(conn) => {
            println!("loading with sqlite");
            let raw_library = conn.get_library();
            let payload = CacheMetadataPayload::from_raw_library(&raw_library);
            (payload, Callback::Sqlite(conn))
        }
    }
}

fn arbitrary_serialize_sort(raw_library: &musiqlibrary::RawLibrary) -> CacheMetadataPayload {
    let artist_info: Vec<musiqlibrary::SortedArtistAlbums<musiqlibrary::FullTrackMetadata>> =
        raw_library
            .artists
            .values()
            .map(|artist| {
                let albums: Vec<musiqlibrary::SortedAlbumDiscs<musiqlibrary::FullTrackMetadata>> =
                    artist
                        .albums
                        .values()
                        .map(|album| {
                            let mut discs = Vec::new();
                            for disc in album.discs.values() {
                                let mut tracks = Vec::new();
                                for track in disc.tracks.values() {
                                    tracks.push(track.clone());
                                }
                                discs.push(musiqlibrary::SortedDiscTracks { tracks: tracks });
                            }
                            musiqlibrary::SortedAlbumDiscs {
                                album_info: album.album_info.clone(),
                                path: album.album_info.path.clone(),
                                discs: discs,
                            }
                        })
                        .collect();
                musiqlibrary::SortedArtistAlbums {
                    artist_info: artist.artist_info.clone(),
                    albums: albums,
                }
            })
            .collect();

    CacheMetadataPayload { info: artist_info }
}

fn compute_diff(
    input_left: &Vec<musiqlibrary::TrackPathInfo>,
    input_right: &Vec<musiqlibrary::TrackPathInfo>,
) -> (
    Vec<musiqlibrary::TrackPathInfo>,
    Vec<musiqlibrary::TrackPathInfo>,
) {
    let mut output_left = Vec::new();
    let mut output_right = Vec::new();

    let input_left_btree: BTreeMap<PathBuf, musiqlibrary::TrackPathInfo> = input_left
        .iter()
        .map(|x| (x.relative_path.clone(), x.clone()))
        .collect();

    let input_right_btree: BTreeMap<PathBuf, musiqlibrary::TrackPathInfo> = input_right
        .iter()
        .map(|x| (x.relative_path.clone(), x.clone()))
        .collect();

    for left in input_left.iter() {
        if !input_right_btree.contains_key(&left.relative_path) {
            output_left.push(left.clone());
        }
    }

    for right in input_right.iter() {
        if !input_left_btree.contains_key(&right.relative_path) {
            output_right.push(right.clone());
        }
    }

    (output_left, output_right)
}
