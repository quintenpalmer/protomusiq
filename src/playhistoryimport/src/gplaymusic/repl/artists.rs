use std::collections::BTreeMap;
use std::io;

use musiqlibrary;

use crate::model;

use super::super::{gmodel};

pub fn prompt_user_for_artist_manual_mappings(
    raw_library: &musiqlibrary::RawLibrary,
    mut existing_manual_mapping: BTreeMap<String, musiqlibrary::ArtistInfo>,
    artist_album_track_vecs: Vec<(String, Vec<(String, Vec<gmodel::CleanedLineItem>)>)>,
) -> BTreeMap<String, musiqlibrary::ArtistInfo> {
    let mut artist_map = BTreeMap::new();

    for (artist_name, albums) in artist_album_track_vecs.into_iter() {
        if existing_manual_mapping.contains_key(&artist_name) {
            println!(
                "already found this artist: {} in existing manual mapping; skipping",
                artist_name,
            )
        } else {
            if model::exists_lowercase_artist(&raw_library, &artist_name) {
                println!(
                    "skipping artist {} prompt as it's not an artist that's missing",
                    artist_name
                );
            } else {
                println!(
                    "this artist: {} has been listened to this many times: {}",
                    artist_name,
                    gmodel::compute_artist_play_count(&albums),
                );
                let maybe_matched_artist = prompt_for_maybe_artist(&raw_library);
                match maybe_matched_artist {
                    Ok(artist) => {
                        artist_map.insert(artist_name.clone(), artist);
                        ()
                    }
                    Err(e) => match e {
                        Error::Skip => continue,
                        Error::Exit => break,
                    },
                };
            }
        }
    }

    artist_map.append(&mut existing_manual_mapping);

    artist_map
}

pub enum Error {
    Skip,
    Exit,
}

fn prompt_for_maybe_artist(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<musiqlibrary::ArtistInfo, Error> {
    println!("type search query for an artist");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return Err(Error::Exit);
    }
    if input == "skip" || input == "s" {
        return Err(Error::Skip);
    }

    let mut artists = Vec::new();
    for artist in raw_library.artists.values() {
        if artist
            .artist_info
            .artist_name
            .to_lowercase()
            .contains(&input.to_lowercase())
        {
            artists.push(artist.artist_info.clone());
        }
    }

    if artists.len() > 30 {
        println!("try a more narrow search query");
        match exact_match_artist(&raw_library, input.to_string()) {
            Some(exact_match_artists) => artists = exact_match_artists,
            None => {
                println!(
                    "couldn't find exact match after failing to find match containing; try again"
                );
                return prompt_for_maybe_artist(&raw_library);
            }
        };
    }

    if artists.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_for_maybe_artist(&raw_library);
    }

    select_artist(artists)
}

fn exact_match_artist(
    raw_library: &musiqlibrary::RawLibrary,
    input: String,
) -> Option<Vec<musiqlibrary::ArtistInfo>> {
    let mut artists = Vec::new();
    for artist in raw_library.artists.values() {
        if artist.artist_info.artist_name.to_lowercase() == input.to_lowercase() {
            artists.push(artist.artist_info.clone());
        }
    }

    if artists.len() > 30 {
        println!("try a more narrow search query, even for exact match");
        return None;
    }

    if artists.len() <= 0 {
        println!("couldn't find anything with that search query, even for exact match; try again");
        return None;
    }

    Some(artists)
}

fn select_artist(
    artists: Vec<musiqlibrary::ArtistInfo>,
) -> Result<musiqlibrary::ArtistInfo, Error> {
    for (i, track) in artists.iter().enumerate() {
        println!("{}:	{}", i, track.artist_name);
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
            if i < artists.len() {
                Ok(artists[i].clone())
            } else {
                println!("number must be within range of artists found");
                select_artist(artists)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_artist(artists)
        }
    }
}
