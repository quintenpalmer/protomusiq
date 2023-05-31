use std::collections::BTreeMap;
use std::io;

use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone};
use serde_json;

use musiqlibrary;

use crate::datastore::jsonbacked::{self, playlists as userplaylists, tracker};
use crate::datastore::loader;
use crate::datastore::traits::LiveHistoryWriteDS;

use crate::model;
use crate::util::config;

#[derive(Debug)]
pub enum TrackerError {
    CouldNotTrack,
}

pub fn report_tracks() -> Result<(), TrackerError> {
    match report_tracks_with_io() {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("reporting to tracker resulted in {:?}", e);
            Err(TrackerError::CouldNotTrack)
        }
    }
}

fn report_tracks_with_io() -> io::Result<()> {
    println!("Starting Interactive Reporting");

    println!("which host is this?");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    let hostname = input;

    let config_state = config::get_default_config();

    let mut tracker = tracker::JSONTracker::new(&config_state.app_data_path, hostname.clone());

    let raw_library = jsonbacked::tracklibrary::load_library_from_cache_and_scan(
        &config_state,
        &loader::Loader::NoCache,
    );

    let mut running = true;
    while running {
        let start_date_time = match prompt_for_date() {
            Ok(d) => d,
            Err(_e) => {
                running = false;
                continue;
            }
        };

        let tracks = match prompt_for_tracks(&config_state, &raw_library) {
            Ok(t) => t,
            Err(_e) => {
                running = false;
                continue;
            }
        };

        match prompt_for_confirmation_and_commit(&mut tracker, start_date_time, tracks) {
            Ok(()) => (),
            Err(_e) => {
                running = false;
                continue;
            }
        };
    }
    Ok(())
}

fn prompt_for_confirmation_and_commit(
    tracker: &mut tracker::JSONTracker,
    start_date_time: DateTime<Local>,
    tracks: Vec<musiqlibrary::FullTrackMetadata>,
) -> Result<(), ()> {
    let tuples = compute_track_times(start_date_time, tracks);
    println!("confirm these tracks at these times");
    for (track, date_time) in tuples.iter() {
        println!(
            "{:?}:\t{} ({})",
            date_time,
            track.title,
            serde_json::to_string(&musiqlibrary::TrackUniqueIdentifier::from_track(&track))
                .unwrap()
        );
    }

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    tracker.increment_tracks_with_dates(tuples);
    Ok(())
}

fn compute_track_times(
    start_date: DateTime<Local>,
    tracks: Vec<musiqlibrary::FullTrackMetadata>,
) -> Vec<(musiqlibrary::FullTrackMetadata, DateTime<Local>)> {
    let mut track_times = Vec::new();
    for track in tracks.into_iter() {
        match track_times.last() {
            None => track_times.push((track, start_date)),
            Some((old_track, old_start_date)) => {
                let new_start_date =
                    *old_start_date + Duration::seconds(old_track.duration.as_secs() as i64);
                track_times.push((track, new_start_date))
            }
        }
    }
    track_times
}

fn prompt_for_date() -> Result<DateTime<Local>, ()> {
    println!("enter date in:\tYYYY-MM-DD HH:MM:SS");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    let naive_date_time = NaiveDateTime::parse_from_str(input.as_str(), "%F %T").unwrap();

    let local_date_time: DateTime<Local> = Local.from_local_datetime(&naive_date_time).unwrap();

    println!("working with this date:\t{:?}", local_date_time);

    Ok(local_date_time)
}

fn prompt_for_tracks(
    config_state: &model::AppConfigState,
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    println!("are you adding an `album`, `track`, or `playlist` (enter one)");
    println!("or if you are done adding media for this session, enter `done`");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    if input == "done" {
        return Ok(Vec::new());
    }

    let mut current_collection = match input.as_str() {
        "album" => prompt_for_album(&raw_library),
        "track" => prompt_for_track(&raw_library).map(|x| vec![x]),
        "disc" => prompt_for_disc(&raw_library),
        "playlist" => {
            let playlist_data =
                userplaylists::PlaylistData::new(&config_state.app_data_path.to_path_buf());

            prompt_for_playlist(&raw_library, &playlist_data)
        }
        _ => prompt_for_tracks(&config_state, &raw_library),
    }?;

    let mut more = prompt_for_tracks(&config_state, &raw_library)?;

    current_collection.append(&mut more);

    Ok(current_collection)
}

fn prompt_for_album(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    let only_albums = prompt_for_only_albums(&raw_library)?;

    select_album(only_albums)
}

fn prompt_for_only_albums(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<
    Vec<(
        musiqlibrary::ArtistInfo,
        musiqlibrary::KeyedAlbumTracks<musiqlibrary::FullTrackMetadata>,
    )>,
    (),
> {
    println!("type search query for an album");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    let mut albums = Vec::new();
    let mut exact = Vec::new();
    for artist in raw_library.artists.values() {
        for album in artist.albums.values() {
            if album.album_info.album_name.to_lowercase() == input.to_lowercase() {
                exact.push((artist.artist_info.clone(), album.clone()));
            }
            if album
                .album_info
                .album_name
                .to_lowercase()
                .contains(&input.to_lowercase())
            {
                albums.push((artist.artist_info.clone(), album.clone()));
            }
        }
    }

    if albums.len() > 9 {
        if exact.len() > 0 {
            println!("found a lot, here are the exact matches");
            return Ok(exact);
        }
        println!("try a more narrow search query");
        return prompt_for_only_albums(&raw_library);
    }
    if albums.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_for_only_albums(&raw_library);
    }

    Ok(albums)
}

