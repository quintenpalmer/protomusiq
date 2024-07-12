use std::collections::BTreeMap;

use super::super::util;

use super::structs;

pub struct SixtyFourLibrary {
    pub artists: BTreeMap<musiqlibrary::ID, Artist>,
}

impl SixtyFourLibrary {
    #[allow(dead_code)]
    pub fn from_db_rows(
        input_artists: Vec<structs::Artist>,
        input_albums: Vec<structs::Album>,
        input_discs: Vec<structs::Disc>,
        input_tracks: Vec<structs::Track>,
    ) -> Self {
        let mut albums_by_artist = util::key_into_vec_by(input_albums, |album| album.artist_id);
        let mut discs_by_album = util::key_into_vec_by(input_discs, |disc| disc.album_id);
        let mut tracks_by_disc = util::key_into_vec_by(input_tracks, |track| track.disc_id);

        let mut library = SixtyFourLibrary {
            artists: BTreeMap::new(),
        };

        for input_artist in input_artists.into_iter() {
            let mut artist = Artist {
                info: input_artist.clone(),
                albums: BTreeMap::new(),
            };

            let albums = albums_by_artist.remove(&input_artist.id).unwrap();

            for input_album in albums.into_iter() {
                let mut album = Album {
                    info: input_album.clone(),
                    discs: BTreeMap::new(),
                };

                let discs = discs_by_album.remove(&input_album.id).unwrap();

                for input_disc in discs.into_iter() {
                    let mut disc = Disc {
                        info: input_disc.clone(),
                        tracks: BTreeMap::new(),
                    };

                    let tracks = tracks_by_disc.remove(&input_disc.id).unwrap();

                    for input_track in tracks.into_iter() {
                        disc.tracks.insert(input_track.track_no as u64, input_track);
                    }

                    album.discs.insert(input_disc.disc_no as u64, disc);
                }

                artist
                    .albums
                    .insert(musiqlibrary::ID::new(&input_album.name), album);
            }
            library
                .artists
                .insert(musiqlibrary::ID::new(&input_artist.name), artist);
        }

        library
    }

    pub fn track_from_unique_key(
        &self,
        key: &musiqlibrary::TrackUniqueIdentifier,
    ) -> &structs::Track {
        self.artists
            .get(&key.artist_id)
            .unwrap()
            .albums
            .get(&key.album_id)
            .unwrap()
            .discs
            .get(&key.disc_no)
            .unwrap()
            .tracks
            .get(&key.track_no)
            .unwrap()
    }
}

pub struct Artist {
    #[allow(unused)]
    pub info: structs::Artist,
    pub albums: BTreeMap<musiqlibrary::ID, Album>,
}

pub struct Album {
    #[allow(unused)]
    pub info: structs::Album,
    pub discs: BTreeMap<u64, Disc>,
}

pub struct Disc {
    #[allow(unused)]
    pub info: structs::Disc,
    pub tracks: BTreeMap<u64, structs::Track>,
}
