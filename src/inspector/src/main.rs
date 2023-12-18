use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use musiqlibrary as library;

use musiqlibrary::claxon;

fn help_text(cmds: &Vec<String>, extra: String) {
    println!("Must supply <library-path> <cmds...>");
    println!("available commands are:");
    for cmd in cmds.into_iter() {
        println!("  {}", cmd);
    }
    println!("{}", extra);
    panic!("exiting with non-zero status");
}

fn main() {
    let available_commands: BTreeMap<&'static str, Box<dyn AppCmd>> = {
        let inner: Vec<(&'static str, Box<dyn AppCmd>)> = vec![
            ("conflicts", Box::new(ConflictLister {})),
            ("tracks", Box::new(TrackLister {})),
            ("covers", Box::new(AlbumCoverChecker {})),
            ("json", Box::new(JsonProducer {})),
            ("dates", Box::new(DateDisplayer {})),
            ("length", Box::new(LengthCalcer {})),
            ("length-check", Box::new(LengthChecker {})),
            ("tree", Box::new(TreeViewer {})),
            ("yearendreport", Box::new(YearEndReporter {})),
            ("flac-tags", Box::new(FlacTagCollector {})),
        ];
        inner
    }
    .into_iter()
    .collect();

    let printable_cmds = available_commands.keys().map(|x| x.to_string()).collect();

    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        help_text(&printable_cmds, "".to_string())
    }

    let lib_path = args[1].clone();
    let parsed_cmds = args.split_off(2);

    println!("lib path: {}", lib_path);

    for cmd in parsed_cmds.into_iter() {
        match available_commands.get(cmd.to_string().as_str()) {
            Some(f) => f.operate(Path::new(lib_path.as_str()).to_path_buf()),
            _ => help_text(&printable_cmds, format!("unknown command: {}", cmd.clone())),
        }
    }
}

pub trait AppCmd {
    fn operate(&self, path: PathBuf);
}

pub struct ConflictLister {}

impl AppCmd for ConflictLister {
    fn operate(&self, path: PathBuf) {
        println!("finding conflicts by artist/album/disc/track...");
        let tracks = library::find_files(&path, &path).unwrap();
        let (_library, conflicts) = library::organize_tracks(tracks);
        for conflict in conflicts.into_iter() {
            println!("\t{}", conflict.path.to_str().unwrap());
        }
        println!("...done reporting said conflicts");
    }
}

pub struct TrackLister {}

impl AppCmd for TrackLister {
    fn operate(&self, path: PathBuf) {
        let tracks = library::find_files(&path, &path).unwrap();
        println!("found these ({}) tracks...", tracks.len());
        for track in tracks.iter() {
            println!(
                "\t{}\t-\t{}\t({:?})\t-\t{}",
                track.title, track.track, track.raw_track, track.disc
            );
        }
        println!("...done listing tracks");
    }
}

pub struct AlbumCoverChecker {}

impl AppCmd for AlbumCoverChecker {
    fn operate(&self, path: PathBuf) {
        println!("finding albums without cover art...");
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        for artist in library.artists.values() {
            for album in artist.albums.values() {
                match fs::metadata(album.album_info.path.join("cover.jpg")) {
                    Ok(_m) => (),
                    Err(_e) => println!("\t{}", album.album_info.path.to_string_lossy()),
                }
            }
        }
        println!("...done reporting said albums");
    }
}

pub struct JsonProducer {}

impl AppCmd for JsonProducer {
    fn operate(&self, path: PathBuf) {
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        println!("{}", serde_json::to_string_pretty(&library).unwrap());
    }
}

pub struct LengthCalcer {}

impl AppCmd for LengthCalcer {
    fn operate(&self, path: PathBuf) {
        let maybe_duration = match path
            .extension()
            .map(|a| a.to_str().map(|x| x.to_lowercase()))
        {
            Some(inner_opt) => match inner_opt {
                Some(ext) => match ext.as_str() {
                    "flac" => {
                        println!("FLAC:");
                        let reader = library::claxon::FlacReader::open(path).unwrap();
                        let stream_info = reader.streaminfo();

                        let num_samples = stream_info.samples.unwrap_or(0);
                        let sample_rate = stream_info.sample_rate;

                        let total_duration = time::Duration::from_secs_f32(
                            (num_samples as f32) / (sample_rate as f32),
                        );
                        Some(total_duration)
                    }
                    "mp3" => {
                        println!("MP3:");
                        let total_duration = library::mp3_duration::from_path(path).unwrap();
                        Some(total_duration)
                    }
                    "m4a" => {
                        println!("M4A/MP4A:");
                        let tag = library::mp4ameta::Tag::read_from_path(path.clone()).unwrap();

                        let total_duration = tag.duration().unwrap_or(time::Duration::ZERO);
                        Some(total_duration)
                    }
                    _ => {
                        println!("could not resolve file extension");
                        None
                    }
                },
                None => {
                    println!("could not resolve file extension");
                    None
                }
            },
            None => {
                println!("could not resolve file extension even higher");
                None
            }
        };
        match maybe_duration {
            Some(duration) => println!("duration: {:?}", duration),
            None => println!("could not parse duration"),
        };
    }
}

