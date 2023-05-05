use std::collections::{BTreeMap, BTreeSet};
use std::io;

use musiqlibrary;

use super::super::smodel;

pub fn prompt_user_for_track_manual_mappings(
    raw_library: &musiqlibrary::RawLibrary,
    mut existing_manually_mapped: BTreeMap<
        smodel::SpotifyKey,
        (musiqlibrary::FullTrackMetadata, Vec<smodel::CleanedLineItem>),
    >,
    manual_ignore_albums: &BTreeSet<(String, String)>,
    not_initially_found: Vec<(smodel::SpotifyKey, Vec<smodel::CleanedLineItem>)>,
) -> (
    BTreeMap<smodel::SpotifyKey, (musiqlibrary::FullTrackMetadata, Vec<smodel::CleanedLineItem>)>,
    BTreeMap<smodel::SpotifyKey, Vec<smodel::CleanedLineItem>>,
) {
    let mut final_manually_mapped = BTreeMap::new();
    let mut not_ever_found = BTreeMap::new();
    let mut auto_skip = false;

    for (not_found_entry_key, plays) in not_initially_found.into_iter() {
        if auto_skip {
            not_ever_found.insert(not_found_entry_key, plays);
        } else {
            if existing_manually_mapped.contains_key(&not_found_entry_key) {
                println!(
                    "already found this key: {:?} in existing manual mapping; skipping",
                    not_found_entry_key
                )
            } else {
                let track_album_artist = (
                    not_found_entry_key.master_metadata_album_artist_name.to_lowercase(),
                    not_found_entry_key.master_metadata_album_album_name.to_lowercase(),
                );
                if manual_ignore_albums.contains(&track_album_artist) {
                    println!(
                        "skipping this track {:?} as it was found in the album ignore list",
                        not_found_entry_key
                    )
                } else {
                    println!(
                        "this track: {:?} has been listened to this many times: {}",
                        not_found_entry_key,
                        plays.len(),
                    );
                    let maybe_matched_track = prompt_for_maybe_track(&raw_library);
                    match maybe_matched_track {
                        Ok(track) => {
                            final_manually_mapped.insert(
                                not_found_entry_key,
                                (track.clone(), plays),
                            );
                            ()
                        }
                        Err(e) => {
                            not_ever_found.insert(not_found_entry_key, plays);
                            match e {
                                Error::Skip => (),
                                Error::Exit => {
                                    auto_skip = true;
                                }
                            }
                        }
                    };
                }
            }
        }
    }

    final_manually_mapped.append(&mut existing_manually_mapped);

    (final_manually_mapped, not_ever_found)
}

pub enum Error {
    Skip,
    Exit,
}

fn prompt_for_maybe_track(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<musiqlibrary::FullTrackMetadata, Error> {
    println!("type search query for a track");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return Err(Error::Exit);
    }
    if input == "skip" || input == "s" {
        return Err(Error::Skip);
    }

    let mut tracks = Vec::new();
    for artist in raw_library.artists.values() {
        for album in artist.albums.values() {
            for disc in album.discs.values() {
                for track in disc.tracks.values() {
                    if track.title.to_lowercase().contains(&input.to_lowercase()) {
                        tracks.push(track.clone());
                    }
                }
            }
        }
    }

    if tracks.len() > 30 {
        println!("try a more narrow search query");
        match exact_match_track(&raw_library, input.to_string()) {
            Some(exact_tracks) => tracks = exact_tracks,
            None => {
                println!(
                    "couldn't find exact match after failing to find match containing; try again"
                );
                return prompt_for_maybe_track(&raw_library);
            }
        };
    }

    if tracks.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_for_maybe_track(&raw_library);
    }

    select_track(tracks)
}

fn exact_match_track(
    raw_library: &musiqlibrary::RawLibrary,
    input: String,
) -> Option<Vec<musiqlibrary::FullTrackMetadata>> {
    let mut tracks = Vec::new();
    for artist in raw_library.artists.values() {
        for album in artist.albums.values() {
            for disc in album.discs.values() {
                for track in disc.tracks.values() {
                    if track.title.to_lowercase() == input.to_lowercase() {
                        tracks.push(track.clone());
                    }
                }
            }
        }
    }

    if tracks.len() > 30 {
        println!("try a more narrow search query, even for exact match");
        return None;
    }

    if tracks.len() <= 0 {
        println!("couldn't find anything with that search query, even for exact match; try again");
        return None;
    }

    Some(tracks)
}

fn select_track(
    tracks: Vec<musiqlibrary::FullTrackMetadata>,
) -> Result<musiqlibrary::FullTrackMetadata, Error> {
    for (i, track) in tracks.iter().enumerate() {
        println!(
            "{}:	{} - {} - {}",
            i, track.title, track.album, track.album_artist
        );
    }

    println!("select a track from above (0-29)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return Err(Error::Exit);
    }
    if input == "skip" || input == "s" {
        return Err(Error::Skip);
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < tracks.len() {
                Ok(tracks[i].clone())
            } else {
                println!("number must be within range of tracks found");
                select_track(tracks)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_track(tracks)
        }
    }
}
