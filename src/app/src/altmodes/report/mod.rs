use std::collections::BTreeMap;

use chrono::TimeZone;

use crate::datastore::jsonbacked;
use crate::model;

#[derive(Debug)]
pub enum Error {}

pub fn generate_year_end_report() -> Result<(), Error> {
    let config_state = musiqcore::model::app::AppConfigState::get_default();

    let lib_path = config_state.library_path.clone();
    let organized = musiqlibrary::RawLibrary::new(lib_path.clone()).unwrap();

    let read_only_tracker = jsonbacked::tracker::ReadOnlyTracker::new(
        &config_state.app_data_path.to_path_buf(),
        config_state.hostname.clone(),
        &None,
    );

    let augmented = organized.map_into(&|track| {
        let uniq_track_id = musiqlibrary::TrackUniqueIdentifier::from_track(&track);
        let last_year_play_history = {
            let all_history_play_history = read_only_tracker.get_track_history(&uniq_track_id);
            match all_history_play_history {
                Some(v) => v
                    .iter()
                    .filter(|x| {
                        x >= &&chrono::Local
                            .from_local_datetime(
                                &chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
                                    .unwrap()
                                    .and_hms_opt(0, 0, 0)
                                    .unwrap(),
                            )
                            .unwrap()
                    })
                    .collect(),
                None => Vec::new(),
            }
        };

        model::AugmentedTrack {
            augmented: model::AugmentedData {
                play_count: last_year_play_history.len(),
                tagged_genres: Vec::new(),
            },
            metadata: track,
        }
    });

    let mut most_listened_to_artists = BTreeMap::new();
    let mut most_listened_to_albums = BTreeMap::new();
    let mut most_listened_to_tracks = BTreeMap::new();

    for artist in augmented.artists.values() {
        for album in artist.albums.values() {
            for disc in album.discs.values() {
                for track in disc.tracks.values() {
                    *most_listened_to_artists
                        .entry(artist.artist_info.artist_name.clone())
                        .or_insert(0) += track.augmented.play_count;

                    *most_listened_to_albums
                        .entry((
                            artist.artist_info.artist_name.clone(),
                            album.album_info.album_name.clone(),
                        ))
                        .or_insert(0) += track.augmented.play_count;

                    *most_listened_to_tracks
                        .entry((
                            artist.artist_info.artist_name.clone(),
                            album.album_info.album_name.clone(),
                            track.metadata.title.clone(),
                        ))
                        .or_insert(0) += track.augmented.play_count;
                }
            }
        }
    }

    for (name, count) in most_listened_to_artists.iter() {
        println!("{}\t{}", count, name);
    }
    println!("|||||||||||||||||||||||||");
    for ((artist, album), count) in most_listened_to_albums.iter() {
        println!("{}\t{}\t{}", count, album, artist);
    }
    println!("+++++++++++++++++++++++++");
    for ((artist, album, track), count) in most_listened_to_tracks.iter() {
        println!("{}\t{}\t{}\t{}", count, track, album, artist);
    }

    Ok(())
}
