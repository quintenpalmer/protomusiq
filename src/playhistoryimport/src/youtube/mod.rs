mod convert;
mod find;
mod intermediate;
mod prompt;
mod sort;
mod util;
mod ytmodel;

use std::collections::BTreeMap;

use super::model;

pub fn translate_youtube_play_history() {
    let entries = util::get_entries();

    let keyed_by_artist_sorted_by_max_song_play_count =
        sort::sort_entries_by_song_max_play_count(entries);

    let raw_library = util::get_library();

    let mut perfect_artist_map: BTreeMap<String, String> = BTreeMap::new();

    let mut manual_artist_map: BTreeMap<String, String> = intermediate::load_intermediate_artists();

    let mut only_take_freebies = false;

    for (artist, track_watch_ats) in keyed_by_artist_sorted_by_max_song_play_count.iter() {
        println!(
            "artist: '{}' has {} for max views",
            artist,
            track_watch_ats
                .iter()
                .fold(0, |seen_max, (_track, watch_info)| {
                    seen_max.max(watch_info.len())
                })
        );
        match model::get_lowercase_artist(&raw_library, &artist) {
            Some(_found_artist) => {
                perfect_artist_map.insert(artist.clone(), artist.clone());
            }
            None => {
                if !manual_artist_map.contains_key(artist) {
                    if !only_take_freebies {
                        let maybe_confirmed_name =
                            prompt::prompt_user_for_artist_name(&raw_library, &artist);
                        match maybe_confirmed_name {
                            ytmodel::PromptResult::Answer(confirmed_name) => {
                                manual_artist_map.insert(artist.clone(), confirmed_name);
                            }
                            ytmodel::PromptResult::NothingFound => (),
                            ytmodel::PromptResult::Stop => only_take_freebies = true,
                        }
                    }
                } else {
                    println!("we already see '{}' in the manual mapping file", artist);
                }
            }
        }
    }

    for (yt_artist_name, library_artist_name) in perfect_artist_map.iter() {
        println!(
            "this artist just matched perfectly: {}: {}",
            yt_artist_name, library_artist_name
        );
    }

    for (yt_artist_name, library_artist_name) in manual_artist_map.iter() {
        println!(
            "matched this artist manually: {}: {}",
            yt_artist_name, library_artist_name
        );
    }

    intermediate::save_intermediate_artists(&manual_artist_map);

    for (artist, track_watch_ats) in keyed_by_artist_sorted_by_max_song_play_count.iter() {
        for (track_name, watched_ats) in track_watch_ats.iter() {
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
        }
    }
}
