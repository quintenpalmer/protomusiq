use crate::youtube::ytmodel::*;

use std::io;

pub fn prompt_user_for_track(
    raw_library: &musiqlibrary::RawLibrary,
    maybe_artist_name: Option<&String>,
    found_track_name: &String,
    track_watches: &Vec<String>,
) -> PromptResult<musiqlibrary::TrackUniqueIdentifier> {
    println!("-----------------");
    println!(
        " > {}\t<= track",
        found_track_name
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "   > {}\t<- artist",
        maybe_artist_name.unwrap_or(&"<unmatched>".to_string())
    );
    println!("  > {} listens", track_watches.len());
    println!("-----------------");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return PromptResult::Stop;
    }
    /*
    if input == "ignore" || input == "i" {
        return Err(Error::Ignore);
    }
    */
    if input == "skip" || input == "s" {
        return PromptResult::NothingFound;
    }

    let mut tracks = find_matches(raw_library, maybe_artist_name, &input);

    if tracks.len() > 30 {
        println!("try a more narrow search query");
        match exact_match_track(raw_library, maybe_artist_name, &input) {
            Some(exact_match_tracks) => tracks = exact_match_tracks,
            None => {
                println!(
                    "couldn't find exact match after failing to find match containing; try again"
                );
                return prompt_user_for_track(
                    raw_library,
                    maybe_artist_name,
                    found_track_name,
                    track_watches,
                );
            }
        };
    }

    if tracks.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_user_for_track(
            raw_library,
            maybe_artist_name,
            found_track_name,
            track_watches,
        );
    }

    select_track(tracks)
}

fn find_matches(
    raw_library: &musiqlibrary::RawLibrary,
    maybe_artist_name: Option<&String>,
    input_track_portion: &String,
) -> Vec<musiqlibrary::FullTrackMetadata> {
    let mut tracks = Vec::new();

    match maybe_artist_name {
        Some(artist_name) => {
            let artist = raw_library
                .artists
                .get(&musiqlibrary::ID::new(artist_name))
                .unwrap();
            for album in artist.albums.values() {
                for disc in album.discs.values() {
                    for track in disc.tracks.values() {
                        if track
                            .title
                            .to_lowercase()
                            .contains(&input_track_portion.to_lowercase())
                        {
                            tracks.push(track.clone());
                        }
                    }
                }
            }
        }
        None => {
            for artist in raw_library.artists.values() {
                for album in artist.albums.values() {
                    for disc in album.discs.values() {
                        for track in disc.tracks.values() {
                            if track
                                .title
                                .to_lowercase()
                                .contains(&input_track_portion.to_lowercase())
                            {
                                tracks.push(track.clone());
                            }
                        }
                    }
                }
            }
        }
    };

    tracks
}

fn exact_match_track(
    raw_library: &musiqlibrary::RawLibrary,
    maybe_artist_name: Option<&String>,
    input_track_portion: &String,
) -> Option<Vec<musiqlibrary::FullTrackMetadata>> {
    let mut tracks = Vec::new();

    match maybe_artist_name {
        Some(artist_name) => {
            let artist = raw_library
                .artists
                .get(&musiqlibrary::ID::new(artist_name))
                .unwrap();
            for album in artist.albums.values() {
                for disc in album.discs.values() {
                    for track in disc.tracks.values() {
                        if track.title.to_lowercase() == input_track_portion.to_lowercase() {
                            tracks.push(track.clone());
                        }
                    }
                }
            }
        }
        None => {
            for artist in raw_library.artists.values() {
                for album in artist.albums.values() {
                    for disc in album.discs.values() {
                        for track in disc.tracks.values() {
                            if track.title.to_lowercase() == input_track_portion.to_lowercase() {
                                tracks.push(track.clone());
                            }
                        }
                    }
                }
            }
        }
    };

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
    track_vec: Vec<musiqlibrary::FullTrackMetadata>,
) -> PromptResult<musiqlibrary::TrackUniqueIdentifier> {
    for (i, track) in track_vec.iter().enumerate() {
        println!("{}:	{}", i, track.title);
    }

    println!("select a artist from above (0-29)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "exit" || input == "e" {
        return PromptResult::Stop;
    }
    /*
    if input == "ignore" || input == "i" {
        return Err(Error::Ignore);
    }
    */
    if input == "skip" || input == "s" {
        return PromptResult::NothingFound;
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < track_vec.len() {
                PromptResult::Answer(track_vec[i].to_unique_id())
            } else {
                println!("number must be within range of track_vec found");
                select_track(track_vec)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_track(track_vec)
        }
    }
}
