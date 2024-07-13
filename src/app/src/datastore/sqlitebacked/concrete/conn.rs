use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;

use chrono::{DateTime, Local};

use crate::datastore::localfs;
use crate::model;
use crate::util::logging;

use super::bootstrap::{migrate, seed};
use super::constants;
use super::dbmodel::bridge;
use super::query;
use super::util;

pub struct Connections {
    pub config: musiqcore::model::app::AppConfigState,
    pub shared_db: rusqlite::Connection,
}

impl Connections {
    pub fn first_bootup(config_state: musiqcore::model::app::AppConfigState) -> Self {
        let conn = Connections::new(config_state.clone());
        migrate::create_all_tables(config_state, &conn.shared_db);

        conn
    }

    fn new(config_state: musiqcore::model::app::AppConfigState) -> Self {
        let main_db = get_connection_from_config(config_state.clone());

        Connections {
            config: config_state,
            shared_db: main_db,
        }
    }

    pub fn spawn_connection(&self) -> Self {
        Connections::new(self.config.clone())
    }

    pub fn bootstrap_tracks(
        config_state: musiqcore::model::app::AppConfigState,
        tracks: &Vec<musiqlibrary::FullTrackMetadata>,
    ) -> Self {
        let mut logger = logging::Logger::new(logging::LogType::Timing, "seed tracks");

        logger.print_elapsed("starting loading (should be 0)");

        let mut conn = Connections::first_bootup(config_state);

        logger.print_elapsed("opened sqlite connection");

        if !conn.check_has_migration(constants::Migration::TracksAndFriends) {
            seed::seed_tracks(tracks, &mut conn.shared_db);

            logger.print_elapsed("seeded tracks");
        } else {
            logger.print_elapsed("skipping seeding tracks (migration says it was already there)");
        }

        conn
    }

    pub fn bootstrap_prehistory(&mut self, prehistory_records: &Vec<model::PrehistoryRecord>) {
        let mut logger = logging::Logger::new(logging::LogType::Timing, "seed prehistory");

        logger.print_elapsed("starting loading (should be 0)");

        if !self.check_has_migration(constants::Migration::Prehistory) {
            let artists = query::artists::select_artists(&self.shared_db);
            let albums = query::albums::select_albums(&self.shared_db);
            let discs = query::discs::select_discs(&self.shared_db);
            let tracks = query::tracks::select_tracks(&self.shared_db);

            let bridge_library =
                bridge::SixtyFourLibrary::from_db_rows(artists, albums, discs, tracks);

            logger.print_elapsed("built id bridge");

            seed::seed_prehistory(&mut self.shared_db, &bridge_library, prehistory_records);

            logger.print_elapsed("seeded prehistory");
        } else {
            logger
                .print_elapsed("skipping seeding prehistory (migration says it was already there)");
        };
    }

    pub fn bootstrap_livehistory(
        &mut self,
        livehistory_records: &BTreeMap<musiqlibrary::TrackUniqueIdentifier, Vec<DateTime<Local>>>,
    ) {
        let mut logger = logging::Logger::new(logging::LogType::Timing, "seed livehistory");

        logger.print_elapsed("starting loading (should be 0)");

        if !self.check_has_migration(constants::Migration::Livehistory) {
            let artists = query::artists::select_artists(&self.shared_db);
            let albums = query::albums::select_albums(&self.shared_db);
            let discs = query::discs::select_discs(&self.shared_db);
            let tracks = query::tracks::select_tracks(&self.shared_db);

            let bridge_library =
                bridge::SixtyFourLibrary::from_db_rows(artists, albums, discs, tracks);

            logger.print_elapsed("built id bridge");

            seed::seed_livehistory(&mut self.shared_db, &bridge_library, livehistory_records);

            logger.print_elapsed("seeded livehistory");
        } else {
            logger
                .print_elapsed("skipping seeding prehistory (migration says it was already there)");
        };
    }

