use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use musiqlibrary as library;
use musiqlibrary::video;

use musiqlibrary::claxon;

fn help_text(cmds: &Vec<String>, extra: String) {
    println!("Must supply <cmd> <library-path>");
    println!("available commands are:");
    for cmd in cmds.into_iter() {
        println!("  {}", cmd);
    }
    println!("{}", extra);
    panic!("exiting with non-zero status");
}

pub struct ParentCommand {
    pub sub_commands: BTreeMap<&'static str, Command>,
}

pub enum Command {
    Parent(ParentCommand),
    Specific(Box<dyn AppCmd>),
}

impl Command {
    pub fn new_parent(btree_map: BTreeMap<&'static str, Command>) -> Self {
        Command::Parent(ParentCommand {
            sub_commands: btree_map,
        })
    }

    pub fn operate_on_args(&self, mut args: Vec<String>) {
        println!("operating on: {:?}", args);
        if args.len() < 1 {
            panic!("no arguments provided, and I don't actually know how to provide a useful help text");
        }
        match self {
            Command::Specific(inner) => {
                if args.len() < 1 {
                    panic!("no arguments provided, this command currently just needs a path as an argument");
                }
                let lib_path = args[0].clone();
                inner.operate(Path::new(lib_path.as_str()).to_path_buf())
            }
            Command::Parent(parent) => {
                let printable_cmds = parent.sub_commands.keys().map(|x| x.to_string()).collect();

                if args.len() < 1 {
                    help_text(&printable_cmds, "did not provide any command".to_string())
                }

                let parsed_cmd = args.remove(0);

                println!("parsed cmd: {}", parsed_cmd);

                match parent.sub_commands.get(parsed_cmd.to_string().as_str()) {
                    Some(f) => f.operate_on_args(args),
                    None => help_text(
                        &printable_cmds,
                        format!("unknown command: {}", parsed_cmd.clone()),
                    ),
                }
            }
        }
    }
}

fn main() {
    let music_commands = vec![
        ("conflicts", Command::Specific(Box::new(ConflictLister {}))),
        ("tracks", Command::Specific(Box::new(TrackLister {}))),
        ("covers", Command::Specific(Box::new(AlbumCoverChecker {}))),
        ("json", Command::Specific(Box::new(JsonProducer {}))),
        ("dates", Command::Specific(Box::new(DateDisplayer {}))),
        ("length", Command::Specific(Box::new(LengthCalcer {}))),
        (
            "length-check",
            Command::Specific(Box::new(LengthChecker {})),
        ),
        ("tree", Command::Specific(Box::new(TreeViewer {}))),
        ("table-view", Command::Specific(Box::new(TableViewer {}))),
        (
            "yearendreport",
            Command::Specific(Box::new(YearEndReporter {})),
        ),
        (
            "flac-tags",
            Command::Specific(Box::new(FlacTagCollector {})),
        ),
        (
            "list-music-files",
            Command::Specific(Box::new(MusicFileLister {})),
        ),
    ]
    .into_iter()
    .collect();

    let movie_commands = vec![(
        "movie-tree",
        Command::Specific(Box::new(MovieTreeViewer {})),
    )]
    .into_iter()
    .collect();

    let available_commands: BTreeMap<&'static str, Command> = vec![
        ("music", Command::new_parent(music_commands)),
        ("movies", Command::new_parent(movie_commands)),
    ]
    .into_iter()
    .collect();

    let command = Command::new_parent(available_commands);

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    println!("args: {:?}", args);

    command.operate_on_args(args)
}

pub trait AppCmd {
    fn operate(&self, path: PathBuf);
}

pub struct ConflictLister {}

impl AppCmd for ConflictLister {
    fn operate(&self, path: PathBuf) {
        println!("finding conflicts by artist/album/disc/track...");
        let tracks = library::find_files(&path).unwrap();
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
        let tracks = library::find_files(&path).unwrap();
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
        let tracks = library::find_files(&path).unwrap();
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
        let tracks = library::find_files(&path).unwrap();
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
                            "{}{}{}{} {}\t: '{}'{}",
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
                            track.title,
                            if track.track_artist != track.album_artist {
                                format!("\t\t(with: {})", track.track_artist)
                            } else {
                                "".to_string()
                            }
                        );
                    }
                }
            }
        }
    }
}

