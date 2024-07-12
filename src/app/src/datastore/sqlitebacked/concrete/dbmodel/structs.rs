use std::path;
use std::time;

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Artist {
    pub id: u32,
    pub name: String,
}

#[derive(Clone)]
pub struct Album {
    pub id: u32,
    pub name: String,
    pub date_number: u32,
    pub disc_total: u32,
    pub full_path: path::PathBuf,
    pub relative_path: path::PathBuf,
    pub artist_id: u32,
}

#[derive(Clone)]
pub struct Disc {
    pub id: u32,
    pub disc_no: u32,
    #[allow(unused)]
    pub name: Option<String>,
    pub album_id: u32,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: u32,
    pub track_name: String,
    pub track_no: u32,
    pub disc_id: u32,

    pub genre_id: Option<u32>,
    pub duration: time::Duration,
    pub full_path: path::PathBuf,
    pub relative_path: path::PathBuf,
    pub last_modified: time::SystemTime,
    pub ext: String,
}

#[derive(Debug, Clone)]
pub struct LivehistoryRecord {
    pub track_id: u32,
    pub listened_date: DateTime<Local>,
}

#[allow(unused)]
#[derive(Clone)]
pub struct PrehistoryRecord {
    pub source: String,
    pub key: DBTrackUniqueIdentifier,
    pub count: u32,
}

#[allow(unused)]
#[derive(Clone)]
pub struct DBTrackUniqueIdentifier {
    pub artist_id: u32,
    pub album_id: u32,
    pub disc_id: u32,
    pub track_id: u32,
}