    pub fn repopulate_tracks(&mut self, library: &musiqlibrary::RawLibrary) {
        let _timestamp = query::undeleted::repopulate_and_soft_delete(&mut self.shared_db, library);
    }

    pub fn _bootstrap_album_art(&mut self, _all_album_art: ()) {}

    pub fn check_has_migration(&self, migration: constants::Migration) -> bool {
        if !self.check_has_tables() {
            return false;
        }

        let mut statement = self
            .shared_db
            .prepare("SELECT count(*) FROM data_migrations WHERE name=?")
            .unwrap();
        let count: u32 = statement
            .query_row(rusqlite::params![migration.get_name()], |line| {
                let ret: u32 = line.get(0)?;
                Ok(ret)
            })
            .unwrap();
        count != 0
    }

    pub fn check_has_tables(&self) -> bool {
        let mut statement = self
            .shared_db
            .prepare(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='data_migrations'",
            )
            .unwrap();
        let count: u32 = statement
            .query_row([], |line| {
                let ret: u32 = line.get(0)?;
                Ok(ret)
            })
            .unwrap();
        count != 0
    }

    pub fn needs_tracks_seeded(&self) -> bool {
        !self.check_has_migration(constants::Migration::TracksAndFriends)
    }

    pub fn needs_prehistory_seeded(&self) -> bool {
        !self.check_has_migration(constants::Migration::Prehistory)
    }

    pub fn needs_livehistory_seeded(&self) -> bool {
        !self.check_has_migration(constants::Migration::Livehistory)
    }

    pub fn get_library(&self) -> musiqlibrary::RawLibrary {
        println!("calling sqlite's get library");
        let artists = query::artists::select_artists(&self.shared_db);
        let albums = query::albums::select_albums(&self.shared_db);
        let discs = query::discs::select_discs(&self.shared_db);
        let tracks = query::tracks::select_tracks(&self.shared_db);

        let albums_by_artist = util::key_into_vec_by(albums, |album| album.artist_id);
        let discs_by_album = util::key_into_vec_by(discs, |disc| disc.album_id);
        let tracks_by_disc = util::key_into_vec_by(tracks, |track| track.disc_id);

        let mut ret_library = musiqlibrary::RawLibrary {
            scan_prefix: Some(self.config.library_path.clone()),
            artists: BTreeMap::new(),
        };

        for artist in artists.iter() {
            let artist_id = musiqlibrary::ID::new(&artist.name);
            let mut artist_entry = musiqlibrary::KeyedArtistAlbums {
                artist_info: musiqlibrary::ArtistInfo {
                    artist_id,
                    artist_name: artist.name.clone(),
                },
                albums: BTreeMap::new(),
            };

            for album in albums_by_artist.get(&artist.id).unwrap() {
                let album_id = musiqlibrary::ID::new(&album.name);

                let mut album_discs = BTreeMap::new();

                let mut known_album_info: Option<musiqlibrary::AlbumInfo> = None;

                for disc in discs_by_album.get(&album.id).unwrap() {
                    let mut disc_entry = musiqlibrary::DiscTracks {
                        disc_no: disc.disc_no as u64,
                        tracks: BTreeMap::new(),
                    };

                    for track in tracks_by_disc.get(&disc.id).unwrap() {
                        known_album_info = known_album_info.map_or(
                            Some(musiqlibrary::AlbumInfo {
                                album_id,
                                album_name: album.name.clone(),
                                genres: BTreeSet::new(),
                                total_duration: track.duration,
                                start_date: album.date_number,
                                end_date: album.date_number,
                                last_modified: track.last_modified,
                                path: album.full_path.clone(),
                                relative_path: album.relative_path.clone(),
                            }),
                            |mut found| {
                                found.total_duration = found.total_duration.add(track.duration);

                                if track.last_modified > found.last_modified {
                                    found.last_modified = track.last_modified;
                                }

                                Some(found)
                            },
                        );

                        let full_track = musiqlibrary::FullTrackMetadata {
                            title: track.track_name.clone(),
                            track: track.track_no as u64,
                            raw_track: Some(track.track_no as u64),
                            disc: disc.disc_no as u64,
                            raw_disc: Some(disc.disc_no as u64),
                            disc_total: Some(album.disc_total as u64),
                            album: album.name.clone(),
                            raw_album: Some(album.name.clone()),
                            album_id: musiqlibrary::ID::new(&album.name),
                            album_artist: artist.name.clone(),
                            album_artist_id: musiqlibrary::ID::new(&artist.name),
                            track_artist: artist.name.clone(),
                            track_artist_id: musiqlibrary::ID::new(&artist.name),
                            genre: "replaceme".to_string(),
                            date_number: album.date_number,
                            raw_date: "replaceme".to_string(),
                            duration: track.duration,
                            path: track.full_path.clone(),
                            relative_path: track.relative_path.clone(),
                            last_modified: track.last_modified,
                            ext: track.ext.clone(),
                        };
                        disc_entry.tracks.insert(track.track_no as u64, full_track);
                    }

                    album_discs.insert(disc.disc_no as u64, disc_entry);
                }

                let album_entry = musiqlibrary::KeyedAlbumTracks {
                    album_info: known_album_info.unwrap(),
                    discs: album_discs,
                };

                artist_entry.albums.insert(album_id, album_entry);
            }

            ret_library.artists.insert(artist_id, artist_entry);
        }

        ret_library
    }

