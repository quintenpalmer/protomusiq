use rusqlite;

pub fn repopulate_and_soft_delete(
    main_db: &mut rusqlite::Connection,
    library: &musiqlibrary::RawLibrary,
) -> chrono::DateTime<chrono::Local> {
    let now = chrono::Local::now();

    let tx = main_db.transaction().unwrap();

    tx.execute(
        "UPDATE artists SET deleted_at = ? WHERE deleted_at IS NULL",
        rusqlite::params![now],
    )
    .unwrap();
    tx.execute(
        "UPDATE albums SET deleted_at = ? WHERE deleted_at IS NULL",
        rusqlite::params![now],
    )
    .unwrap();
    tx.execute(
        "UPDATE discs SET deleted_at = ? WHERE deleted_at IS NULL",
        rusqlite::params![now],
    )
    .unwrap();
    tx.execute(
        "UPDATE tracks SET deleted_at = ? WHERE deleted_at IS NULL",
        rusqlite::params![now],
    )
    .unwrap();

    for artist in library.artists.values() {
        let artist_id: u32 = tx
            .query_row(
                "UPDATE artists SET deleted_at = NULL WHERE name = ? RETURNING id",
                rusqlite::params![artist.artist_info.artist_name,],
                |row| row.get(0),
            )
            .unwrap();

        for album in artist.albums.values() {
            let album_id: u32 = tx.query_row(
                "UPDATE albums SET deleted_at = NULL WHERE artist_id = ? AND name = ? RETURNING id",
                rusqlite::params![
                    artist_id,
                    album.album_info.album_name,
                ],
                |row| row.get(0),
            )
            .unwrap();

            for disc in album.discs.values() {
                let disc_id: u32 = tx.query_row(
                    "UPDATE discs SET deleted_at = NULL WHERE album_id = ? AND disc_no = ? RETURNING id",
                    rusqlite::params![
                        album_id,
                        disc.disc_no,
                    ],
                    |row| row.get(0),
                )
                .unwrap();

                for track in disc.tracks.values() {
                    tx.execute(
                        "UPDATE tracks SET deleted_at = NULL WHERE disc_id = ? and track_no = ?",
                        rusqlite::params![disc_id, track.track],
                    )
                    .unwrap();
                }
            }
        }
    }

    tx.commit().unwrap();

    now
}