pub struct LengthChecker {}

impl AppCmd for LengthChecker {
    fn operate(&self, path: PathBuf) {
        let tracks = library::find_files(&path, &path).unwrap();
        for track in tracks.into_iter() {
            if track.duration == time::Duration::ZERO {
                println!("track: {} has no duration", track.title);
            }
        }
    }
}

pub struct DateDisplayer {}

impl AppCmd for DateDisplayer {
    fn operate(&self, path: PathBuf) {
        let tracks = library::find_files(&path, &path).unwrap();
        for track in tracks.into_iter() {
            println!(
                "{} date for album {}'s track: {}",
                track.raw_date, track.album, track.title
            );
        }
    }
}

pub struct TreeViewer {}

impl AppCmd for TreeViewer {
    fn operate(&self, path: PathBuf) {
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        println!("Library:");
        let artist_count = library.artists.keys().len() - 1;
        for (current_artist_index, artist) in library.artists.values().enumerate() {
            println!(
                "{} Artist: '{}' ({})",
                if current_artist_index == artist_count {
                    "└───"
                } else {
                    "├───"
                },
                artist.artist_info.artist_name,
                artist.artist_info.artist_id.hashed()
            );
            let album_count = artist.albums.keys().len() - 1;
            for (current_album_index, album) in artist.albums.values().enumerate() {
                println!(
                    "{}{} Album: '{}' ({})",
                    if current_artist_index == artist_count {
                        "    "
                    } else {
                        "│   "
                    },
                    if current_album_index == album_count {
                        " └──"
                    } else {
                        " ├──"
                    },
                    album.album_info.album_name,
                    album.album_info.album_id.hashed()
                );
                let disc_count = album.discs.keys().len() - 1;
                for (current_disc_index, disc) in album.discs.values().enumerate() {
                    println!(
                        "{}{}{} Disc: '{}' (of '{}')",
                        if current_artist_index == artist_count {
                            "    "
                        } else {
                            "│   "
                        },
                        if current_album_index == album_count {
                            "    "
                        } else {
                            " │  "
                        },
                        if current_disc_index == disc_count {
                            " └──"
                        } else {
                            " ├──"
                        },
                        disc.disc_no,
                        disc.tracks
                            .values()
                            .next()
                            .unwrap()
                            .disc_total
                            .map(|x| x.to_string())
                            .unwrap_or("<none seen>".to_string())
                    );
                    let track_count = disc.tracks.keys().len() - 1;
                    for (current_track_index, track) in disc.tracks.values().enumerate() {
                        println!(
                            "{}{}{}{} {}\t: '{}'",
                            if current_artist_index == artist_count {
                                "    "
                            } else {
                                "│   "
                            },
                            if current_album_index == album_count {
                                "    "
                            } else {
                                " │  "
                            },
                            if current_disc_index == disc_count {
                                "    "
                            } else {
                                " │  "
                            },
                            if current_track_index == track_count {
                                " └──"
                            } else {
                                " ├──"
                            },
                            track.track,
                            track.title
                        );
                    }
                }
            }
        }
    }
}

pub struct YearEndReporter {}

impl AppCmd for YearEndReporter {
    fn operate(&self, _path: PathBuf) {}
}

pub struct FlacTagCollector {}

impl AppCmd for FlacTagCollector {
    fn operate(&self, path: PathBuf) {
        let mut tags = BTreeMap::new();
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        println!("Library:");
        for artist in library.artists.values() {
            for album in artist.albums.values() {
                for disc in album.discs.values() {
                    for track in disc.tracks.values() {
                        if track.ext == "flac".to_string() {
                            for (key, _value) in
                                claxon::FlacReader::open(track.path.clone()).unwrap().tags()
                            {
                                let entry = tags.entry(key.to_string().to_lowercase()).or_insert(0);
                                *entry += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("Found these tags:");
        println!("-----------------");
        for (key, value) in tags.iter() {
            println!("{} ({})", key, value);
        }
        println!("-----------------");
    }
}
