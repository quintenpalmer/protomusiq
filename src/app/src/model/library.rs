use std::collections::BTreeMap;

use musiqlibrary;

use crate::datastore::jsonbacked::playlists;

use super::{augmented, common, sorts};

pub struct LibraryState {
    pub raw_library: augmented::AugmentedLibrary,

    pub extra_library: ExtraLibraryKeys,

    pub user_playlists: playlists::PlaylistData,

    pub artist_sorts: sorts::ArtistSorts,
    pub album_sorts: sorts::AlbumSorts,
    pub track_sorts: sorts::TrackSorts,

    pub grid_info: GridInfo,

    pub album_art: common::AlbumArt,
}

impl LibraryState {
    pub fn get_artist_map(
        &self,
    ) -> &BTreeMap<musiqlibrary::ID, musiqlibrary::KeyedArtistAlbums<augmented::AugmentedTrack>>
    {
        &self.raw_library.artists
    }

    pub fn get_album_map(
        &self,
    ) -> BTreeMap<
        musiqlibrary::AlbumUniqueIdentifier,
        &musiqlibrary::KeyedAlbumTracks<augmented::AugmentedTrack>,
    > {
        let mut ret = BTreeMap::new();
        for (artist_id, artist) in self.raw_library.artists.iter() {
            for (album_id, album) in artist.albums.iter() {
                ret.insert(
                    musiqlibrary::AlbumUniqueIdentifier {
                        artist_id: artist_id.clone(),
                        album_id: album_id.clone(),
                    },
                    album,
                );
            }
        }
        ret
    }

    pub fn get_artist_info(&self, artist_id: musiqlibrary::ID) -> musiqlibrary::ArtistInfo {
        self.raw_library
            .artists
            .get(&artist_id)
            .unwrap()
            .artist_info
            .clone()
    }

