use std::collections::BTreeSet;

use crate::youtube::ytmodel::PromptResult;

use super::userinput;
use super::intermediate;

pub fn get_ignore_artists(
    keyed_by_artist_sorted_by_max_song_play_count: &Vec<(String, Vec<(String, Vec<String>)>)>,
) -> BTreeSet<String> {
    let mut keyed_mut_clone = keyed_by_artist_sorted_by_max_song_play_count.clone();
    keyed_mut_clone.sort_by_key(
        |(_artist, track_watcheds)|
        track_watcheds
        .iter()
        .fold(
            0,
            |total, (_track, watched_ats)| total + watched_ats.len()
        )
    );
    keyed_mut_clone.reverse();

    let mut ignore = intermediate::load_intermediate_ignore_artists();

    for (artist, _track_watched_ats) in keyed_mut_clone.iter() {
        if ignore.contains(artist) {
            println!("already know to ignore {}", artist);
        } else {
            match userinput::prompt_user_for_ignore_artist(artist) {
                PromptResult::Answer(()) => {
                    let _ = ignore.insert(artist.clone());
                },
                PromptResult::Stop => {
                    println!("told to stop looking for ignore artists");
                    break;
                },
                PromptResult::NothingFound => println!("told to skip, which is to say, do not ignore"),
            }
        }
    }

    intermediate::save_intermediate_ignore_artists(&ignore);

    ignore
}
