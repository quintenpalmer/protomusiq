use std::path;

use crate::util::logging;

use super::super::super::constants;
use super::super::super::query::{albums, artists, discs, rawtracks, tracks};

pub fn seed_tracks(
    tracks: &Vec<musiqlibrary::FullTrackMetadata>,
    main_db: &mut rusqlite::Connection,
) {
    let mut logger = logging::Logger::new(logging::LogType::Timing, "seed tracks from vec");

    let tx = main_db.transaction().unwrap();

    let library =
        musiqlibrary::RawLibrary::from_track_list::<path::PathBuf>(None, tracks.clone()).unwrap();

    for artist in library.artists.values() {
        let artist_id = artists::insert_artist(&tx, &artist.artist_info);
        for album in artist.albums.values() {
            let disc_total = album.discs.values().collect::<Vec<_>>()[0]
                .tracks
                .values()
                .collect::<Vec<_>>()[0]
                .disc_total
                .unwrap_or(1) as u32;
            let album_id = albums::insert_album(&tx, artist_id, disc_total, &album.album_info);
            for disc in album.discs.values() {
                let disc_id = discs::insert_disc(&tx, album_id, disc.disc_no as u32);
                for track in disc.tracks.values() {
                    tracks::insert_track(&tx, disc_id, track);
                }
            }
        }
    }

    for track in tracks.iter() {
        println!("about to insert {}", track.title);
        rawtracks::insert_raw_track(&tx, track);
        logger.print_elapsed(format!("inserting track {}", track.title));
    }

    tx.execute(
        "INSERT INTO data_migrations VALUES (
                ?,
                ?)",
        rusqlite::params![
            rusqlite::types::Null,
            constants::Migration::TracksAndFriends.get_name()
        ],
    )
    .unwrap();

    tx.commit().unwrap();
}
