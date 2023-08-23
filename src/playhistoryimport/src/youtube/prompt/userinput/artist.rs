use crate::youtube::ytmodel::*;

use std::io;

pub fn prompt_user_for_artist_name(
    raw_library: &musiqlibrary::RawLibrary,
    found_artist_name: &String,
) -> PromptResult<String> {
    println!("let's try {}", found_artist_name);

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
                return prompt_user_for_artist_name(&raw_library, &found_artist_name);
            }
        };
    }

    if artists.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_user_for_artist_name(&raw_library, &found_artist_name);
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

fn select_artist(artist_vec: Vec<musiqlibrary::ArtistInfo>) -> PromptResult<String> {
    for (i, track) in artist_vec.iter().enumerate() {
        println!("{}:	{}", i, track.artist_name);
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
            if i < artist_vec.len() {
                PromptResult::Answer(artist_vec[i].artist_name.clone())
            } else {
                println!("number must be within range of artist_vec found");
                select_artist(artist_vec)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_artist(artist_vec)
        }
    }
}
