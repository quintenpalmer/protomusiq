use std::fs;
use std::io;

use chrono::TimeZone;

use super::smodel;

const RAW_SPOTIFY_JSON: &'static str = "spotify/input/endsong_.json";
const CLEAN_SPOTIFY_JSON: &'static str = "spotify/input/play_history.json";

pub fn maybe_clean_and_convert_json() {
    match fs::File::open(CLEAN_SPOTIFY_JSON) {
        Ok(_) => println!("found cleaned json file, using that"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => clean_and_convert_json(),
            _ => panic!("error checking for cleaned json file"),
        },
    };
}

pub fn clean_and_convert_json() {
    let in_all_tracks_file = fs::File::open(RAW_SPOTIFY_JSON).unwrap();

    let raw_line_items: Vec<smodel::RawLineItem> =
        serde_json::from_reader(in_all_tracks_file).unwrap();

    let cleaned_line_items: Vec<smodel::CleanedLineItem> = raw_line_items
        .into_iter()
        .map(|track| {
            let naive_date_time =
                chrono::NaiveDateTime::parse_from_str(track.ts.as_str(), "%+").unwrap();

            let local_date_time: chrono::DateTime<chrono::Local> =
                chrono::Local.from_local_datetime(&naive_date_time).unwrap();

            smodel::CleanedLineItem {
                master_metadata_track_name: track
                    .master_metadata_track_name
                    .unwrap_or("UNKNOWN".to_string())
                    .to_lowercase(),
                master_metadata_album_artist_name: track
                    .master_metadata_album_artist_name
                    .unwrap_or("UNKNOWN".to_string())
                    .to_lowercase(),
                master_metadata_album_album_name: track
                    .master_metadata_album_album_name
                    .unwrap_or("UNKNOWN".to_string())
                    .to_lowercase(),
                ms_played: track.ms_played,
                ts: local_date_time,
            }
        })
        .collect();

    let out_json_file = fs::File::create(CLEAN_SPOTIFY_JSON).unwrap();
    serde_json::to_writer_pretty(out_json_file, &cleaned_line_items).unwrap();
}
