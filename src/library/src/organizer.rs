use std::collections::{BTreeMap, BTreeSet};
use std::path;
use std::time;

use crate::model::*;

pub fn organize_tracks(
    tracks: Vec<FullTrackMetadata>,
) -> (
    BTreeMap<ID, KeyedArtistAlbums<FullTrackMetadata>>,
    Vec<FullTrackMetadata>,
) {
    let mut conflicts = Vec::new();
    let artist_info = tracks
        .clone()
        .into_iter()
        .fold(BTreeMap::new(), |mut map, track| {
            let albums = map
                .entry(ID::new(&track.album_artist))
                .or_insert(KeyedArtistAlbums {
                    artist_info: ArtistInfo {
                        artist_id: ID::new(&track.album_artist),
                        artist_name: track.clone().album_artist,
                    },
                    albums: BTreeMap::new(),
                });

            let (album_path, relative_album_path) =
                compute_album_paths(&track.disc_total, &track.path, &track.relative_path);

            let album = albums
                .albums
                .entry(ID::new(&track.album))
                .or_insert(KeyedAlbumTracks {
                    album_info: AlbumInfo {
                        album_id: ID::new(&track.album),
                        album_name: track.album.clone(),
                        genres: BTreeSet::new(),
                        total_duration: time::Duration::ZERO,
                        start_date: track.date_number,
                        end_date: track.date_number,
                        last_modified: track.last_modified.clone(),
                        path: album_path,
                        relative_path: relative_album_path,
                    },
                    discs: BTreeMap::new(),
                });

            if track.date_number < album.album_info.start_date {
                album.album_info.start_date = track.date_number
            }
            if track.date_number > album.album_info.end_date {
                album.album_info.end_date = track.date_number
            }

            album.album_info.total_duration = album
                .album_info
                .total_duration
                .checked_add(track.duration.clone())
                .unwrap();

            if track.last_modified > album.album_info.last_modified {
                album.album_info.last_modified = track.last_modified.clone();
            }

            album.album_info.genres.insert(track.genre.clone());

            let album_discs = album.discs.entry(track.disc).or_insert(DiscTracks {
                disc_no: track.disc,
                tracks: BTreeMap::new(),
            });

            let found_conflict = album_discs.tracks.insert(track.track, track.clone());
            match found_conflict {
                Some(v) => conflicts.push(v),
                None => (),
            }

            map
        });

    (artist_info, conflicts)
}

/// Function to compute the path(s) for an album based on known disc information.
///
/// All multi-disc albums have Disc1, Disc2, etc folders for each disc
/// but all single-disc albums have just all tracks immediately in the album folder.
///
/// This logic reconciles this by looking for the single-album-art at the
/// album-level (next to the Disc1, Disc2, etc)
///
/// If an album has a disc total of 1 or does not specify, it is assumed that it has one
/// disc.
///
/// example with the artist "Chillest"'s album "Songs To Dream To" (a 2 disc album) and
/// "The Rockers"'s album "Party Time" (a single disc album):
///   $ tree path/to/music/
///  path/to/music/
///  ├── Chillest
///  │   └── Songs To Dream To
///  │       ├── cover.jpg
///  │       ├── Disc 1
///  │       │   ├── 1.Lying There.flac
///  │       │   ├── 2.Heavy Eyelids.flac
///  │       │   └── 3.Dozing.flac
///  │       └── Disc 2
///  │           ├── 1.Enter the Dream.flac
///  │           ├── 2.The Adventure.flac
///  │           └── 3.Sunlight.flac
///  └── The Rockers
///      └── Party Time
///          ├── 1.Intro.flac
///          ├── 2.The Hit.flac
///          ├── 3.Outro.flac
///          └── cover.jpg
///
///  6 directories, 11 files
pub fn compute_album_paths(
    disc_total: &Option<u64>,
    track_full_path: &path::PathBuf,
    track_relative_path: &path::PathBuf,
) -> (path::PathBuf, path::PathBuf) {
    let (album_path, relative_album_path) = match disc_total {
        Some(v) => {
            if v > &1 {
                (
                    track_full_path
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_path_buf(),
                    track_relative_path
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_path_buf(),
                )
            } else {
                (
                    track_full_path.parent().unwrap().to_path_buf(),
                    track_relative_path.parent().unwrap().to_path_buf(),
                )
            }
        }
        None => (
            track_full_path.parent().unwrap().to_path_buf(),
            track_relative_path.parent().unwrap().to_path_buf(),
        ),
    };
    (album_path, relative_album_path)
}
