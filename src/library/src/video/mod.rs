use std::{fs, path, str};

use chrono::NaiveDate;
use mp4;
use serde::Deserialize;

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
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtraMetadata {
    pub release: NaiveDate,
    pub genres: Vec<String>,
    pub cast: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
}

/// Recursively find all movies with metadata
pub fn find_movies_in_dir<P: AsRef<path::Path>>(movie_path: P) -> Vec<MovieMetadata> {
    let all_paths = find_movie_paths(movie_path.as_ref().to_path_buf());
    let all_movie_metadata: Vec<_> = all_paths
        .iter()
        .map(|x| find_movie_metadata(&movie_path.as_ref().to_path_buf(), x))
        .filter_map(|x| {
            match x {
                Ok(_) => (),
                Err(ref e) => println!("error: {:?}", e),
            };
            x.ok()
        })
        .collect();

    all_movie_metadata
}

/// Recursively find the paths for all movie files in a directory
pub fn find_movie_paths(current_path: path::PathBuf) -> Vec<path::PathBuf> {
    if current_path.is_file() {
        let fileext = current_path.extension().unwrap().to_str().unwrap();

        return match fileext {
            "m4v" | "mp4" => vec![current_path.clone()],
            _ => {
                println!("unexpected file extension: {}", fileext);
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
    println!(
        "unexpected file type: {}",
        current_path.into_os_string().to_string_lossy()
    );
    return Vec::new();
}

/// Extract the movie metadata from a given movie file path
pub fn find_movie_metadata(
    orig_scan_path: &path::PathBuf,
    movie_path: &path::PathBuf,
) -> Result<MovieMetadata, Error> {
    println!(
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

    let title = match udta.meta.unwrap() {
        mp4::MetaBox::Mdir { ilst } => {
            let ilst = ilst.unwrap();
            //println!("ilst {:?}", ilst.items);
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

    Ok(MovieMetadata {
        title: title,
        path: movie_path.clone(),
        relative_path: movie_path
            .to_path_buf()
            .strip_prefix(orig_scan_path)
            .unwrap()
            .to_path_buf(),
    })
}
