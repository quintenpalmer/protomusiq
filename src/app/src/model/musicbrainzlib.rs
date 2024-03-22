use std::collections::BTreeMap;

use crate::datastore::cache;

pub struct Library {
    pub artists: BTreeMap<musiqlibrary::ID, musicbrainz::Artist>,
}

impl Library {
    pub fn new(library: &musiqlibrary::RawLibrary) -> Self {
        let mut mb_artists = BTreeMap::new();

        let musicbrainz_cache = cache::MusicBrainzCacheInterface::new();

        for ml_artist in library.artists.values() {
            match musicbrainz_cache
                .read_musicbrainz_artist_approved_file(ml_artist.artist_info.artist_name.clone())
            {
                Some(mb_artist) => {
                    let _ = mb_artists.insert(ml_artist.artist_info.artist_id.clone(), mb_artist);
                }
                None => (),
            };
        }

        Library {
            artists: mb_artists,
        }
    }

    pub fn get_artist_info(&self, artist_id: &musiqlibrary::ID) -> Option<&musicbrainz::Artist> {
        self.artists.get(artist_id)
    }
}
