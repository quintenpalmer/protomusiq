use std::fs;
use std::io;

use csv;
use html_escape;

use super::gmodel::{self, ALL_TRACKS_CSV, ALL_TRACKS_JSON};

pub fn maybe_group_all_csvs_into_one() {
    match fs::File::open(ALL_TRACKS_CSV) {
        Ok(_) => println!("found grouped csv file, using that"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => group_all_csvs_into_one(),
            _ => panic!("error checking for grouped csv file"),
        },
    };
}

pub fn group_all_csvs_into_one() {
    let mut all_tracks = Vec::new();
    for entry in fs::read_dir("gplaymusic/rawcsvs/").unwrap() {
        let entry = entry.unwrap();
        println!("operating on {}", entry.path().display());
        let mut reader = csv::Reader::from_reader(fs::File::open(entry.path()).unwrap());
        for result in reader.deserialize() {
            let record: gmodel::RawLineItem = result.unwrap();
            all_tracks.push(record);
        }
    }

    let mut writer = csv::Writer::from_writer(fs::File::create(ALL_TRACKS_CSV).unwrap());
    for track in all_tracks.into_iter() {
        writer.serialize(track).unwrap();
    }
    writer.flush().unwrap();
}

pub fn maybe_clean_and_convert_to_json() {
    match fs::File::open(ALL_TRACKS_JSON) {
        Ok(_) => println!("found cleaned json file, using that"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => clean_and_convert_to_json(),
            _ => panic!("error checking for cleaned json file"),
        },
    };
}

pub fn clean_and_convert_to_json() {
    let mut json_vec = Vec::new();
    let mut reader = csv::Reader::from_reader(fs::File::open(ALL_TRACKS_CSV).unwrap());
    for result in reader.deserialize() {
        let record: gmodel::RawLineItem = result.unwrap();
        let converted_record = gmodel::CleanedLineItem {
            artist: html_decode(record.artist),
            album: html_decode(record.album),
            title: html_decode(record.title),
            play_count: record.play_count.parse().unwrap(),
        };
        json_vec.push(converted_record);
    }

    let json_file = fs::File::create(ALL_TRACKS_JSON).unwrap();
    serde_json::to_writer_pretty(json_file, &json_vec).unwrap();
}

fn html_decode(s: String) -> String {
    html_escape::decode_html_entities(s.as_str()).to_string()
}
