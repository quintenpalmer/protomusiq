use std::path::{Path, PathBuf};
use std::time;

use crate::model::{FullTrackMetadata, ID};

pub fn trimmer(s: String) -> String {
    s.trim_end_matches('\0').to_string()
}

pub trait MetadataParser {
    fn album(&self) -> Option<String>;
    fn album_artist(&self) -> Option<String>;
    fn artist(&self) -> String;
    fn disc(&self) -> Option<u64>;
    fn disc_total(&self) -> Option<u64>;
    fn track(&self) -> Option<u64>;
    fn title(&self) -> String;
    fn genre(&self) -> Option<String>;
    fn date(&self) -> Option<String>;
    fn duration(&self) -> time::Duration;
    fn path(&self) -> PathBuf;
    fn last_mod(&self) -> time::SystemTime;
    fn ext(&self) -> String;
}

pub fn resolve_metadata_from_parser<P: AsRef<Path>>(
    orig_prefix: &P,
    parser: Box<dyn MetadataParser>,
) -> FullTrackMetadata {
    let album = parser.album().unwrap_or_else(|| parser.title());
    let album_id = ID::new(&album);
    let album_artist = parser.album_artist().unwrap_or_else(|| parser.artist());
    let album_artist_id = ID::new(&album_artist);
    let date = parser
        .date()
        .expect(format!("album: {}, artist: {}", album, album_artist).as_str());

    FullTrackMetadata {
        album,
        raw_album: parser.album(),
        album_id,
        album_artist,
        album_artist_id,
        track_artist: parser.artist(),
        track_artist_id: ID::new(&parser.artist()),
        disc: parser.disc().unwrap_or(1),
        raw_disc: parser.disc(),
        disc_total: parser.disc_total(),
        track: parser.track().unwrap_or(1),
        raw_track: parser.track(),
        title: parser.title(),
        genre: parser.genre().unwrap_or("Unknown".to_string()),
        date_number: year_number_from_date_string(date.clone()),
        raw_date: date,
        duration: parser.duration(),
        path: parser.path(),
        relative_path: parser
            .path()
            .strip_prefix(orig_prefix)
            .unwrap()
            .to_path_buf(),
        last_modified: parser.last_mod(),
        ext: parser.ext(),
    }
}

fn year_number_from_date_string(s: String) -> u32 {
    let mut chars = s.chars();
    let thousands = chars.next().unwrap().to_digit(10).unwrap();
    let hundreds = chars.next().unwrap().to_digit(10).unwrap();
    let tens = chars.next().unwrap().to_digit(10).unwrap();
    let ones = chars.next().unwrap().to_digit(10).unwrap();

    thousands * 1000 + hundreds * 100 + tens * 10 + ones
}