fn select_album(
    albums: Vec<(
        musiqlibrary::ArtistInfo,
        musiqlibrary::KeyedAlbumTracks<musiqlibrary::FullTrackMetadata>,
    )>,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    for (i, (artist, album)) in albums.iter().enumerate() {
        println!(
            "{}:\t{}\t({})",
            i, album.album_info.album_name, artist.artist_name
        );
    }

    println!("select an album from above (0-9)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < albums.len() {
                let mut tracks = Vec::new();
                for disc in albums[i].1.discs.values() {
                    for track in disc.tracks.values() {
                        tracks.push(track.clone());
                    }
                }
                Ok(tracks)
            } else {
                println!("number must be within range of tracks found");
                select_album(albums)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_album(albums)
        }
    }
}

fn prompt_for_disc(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    let only_albums = prompt_for_only_albums(&raw_library)?;

    select_disc(only_albums)
}

fn select_disc(
    albums: Vec<(
        musiqlibrary::ArtistInfo,
        musiqlibrary::KeyedAlbumTracks<musiqlibrary::FullTrackMetadata>,
    )>,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    for (i, (artist, album)) in albums.iter().enumerate() {
        println!(
            "{}:\t{}\t({})",
            i, album.album_info.album_name, artist.artist_name
        );
    }

    println!("select an album from above (0-9)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < albums.len() {
                select_disc_in_album(&albums[i].1.discs)
            } else {
                println!("number must be within range of tracks found");
                select_disc(albums)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_disc(albums)
        }
    }
}

fn select_disc_in_album(
    discs: &BTreeMap<u64, musiqlibrary::DiscTracks<musiqlibrary::FullTrackMetadata>>,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    for (i, disc) in discs.values().enumerate() {
        println!("{}:\t{}", i, disc.disc_no);
    }

    println!("select an album from above (0-9)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < discs.len() {
                let mut tracks = Vec::new();
                for disc in discs.values() {
                    for track in disc.tracks.values() {
                        tracks.push(track.clone());
                    }
                }
                Ok(tracks)
            } else {
                println!("number must be within range of tracks found");
                select_disc_in_album(discs)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_disc_in_album(discs)
        }
    }
}

fn prompt_for_track(
    raw_library: &musiqlibrary::RawLibrary,
) -> Result<musiqlibrary::FullTrackMetadata, ()> {
    println!("type search query for a track");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
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

    if tracks.len() > 9 {
        println!("try a more narrow search query");
        return prompt_for_track(&raw_library);
    }
    if tracks.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_for_track(&raw_library);
    }

    select_track(tracks)
}

fn select_track(
    tracks: Vec<musiqlibrary::FullTrackMetadata>,
) -> Result<musiqlibrary::FullTrackMetadata, ()> {
    for (i, track) in tracks.iter().enumerate() {
        println!(
            "{}:	{} ({} - {})",
            i, track.title, track.album_artist, track.album
        );
    }

    println!("select a track from above (0-9)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
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

fn prompt_for_playlist(
    raw_library: &musiqlibrary::RawLibrary,
    playlist_data: &userplaylists::PlaylistData,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    println!("type search query for a playlist");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    let mut playlists = Vec::new();
    let mut exact = Vec::new();
    for p in playlist_data.entries_as_vec().into_iter() {
        if p.name.to_lowercase() == input.to_lowercase() {
            exact.push(p.clone());
        }
        if p.name.to_lowercase().contains(&input.to_lowercase()) {
            playlists.push(p);
        }
    }

    if playlists.len() > 9 {
        if exact.len() > 0 {
            println!("found a lot, here are the exact matches");
            return select_playlist(&raw_library, exact);
        }
        println!("try a more narrow search query");
        return prompt_for_playlist(&raw_library, &playlist_data);
    }
    if playlists.len() <= 0 {
        println!("couldn't find anything with that search query; try again");
        return prompt_for_playlist(&raw_library, &playlist_data);
    }

    select_playlist(&raw_library, playlists)
}

fn select_playlist(
    raw_library: &musiqlibrary::RawLibrary,
    playlists: Vec<model::playlist::PlaylistEntry>,
) -> Result<Vec<musiqlibrary::FullTrackMetadata>, ()> {
    for (i, playlist) in playlists.iter().enumerate() {
        println!("{}:	{}", i, playlist.name);
    }

    println!("select a playlist from above (0-9)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "abort" {
        return Err(());
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < playlists.len() {
                Ok(playlist_full_track_metadata(&raw_library, &playlists[i]))
            } else {
                println!("number must be within range of playlists found");
                select_playlist(&raw_library, playlists)
            }
        }
        Err(_) => {
            println!("must input a number");
            select_playlist(&raw_library, playlists)
        }
    }
}

fn playlist_full_track_metadata(
    raw_library: &musiqlibrary::RawLibrary,
    playlist: &model::playlist::PlaylistEntry,
) -> Vec<musiqlibrary::FullTrackMetadata> {
    playlist
        .tracks
        .iter()
        .map(|key| raw_library.get_track(&key).clone())
        .collect()
}
