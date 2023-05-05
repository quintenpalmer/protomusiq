use std::collections::{BTreeMap, BTreeSet};
use std::io;

use crate::model;

use super::super::{gmodel};

use musiqlibrary;

pub fn prompt_user_for_artist_album_manual_mappings(
    raw_library: &musiqlibrary::RawLibrary,
    existing_manual_mapping: BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    existing_ignore_albums: BTreeSet<(String, String)>,
    artist_mapping: &BTreeMap<String, musiqlibrary::ArtistInfo>,
    artist_album_track_vecs: Vec<(String, Vec<(String, Vec<gmodel::CleanedLineItem>)>)>,
) -> (
    BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    BTreeSet<(String, String)>,
) {
    let mut artist_album_map = BTreeMap::new();

    let mut ignore_albums = BTreeSet::new();

    for (orig_artist_name, albums) in artist_album_track_vecs.into_iter() {
        println!("let's try {}", orig_artist_name);
        let maybe_artist = match model::get_lowercase_artist(&raw_library, &orig_artist_name) {
            Some(artist) => Some(artist),
            None => match artist_mapping.get(&orig_artist_name) {
                Some(artist_info) => Some(raw_library.artists.get(&artist_info.artist_id).unwrap()),
                None => None,
            },
        };

        let artist_name = match artist_mapping.get(&orig_artist_name) {
            Some(artist_info) => artist_info.artist_name.to_lowercase(),
            None => orig_artist_name.clone(),
        };

        for (album_name, tracks) in albums.into_iter() {
            if existing_manual_mapping.contains_key(&(artist_name.clone(), album_name.clone())) {
                println!(
                    "already found this album: {} in existing manual mapping; skipping",
                    album_name,
                )
            } else {
                if existing_ignore_albums.contains(&(artist_name.clone(), album_name.clone())) {
                    println!(
                        "skipping this album {} (by {}) as it was found in the album ignore list",
                        album_name, artist_name
                    );
                } else {
                    let letstry = match maybe_artist {
                        Some(artist) => {
                            if model::exists_lowercase_album(&artist, &album_name) {
                                println!(
                                    "skipping album {} (by {}) as it already maps without intervention",
                                    album_name, artist_name,
                                );
                                false
                            } else {
                                println!(
                                    "could not find album {} (by {}) in library",
                                    album_name, artist_name
                                );
                                true
                            }
                        }
                        None => {
                            println!("could not find artist {} in library", artist_name);
                            true
                        }
                    };
                    if letstry {
                        println!(
                            "this album: {} (by {}) has been listened to this many times: {}",
                            album_name,
                            artist_name,
                            gmodel::compute_album_play_count(&tracks),
                        );
                        let maybe_matched_artist = prompt_for_maybe_artist(&raw_library);
                        match maybe_matched_artist {
                            Ok(album_artist) => {
                                artist_album_map.insert(
                                    (artist_name.clone(), album_name.clone()),
                                    album_artist,
                                );
                                ()
                            }
                            Err(e) => match e {
                                Error::Skip => continue,
                                Error::Ignore => {
                                    ignore_albums.insert((artist_name.clone(), album_name.clone()));
                                    ()
                                }
                                Error::Exit => {
                                    return compute_final_artist_album_map(
                                        artist_album_map,
                                        existing_manual_mapping,
                                        ignore_albums,
                                        existing_ignore_albums,
                                    )
                                }
                            },
                        };
                    }
                }
            }
        }
    }

    compute_final_artist_album_map(
        artist_album_map,
        existing_manual_mapping,
        ignore_albums,
        existing_ignore_albums,
    )
}

fn compute_final_artist_album_map(
    mut artist_album_map: BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    mut existing_manual_mapping: BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    mut ignore_albums: BTreeSet<(String, String)>,
    mut existing_ignore_albums: BTreeSet<(String, String)>,
) -> (
    BTreeMap<(String, String), musiqlibrary::ArtistAlbumInfo>,
    BTreeSet<(String, String)>,
) {
    artist_album_map.append(&mut existing_manual_mapping);

    ignore_albums.append(&mut existing_ignore_albums);

    (artist_album_map, ignore_albums)
}

pub enum Error {
    Skip,
    Ignore,
    Exit,
}

fn prompt_for_maybe_artist(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<musiqlibrary::ArtistAlbumInfo, Error> {
    println!("type search query for an album");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return Err(Error::Exit);
    }
    if input == "ignore" || input == "i" {
        return Err(Error::Ignore);
    }
    if input == "skip" || input == "s" {
        return Err(Error::Skip);
    }

    let mut artists = Vec::new();
    for artist in raw_library.artists.values() {
        for album in artist.albums.values() {
            if album
                .album_info
                .album_name
                .to_lowercase()
                .contains(&input.to_lowercase())
            {
                artists.push(musiqlibrary::ArtistAlbumInfo {
                    artist: artist.artist_info.clone(),
                    album: album.album_info.clone(),
                });
            }
        }
    }

    if artists.len() > 30 {
        println!("try a more narrow search query");
        match exact_match_album(&raw_library, input.to_string()) {
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

    select_album(artists)
}

fn exact_match_album(
    raw_library: &musiqlibrary::RawLibrary,
    input: String,
) -> Option<Vec<musiqlibrary::ArtistAlbumInfo>> {
    let mut artists = Vec::new();
    for artist in raw_library.artists.values() {
        for album in artist.albums.values() {
            if album.album_info.album_name.to_lowercase() == input.to_lowercase() {
                artists.push(musiqlibrary::ArtistAlbumInfo {
                    album: album.album_info.clone(),
                    artist: artist.artist_info.clone(),
                });
            }
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

fn select_album(
    artist_album_vec: Vec<musiqlibrary::ArtistAlbumInfo>,
) -> Result<musiqlibrary::ArtistAlbumInfo, Error> {
    for (i, track) in artist_album_vec.iter().enumerate() {
        println!(
            "{}:	{} (by {})",
            i, track.album.album_name, track.artist.artist_name
        );
    }

    println!("select a artist from above (0-29)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return Err(Error::Exit);
    }
    if input == "ignore" || input == "i" {
        return Err(Error::Ignore);
    }
    if input == "skip" || input == "s" {
        return Err(Error::Skip);
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < artist_album_vec.len() {
                Ok(artist_album_vec[i].clone())
            } else {
                println!("number must be within range of artist_album_vec found");
                select_album(artist_album_vec)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_album(artist_album_vec)
        }
    }
}
