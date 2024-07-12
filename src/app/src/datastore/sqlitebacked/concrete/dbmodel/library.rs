use std::collections::BTreeMap;

use super::super::util;

use super::structs;

pub struct Library {
    pub artists: BTreeMap<u32, Artist>,
}

impl Library {
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

        let mut library = Library {
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
                        disc.tracks.insert(input_track.id, input_track);
                    }

                    album.discs.insert(input_disc.id, disc);
                }

                artist.albums.insert(input_album.id, album);
            }
            library.artists.insert(input_artist.id, artist);
        }

        library
    }
}

pub struct Artist {
    #[allow(unused)]
    pub info: structs::Artist,
    pub albums: BTreeMap<u32, Album>,
}

pub struct Album {
    #[allow(unused)]
    pub info: structs::Album,
    pub discs: BTreeMap<u32, Disc>,
}

pub struct Disc {
    #[allow(unused)]
    pub info: structs::Disc,
    pub tracks: BTreeMap<u32, structs::Track>,
}
