use std::path::PathBuf;

use musiqlibrary as library;

use super::commands::AppCmd;

mod difflibs;
pub mod misc;
mod reconcile;

pub use difflibs::LibDiffer;
pub use reconcile::TrackerReconciler;

pub struct ConflictLister {}

impl AppCmd for ConflictLister {
    fn operate(&self, path: PathBuf) {
        eprintln!("finding conflicts by artist/album/disc/track...");
        let tracks = library::find_files(&path).unwrap();
        let (_library, conflicts) = library::organize_tracks(tracks);
        for conflict in conflicts.into_iter() {
            println!("\t{}", conflict.path.to_str().unwrap());
        }
        eprintln!("...done reporting said conflicts");
    }
}

pub struct TreeViewer {}

impl AppCmd for TreeViewer {
    fn operate(&self, path: PathBuf) {
        let library = library::model::RawLibrary::new(path.clone()).unwrap();
        eprintln!("Library:");
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

        eprintln!("Library Table:");
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

pub struct MusicFileLister {}

impl AppCmd for MusicFileLister {
    fn operate(&self, path: PathBuf) {
        eprintln!("let's find the files");
        let files = library::find_only_files(&path).unwrap();
        eprintln!("we found them, they are:");
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
