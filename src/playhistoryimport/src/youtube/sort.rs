use std::collections::BTreeMap;

use crate::youtube::ytmodel;

pub fn sort_entries_by_song_max_play_count(
    entries: Vec<ytmodel::Entry>,
) -> Vec<(String, Vec<(String, Vec<String>)>)> {
    let mut by_artist_by_track_map: BTreeMap<String, BTreeMap<String, Vec<String>>> =
        BTreeMap::new();

    for entry in entries.into_iter() {
        let track_map = by_artist_by_track_map
            .entry(entry.artist)
            .or_insert(BTreeMap::new());

        let watched_vec = track_map.entry(entry.title).or_insert(Vec::new());

        watched_vec.push(entry.watched);
    }

    by_artist_by_track_map = by_artist_by_track_map
        .into_iter()
        .filter(|(_artist_, track_watch_map)| {
            track_watch_map
                .iter()
                .fold(0, |total, (_track, watch_info)| total + watch_info.len())
                > 1
        })
        .collect();

    let mut entry_by_title_sorted: Vec<(String, Vec<(String, Vec<String>)>)> =
        by_artist_by_track_map
            .into_iter()
            .map(|(artist, track_watched_vec)| (artist, track_watched_vec.into_iter().collect()))
            .collect();

    entry_by_title_sorted.sort_by_key(|(_artist, track_watch_ats)| {
        track_watch_ats
            .iter()
            .fold(0, |seen_max, (_track, watch_info)| {
                seen_max.max(watch_info.len())
            })
    });

    entry_by_title_sorted.reverse();

    for (artist, track_watched_ats) in entry_by_title_sorted.iter() {
        for (track, watch_info) in track_watched_ats.iter() {
            println!(
                "({} - {}) views: {}, artist: {}, title: {}",
                watch_info.first().unwrap(),
                watch_info.last().unwrap(),
                watch_info.len(),
                artist,
                track,
            );
        }
    }

    println!(
        "that was these with more than one view: {}",
        entry_by_title_sorted.len()
    );

    entry_by_title_sorted
}
