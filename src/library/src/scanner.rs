use std::fs::{self};
use std::io;
use std::path::Path;
use std::sync::mpsc;

use crate::model::{FullTrackMetadata, TrackPathInfo};

use crate::parser;

pub fn find_only_files<P: AsRef<Path>>(path: &P) -> io::Result<Vec<TrackPathInfo>> {
    find_only_files_helper(&path, &path)
}

fn find_only_files_helper<O: AsRef<Path>, P: AsRef<Path>>(
    orig_prefix: &O,
    path: &P,
) -> io::Result<Vec<TrackPathInfo>> {
    let mut path_info_list = Vec::new();

    for path in fs::read_dir(path)? {
        let path = path?;
        if path.file_type()?.is_dir() {
            path_info_list.append(&mut find_only_files_helper(orig_prefix, &path.path())?);
        }
        if path.file_type()?.is_file() {
            let maybe_track_info: Option<TrackPathInfo> = match path
                .path()
                .extension()
                .map(|a| a.to_str().map(|x| x.to_lowercase()))
            {
                Some(inner_opt) => {
                    match inner_opt {
                        Some(ext) => match ext.as_str() {
                            "flac" | "mp3" | "m4a" => Some(TrackPathInfo {
                                path: path.path().to_path_buf(),
                                relative_path: path
                                    .path()
                                    .to_path_buf()
                                    .strip_prefix(orig_prefix)
                                    .unwrap()
                                    .to_path_buf(),
                                last_modified: fs::metadata(path.path().clone())
                                    .map_err(|e| format!("{:?}", e))
                                    .unwrap()
                                    .modified()
                                    .map_err(|e| format!("{:?}", e))
                                    .unwrap(),
                            }),
                            // these files are common to see, so we don't log if we see them
                            // consider extracting this out to an 'silent-ignore-suffix' list
                            "png" => None,
                            "txt" => None,
                            "rtf" => None,
                            "jpg" => None,
                            "gif" => None,
                            "pdf" => None,
                            "webp" => None,
                            unexpected_ext => {
                                eprintln!(
                                    "no music metadata parsed for extension {}\t(path: {})",
                                    unexpected_ext,
                                    path.path().display()
                                );
                                None
                            }
                        },
                        None => {
                            eprintln!("could not resolve file extension, let me know if you ever see this");
                            None
                        }
                    }
                }
                None => {
                    eprintln!(
                        "no music metadata parsed file with no extension\t(path: {})",
                        path.path().display()
                    );
                    None
                }
            };

            match maybe_track_info {
                Some(track_info) => {
                    path_info_list.push(track_info);
                }
                None => (),
            };
        }
    }
    Ok(path_info_list)
}

pub fn find_files<O: AsRef<Path>>(orig_prefix: &O) -> io::Result<Vec<FullTrackMetadata>> {
    let files = find_only_files(orig_prefix)?;

    let mut metadata_map = Vec::new();

    let num_threads = std::thread::available_parallelism()
        .map(|x| x.into())
        .unwrap_or(1);

    eprintln!("creating thread pool with {} threads", num_threads);

    let tpool = threadpool::ThreadPool::new(num_threads);
    let (tx, rx) = mpsc::channel();

    for path in files.into_iter() {
        let tx = tx.clone();

        let orig_prefix = orig_prefix.as_ref().to_path_buf().clone();

        tpool.execute(move || {
            let maybe_parser: Option<Box<dyn parser::MetadataParser>> = match path
                .path
                .extension()
                .map(|a| a.to_str().map(|x| x.to_lowercase()))
            {
                Some(inner_opt) => {
                    match inner_opt {
                        Some(ext) => match ext.as_str() {
                            "flac" => Some(Box::new(
                                parser::FlacMetadataParser::new(path.path).unwrap(),
                            )),
                            "mp3" => {
                                Some(Box::new(parser::ID3MetadataParser::new(path.path).unwrap()))
                            }
                            "m4a" => Some(Box::new(
                                parser::MP4AMetadataParser::new(path.path).unwrap(),
                            )),
                            // these files are common to see, so we don't log if we see them
                            // consider extracting this out to an 'silent-ignore-suffix' list
                            "png" => None,
                            "txt" => None,
                            "rtf" => None,
                            "jpg" => None,
                            "gif" => None,
                            "pdf" => None,
                            "webp" => None,
                            unexpected_ext => {
                                eprintln!(
                                    "no music metadata parsed for extension {}\t(path: {})",
                                    unexpected_ext,
                                    path.path.display()
                                );
                                None
                            }
                        },
                        None => {
                            eprintln!(
                            "could not resolve file extension, let me know if you ever see this"
                        );
                            None
                        }
                    }
                }
                None => {
                    eprintln!(
                        "no music metadata parsed file with no extension\t(path: {})",
                        path.path.display()
                    );
                    None
                }
            };

            match maybe_parser {
                Some(parser) => {
                    let track_info = parser::resolve_metadata_from_parser(&orig_prefix, parser);
                    tx.send(track_info)
                        .expect("I hope the music metadata scanner rx is receiving");
                }
                None => (),
            };
        });
    }

    drop(tx);

    for music_metadata in rx {
        metadata_map.push(music_metadata);
    }

    Ok(metadata_map)
}
