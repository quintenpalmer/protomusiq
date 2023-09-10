use std::collections::{BTreeMap, BTreeSet};

use super::super::{sort, ytmodel};

use super::{intermediate, userinput};

pub fn get_all_track_info_resolved(
    keyed_by_artist_sorted_by_max_song_play_count: &Vec<(String, Vec<(String, Vec<String>)>)>,
    raw_library: &musiqlibrary::RawLibrary,
    perfect_artist_map: &BTreeMap<String, String>,
    manual_artist_map: &BTreeMap<String, String>,
    ignore_artists: &BTreeSet<String>,
) -> BTreeMap<String, musiqlibrary::TrackUniqueIdentifier> {
    let mut manual_track_map = intermediate::load_intermediate_tracks();

    let mut keep_looping = true;

    let folded =
        sort::sort_by_track_play_count_folded(keyed_by_artist_sorted_by_max_song_play_count);

    for ((artist, track_name), watched_ats) in folded.iter() {
        if keep_looping {
            match manual_track_map.get(track_name) {
                Some(found_track) => {
                    println!("already found track, matched as: {:?}", found_track);
                }
                None => {
                    let matched_artist = match perfect_artist_map.get(artist) {
                        Some(matched_a) => Some(matched_a),
                        None => match manual_artist_map.get(artist) {
                            Some(matched_a) => Some(matched_a),
                            None => None,
                        },
                    };
                    println!(
                        "artist '{}' (matched as {:?}) has track '{}' with {} views",
                        artist,
                        matched_artist,
                        track_name,
                        watched_ats.len()
                    );
                    match matched_artist {
                        Some(_artist) => {
                            println!("found an actual artist, will only search their songs");
                        }
                        None => {
                            println!("could not find an actual artist, will search all songs");
                        }
                    };

                    if ignore_artists.contains(artist) {
                        println!("artist: {} is in the ignore list, so skipping", artist);
                    } else {
                        let track_id_result = userinput::prompt_user_for_track(
                            raw_library,
                            matched_artist,
                            track_name,
                            watched_ats,
                        );

                        match track_id_result {
                            ytmodel::PromptResult::Answer(confirmed_track) => {
                                println!("found this maybe track_id: {:?}", confirmed_track);
                                manual_track_map.insert(track_name.clone(), confirmed_track);
                            }
                            ytmodel::PromptResult::NothingFound => {
                                println!("skipping this track: {}", track_name)
                            }
                            ytmodel::PromptResult::Stop => {
                                println!("breaking as instructed");
                                keep_looping = false;
                            }
                        }
                    }
                }
            }
        }
    }

    intermediate::save_intermediate_tracks(&manual_track_map);

    manual_track_map
}