pub struct TableViewer {}

impl AppCmd for TableViewer {
    fn operate(&self, path: PathBuf) {
        let bar_width = 64;
        let library = library::model::RawLibrary::new(path.clone()).unwrap();

        println!("Library Table:");
        for artist in library.artists.values() {
            for album in artist.albums.values() {
                println!("┌{}", "─".repeat(bar_width));

                println!(
                    "│ {: <40} ({})",
                    album.album_info.album_name,
                    album.album_info.album_id.hashed()
                );

                println!(
                    "│       {: <34} ({})",
                    artist.artist_info.artist_name,
                    artist.artist_info.artist_id.hashed()
                );

                println!(
                    "│       Year: {: <5} {}Duration:  {}",
                    album.album_info.start_date,
                    " ".repeat(22),
                    format_duration(album.album_info.total_duration.as_secs()),
                );

                println!("├{}", "─".repeat(bar_width));

                let disc_count = album.discs.keys().len() - 1;
                for (current_disc_index, disc) in album.discs.values().enumerate() {
                    println!(
                        "│       Disc: {: <29}(of {})",
                        disc.disc_no,
                        disc.tracks
                            .values()
                            .next()
                            .unwrap()
                            .disc_total
                            .map(|x| x.to_string())
                            .unwrap_or("\"1\"".to_string())
                    );

                    println!("├{}┬{} ", "─".repeat(6), "─".repeat(bar_width - 7));

                    for track in disc.tracks.values() {
                        println!(
                            "│ {: >4} │ {: <44}{}",
                            track.track,
                            track.title,
                            format_duration(track.duration.as_secs())
                        );
                    }

                    println!(
                        "{}{}┴{} ",
                        if current_disc_index == disc_count {
                            "└"
                        } else {
                            "├"
                        },
                        "─".repeat(6),
                        "─".repeat(bar_width - 7)
                    );
                }
            }
        }
    }
}

pub struct MovieTreeViewer {}

impl AppCmd for MovieTreeViewer {
    fn operate(&self, path: PathBuf) {
        let mut movies = video::find_movies_in_dir(path.clone());
        movies.sort_by(|a, b| a.title.cmp(&b.title));
        println!("Movie Library:");
        let movie_count = movies.len() - 1;
        for (current_movie_index, movie) in movies.iter().enumerate() {
            println!(
                "{} Movie: '{}'{}",
                if current_movie_index == movie_count {
                    "└───"
                } else {
                    "├───"
                },
                movie.title,
                match movie.extra {
                    Some(ref extra) => match extra.series {
                        Some(ref series) => {
                            format!("\t{} in {}", series.index, series.name)
                        }
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                }
            );
            println!(
                "{}    └─── {}",
                if current_movie_index < movie_count {
                    "│"
                } else {
                    " "
                },
                movie.path.clone().into_os_string().to_string_lossy()
            );
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

pub struct MusicFileLister {}

impl AppCmd for MusicFileLister {
    fn operate(&self, path: PathBuf) {
        println!("let's find the files");
        let files = library::find_only_files(&path).unwrap();
        println!("we found them, they are:");
        for f in files.into_iter() {
            println!("{}", f.relative_path.to_string_lossy());
        }
    }
}

pub fn format_duration(seconds: u64) -> String {
    let to_display_seconds = seconds % 60;
    let to_display_minutes = (seconds / 60) % 60;
    let to_display_hours = (seconds / 3600) % 24;
    let to_display_days = seconds / 86400;
    if to_display_days > 0 {
        format!(
            "{}d{:02}h{:02}m{:02}s",
            to_display_days, to_display_hours, to_display_minutes, to_display_seconds,
        )
    } else if to_display_hours > 0 {
        format!(
            "{}h{:02}m{:02}s",
            to_display_hours, to_display_minutes, to_display_seconds
        )
    } else if to_display_minutes > 0 {
        format!("{}m{:02}s", seconds / 60, seconds % 60)
    } else {
        format!("0m{:02}s", seconds)
    }
}
