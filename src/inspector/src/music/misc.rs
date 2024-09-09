use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time;

use musiqlibrary as library;
use musiqlibrary::claxon;

use super::super::commands::AppCmd;

pub struct AlbumCoverChecker {}

impl AppCmd for AlbumCoverChecker {
    fn operate(&self, path: PathBuf) {
        eprintln!("finding albums without cover art...");
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        for artist in library.artists.values() {
            for album in artist.albums.values() {
                match fs::metadata(album.album_info.path.join("cover.jpg")) {
                    Ok(_m) => (),
                    Err(_e) => println!("\t{}", album.album_info.path.to_string_lossy()),
                }
            }
        }
        eprintln!("...done reporting said albums");
    }
}

pub struct DateDisplayer {}

impl AppCmd for DateDisplayer {
    fn operate(&self, path: PathBuf) {
        eprintln!("let's see the date for all of the tracks under: {:?}", path);
        let tracks = library::find_files(&path).unwrap();
        for track in tracks.into_iter() {
            println!(
                "{} date for album {}'s track: {}",
                track.raw_date, track.album, track.title
            );
        }
    }
}

pub struct FlacTagCollector {}

impl AppCmd for FlacTagCollector {
    fn operate(&self, path: PathBuf) {
        let mut tags = BTreeMap::new();
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        eprintln!(
            "Let's see the flac tags for every file in the directory: {:?}",
            path
        );
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

pub struct JsonProducer {}

impl AppCmd for JsonProducer {
    fn operate(&self, path: PathBuf) {
        eprintln!(
            "we're going to try to provide the JSON output for the provided path: {:?}",
            path
        );
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        println!("{}", serde_json::to_string_pretty(&library).unwrap());
    }
}

pub struct LengthCalcer {}

impl AppCmd for LengthCalcer {
    fn operate(&self, path: PathBuf) {
        eprintln!(
            "let's calculation the length for this specific track: {:?}",
            path
        );
        let maybe_duration = match path
            .extension()
            .map(|a| a.to_str().map(|x| x.to_lowercase()))
        {
            Some(inner_opt) => match inner_opt {
                Some(ext) => match ext.as_str() {
                    "flac" => {
                        eprintln!("FLAC:");
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
                        eprintln!("MP3:");
                        let total_duration = library::mp3_duration::from_path(path).unwrap();
                        Some(total_duration)
                    }
                    "m4a" => {
                        eprintln!("M4A/MP4A:");
                        let tag = library::mp4ameta::Tag::read_from_path(path.clone()).unwrap();

                        let total_duration = tag.duration().unwrap_or(time::Duration::ZERO);
                        Some(total_duration)
                    }
                    _ => {
                        eprintln!("could not resolve file extension");
                        None
                    }
                },
                None => {
                    eprintln!("could not resolve file extension");
                    None
                }
            },
            None => {
                eprintln!("could not resolve file extension even higher");
                None
            }
        };
        match maybe_duration {
            Some(duration) => println!("duration: {:?}", duration),
            None => eprintln!("could not parse duration"),
        };
    }
}

pub struct LengthChecker {}

impl AppCmd for LengthChecker {
    fn operate(&self, path: PathBuf) {
        eprintln!("let's see if the duration for the provided track is zero (will be no output if it's non-zero)");
        let tracks = library::find_files(&path).unwrap();
        for track in tracks.into_iter() {
            if track.duration == time::Duration::ZERO {
                println!("track: {} has no duration", track.title);
            }
        }
    }
}

pub struct TrackLister {}

impl AppCmd for TrackLister {
    fn operate(&self, path: PathBuf) {
        let tracks = library::find_files(&path).unwrap();
        eprintln!("found these ({}) tracks...", tracks.len());
        for track in tracks.iter() {
            println!(
                "\t{}\t-\t{}\t({:?})\t-\t{}",
                track.title, track.track, track.raw_track, track.disc
            );
        }
        eprintln!("...done listing tracks");
    }
}