    pub fn get_artist_album_info(
        &self,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> musiqlibrary::ArtistAlbumInfo {
        let artist = self.raw_library.artists.get(&artist_id).unwrap();

        let album = artist.albums.get(&album_id).unwrap();

        musiqlibrary::ArtistAlbumInfo {
            artist: artist.artist_info.clone(),
            album: album.album_info.clone(),
        }
    }

    #[allow(unused)]
    pub fn get_artist_album_tracks(
        &self,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> &musiqlibrary::KeyedAlbumTracks<augmented::AugmentedTrack> {
        &self
            .raw_library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .get(&album_id)
            .unwrap()
    }

    pub fn get_featured_tracks_for_artist(
        &self,
        artist_id: &musiqlibrary::ID,
    ) -> Vec<augmented::AugmentedTrack> {
        self.extra_library.get_featured_tracks_for_artist(artist_id)
    }

    pub fn get_track(
        &self,
        track_identifier: &musiqlibrary::TrackUniqueIdentifier,
    ) -> &augmented::AugmentedTrack {
        &self
            .raw_library
            .artists
            .get(&track_identifier.artist_id)
            .unwrap()
            .albums
            .get(&track_identifier.album_id)
            .unwrap()
            .discs
            .get(&track_identifier.disc_no)
            .unwrap()
            .tracks
            .get(&track_identifier.track_no)
            .unwrap()
    }

    pub fn get_track_max_play_count(&self) -> usize {
        self.raw_library
            .get_all_tracks()
            .iter()
            .map(|track| track.augmented.play_count)
            .max()
            .unwrap_or(0)
    }

    fn get_artist_first_album(&self, artist_id: &musiqlibrary::ID) -> musiqlibrary::AlbumInfo {
        let artist = self.raw_library.artists.get(&artist_id).unwrap();
        let mut albums = artist.albums.values().collect::<Vec<_>>().clone();
        albums.sort_unstable_by(|a, b| b.album_info.start_date.cmp(&a.album_info.start_date));
        let album = albums[0].album_info.clone();
        album
    }

    pub fn get_artists_first_album_cover(
        &self,
        album_size: common::AlbumSize,
        artist_id: musiqlibrary::ID,
    ) -> Vec<u8> {
        let album_info = self.get_artist_first_album(&artist_id);

        self.album_art
            .get_album_cover(album_size, artist_id, album_info.album_id.clone())
    }

    pub fn get_album_cover(
        &self,
        album_size: common::AlbumSize,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> Vec<u8> {
        self.album_art
            .get_album_cover(album_size, artist_id, album_id)
    }

    pub fn search(&self, query: String) -> common::SearchResults<()> {
        let mut artists = Vec::new();
        let mut artist_albums = Vec::new();
        let mut tracks = Vec::new();
        let mut track_artists = Vec::new();

        for artist in self.raw_library.artists.values() {
            if artist
                .artist_info
                .artist_name
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                artists.push(common::Pair::new_empty(musiqlibrary::ArtistInfo {
                    artist_id: artist.artist_info.artist_id.clone(),
                    artist_name: artist.artist_info.artist_name.clone(),
                }));
            }
            for album in artist.albums.values() {
                if album
                    .album_info
                    .album_name
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                {
                    artist_albums.push(common::Pair::new_empty(musiqlibrary::ArtistAlbumInfo {
                        artist: artist.artist_info.clone(),
                        album: album.album_info.clone(),
                    }));
                }

                for disc in album.discs.values() {
                    for track in disc.tracks.values() {
                        if track
                            .metadata
                            .title
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                        {
                            tracks.push(common::Pair::new_empty(track.clone()));
                        }
                        if track.metadata.track_artist != track.metadata.album_artist {
                            if track
                                .metadata
                                .track_artist
                                .to_lowercase()
                                .contains(&query.to_lowercase())
                            {
                                track_artists.push(common::Pair::new_empty(track.clone()));
                            }
                        }
                    }
                }
            }
        }

        artists
            .sort_unstable_by(|a, b| a.first.artist_name.cmp(&b.first.artist_name.to_lowercase()));

        artist_albums.sort_unstable_by(|a, b| {
            a.first
                .album
                .album_name
                .to_lowercase()
                .cmp(&b.first.album.album_name.to_lowercase())
        });

        tracks.sort_unstable_by(|a, b| {
            a.first
                .metadata
                .title
                .to_lowercase()
                .cmp(&b.first.metadata.title.to_lowercase())
        });

        common::SearchResults {
            artists: artists,
            albums: artist_albums,
            tracks: tracks,
            track_artists: track_artists,
        }
    }
}

pub struct ExtraLibraryKeys {
    pub featured_artists: BTreeMap<musiqlibrary::ID, Vec<augmented::AugmentedTrack>>,
}

impl ExtraLibraryKeys {
    pub fn from_library(library: &augmented::AugmentedLibrary) -> Self {
        let mut featured: BTreeMap<musiqlibrary::ID, Vec<augmented::AugmentedTrack>> =
            BTreeMap::new();

        for track in library.get_all_tracks().iter() {
            match track.metadata.get_maybe_track_artist() {
                Some(track_artist) => featured
                    .entry(musiqlibrary::ID::new(&track_artist))
                    .or_insert(Vec::new())
                    .push(track.clone().clone()),
                None => (),
            }
        }

        ExtraLibraryKeys {
            featured_artists: featured,
        }
    }

    pub fn get_featured_tracks_for_artist(
        &self,
        artist_id: &musiqlibrary::ID,
    ) -> Vec<augmented::AugmentedTrack> {
        match self.featured_artists.get(artist_id) {
            Some(v) => v.clone(),
            None => Vec::new(),
        }
    }
}

pub struct GridInfo {
    layout_width: u32,
    layout_height: u32,
    track_multiplier: u32,
}

impl GridInfo {
    pub fn new(width: u32, height: u32, track_multiplier: u32) -> Self {
        GridInfo {
            layout_width: width,
            layout_height: height,
            track_multiplier: track_multiplier,
        }
    }

    pub fn get_layout_width(&self) -> u32 {
        self.layout_width
    }

    pub fn get_layout_height(&self) -> u32 {
        self.layout_height
    }

    pub fn get_page_size_usize(&self) -> usize {
        (self.layout_width * self.layout_height) as usize
    }

    pub fn get_track_page_size_usize(&self) -> usize {
        (self.layout_width * self.layout_height * self.track_multiplier) as usize
    }
}