    pub fn get_historical_track_count(
        &self,
        id: &musiqlibrary::TrackUniqueIdentifier,
        bridge: &bridge::SixtyFourLibrary,
    ) -> usize {
        let track = bridge.track_from_unique_key(id);

        query::prehistory::get_historical_track_count(&self.shared_db, track.id)
    }

    pub fn get_all_historical_track_counts(&self) -> BTreeMap<u32, usize> {
        query::prehistory::get_all_historical_track_counts(&self.shared_db)
    }

    pub fn get_live_track_count(
        &self,
        id: &musiqlibrary::TrackUniqueIdentifier,
        bridge: &bridge::SixtyFourLibrary,
    ) -> usize {
        let track = bridge.track_from_unique_key(id);

        query::tracking::get_live_track_count(&self.shared_db, track.id)
    }

    pub fn get_all_live_track_counts(&self) -> BTreeMap<u32, usize> {
        query::tracking::get_all_live_track_counts(&self.shared_db)
    }

    pub fn increment_track_with_date(
        &mut self,
        bridge: &bridge::SixtyFourLibrary,
        track: &musiqlibrary::FullTrackMetadata,
        date_time: DateTime<Local>,
    ) {
        self.increment_tracks_with_dates(bridge, vec![(track.clone(), date_time)])
    }

    pub fn increment_tracks_with_dates(
        &mut self,
        bridge: &bridge::SixtyFourLibrary,
        tracks_with_dates: Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)>,
    ) {
        query::tracking::increment_tracks_with_dates(&mut self.shared_db, bridge, tracks_with_dates)
    }
}

fn get_connection_from_config(
    config_state: musiqcore::model::app::AppConfigState,
) -> rusqlite::Connection {
    let app_data_path = config_state.app_data_path.clone();

    let main_db_file = localfs::build_tree_for_file(&app_data_path, vec!["dbs", "main.db"]);

    let main_db =
        rusqlite::Connection::open(main_db_file).expect("non-sqlite database at expected path");

    main_db.execute(
"CREATE TABLE IF NOT EXISTS schema_migrations (
    id	INTEGER	PRIMARY KEY,
    migration_number	INTEGER	NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS schema_migrations_unique_number ON schema_migrations(migration_number);",
[],
        ).unwrap();

    main_db
}
