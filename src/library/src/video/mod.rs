use std::{fs, io, path, str, time};

use chrono::NaiveDate;
use mp4;
use serde::Deserialize;
use serde_json;

/// Possible Errors from Movie Searching/Decoding
#[derive(Debug)]
pub enum Error {
    NonTextTitle,
    NonMP4File,
}

/// Parsed and Normalized Movie Data
#[derive(Debug, Clone)]
pub struct MovieMetadata {
    pub title: String,
    pub path: path::PathBuf,
    pub relative_path: path::PathBuf,
    pub last_modified: time::SystemTime,
    pub duration: time::Duration,
    pub extra: Option<ExtraMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtraMetadata {
    #[serde(with = "slash_date")]
    pub release: NaiveDate,
    pub genres: Vec<String>,
    pub production: Vec<String>,
    pub cast: Vec<String>,
    pub producers: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
    pub series: Option<SeriesInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeriesInfo {
    pub index: u32,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MovieID {
    inner_id: String,
}

impl MovieMetadata {
    pub fn get_id(&self) -> MovieID {
        MovieID {
            inner_id: self.title.clone(),
        }
    }
}

/// Recursively find all movies with metadata
pub fn find_movies_in_dir<P: AsRef<path::Path>>(movie_path: P) -> Vec<MovieMetadata> {
    let all_paths = find_movie_paths(movie_path.as_ref().to_path_buf());

    let mut all_movie_metadata = Vec::new();

    for specific_movie_path in all_paths.into_iter() {
        match find_movie_metadata(&movie_path.as_ref().to_path_buf(), &specific_movie_path) {
            Ok(movie_metadata) => all_movie_metadata.push(movie_metadata),
            Err(ref e) => eprintln!("error: {:?}", e),
        }
    }

    all_movie_metadata
}

/// Recursively find the paths for all movie files in a directory
pub fn find_movie_paths(current_path: path::PathBuf) -> Vec<path::PathBuf> {
    if current_path.is_file() {
        let fileext = current_path.extension().unwrap().to_str().unwrap();

        return match fileext {
            "m4v" | "mp4" => vec![current_path.clone()],
            "jpg" | "json" => Vec::new(), // movies should have jpg box art and json metadata
            // files, but there's nothing to do with them here
            _ => {
                eprintln!("unexpected file extension: {}", fileext);
                Vec::new()
            }
        };
    }
    if current_path.is_dir() {
        let mut ret = Vec::new();
        for entry in current_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let mut children = find_movie_paths(entry.path());
            ret.append(&mut children);
        }
        return ret;
    }
    eprintln!(
        "unexpected file type: {}",
        current_path.into_os_string().to_string_lossy()
    );

    Vec::new()
}

/// Extract the movie metadata from a given movie file path
pub fn find_movie_metadata(
    orig_scan_path: &path::PathBuf,
    movie_path: &path::PathBuf,
) -> Result<MovieMetadata, Error> {
    eprintln!(
        "path: {}",
        movie_path.clone().into_os_string().to_string_lossy()
    );
    let fileext = path::Path::new(movie_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    match fileext {
        "m4v" | "mp4" => find_mp4_metadata(orig_scan_path, movie_path),
        _ => Err(Error::NonMP4File),
    }
}

/// Extract the movie metadata for a "MPEG-4 Part 14" or "MP4" file
/// https://en.wikipedia.org/wiki/MP4_file_format
/// https://en.wikipedia.org/wiki/Comparison_of_video_container_formats
fn find_mp4_metadata(
    orig_scan_path: &path::PathBuf,
    movie_path: &path::PathBuf,
) -> Result<MovieMetadata, Error> {
    let movie_file = fs::File::open(movie_path).unwrap();
    let raw_metadata = mp4::read_mp4(movie_file).unwrap();

    let udta = raw_metadata.moov.udta.unwrap();
    let mvhd = raw_metadata.moov.mvhd;

    let title = match udta.meta.unwrap() {
        mp4::MetaBox::Mdir { ilst } => {
            let ilst = ilst.unwrap();
            //eprintln!("ilst {:?}", ilst.items);
            let title_data = &ilst.items.get(&mp4::MetadataKey::Title).unwrap().data;
            if title_data.data_type == mp4::DataType::Text {
                Ok(str::from_utf8(title_data.data.as_slice())
                    .unwrap()
                    .to_string())
            } else {
                Err(Error::NonTextTitle)
            }
        }
        _ => Err(Error::NonMP4File),
    }?;

    let duration = time::Duration::from_millis(mvhd.duration);

    let last_mod = fs::metadata(movie_path).unwrap().modified().unwrap();

    let extra = find_extra_metadata(movie_path);

    Ok(MovieMetadata {
        title,
        path: movie_path.clone(),
        relative_path: movie_path
            .to_path_buf()
            .strip_prefix(orig_scan_path)
            .unwrap()
            .to_path_buf(),
        last_modified: last_mod,
        duration,
        extra,
    })
}

fn find_extra_metadata(movie_path: &path::PathBuf) -> Option<ExtraMetadata> {
    let parent_dir = movie_path.parent().unwrap();

    let metadata_json_file = parent_dir.join("metadata.json");

    let maybe_raw: Option<ExtraMetadata> = match fs::File::open(metadata_json_file.clone()) {
        Ok(reader) => match serde_json::from_reader(io::BufReader::new(reader)) {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                eprintln!(
                    "could not deserialize data from path: {:?} {:?}",
                    metadata_json_file.display(),
                    e
                );
                None
            }
        },
        Err(e) => {
            eprintln!(
                "could not load file: {:?} {:?}",
                metadata_json_file.display(),
                e
            );
            None
        }
    };

    maybe_raw
}

mod slash_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y/%m/%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(date)
    }
}
