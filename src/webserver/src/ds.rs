use std::fs;
use std::path::PathBuf;

use crate::model::{
    AlbumInfo, ArtistInfo, FullTrackMetadata, RawLibrary, SortedAlbumDiscs, SortedArtistAlbums,
    SortedDiscTracks, ID,
};

#[derive(Clone)]
pub struct Datastore {
    library: RawLibrary,
}

impl Datastore {
    pub fn new(lib_path: PathBuf) -> Self {
        let library = RawLibrary::new(lib_path).unwrap();

        Datastore { library: library }
    }

    pub fn list_artists(&self) -> Vec<ArtistInfo> {
        let mut artists: Vec<ArtistInfo> = self
            .library
            .artists
            .values()
            .map(|artist| ArtistInfo {
                artist_id: artist.artist_info.artist_id.clone(),
                artist_name: artist.artist_info.artist_name.clone(),
            })
            .collect();
        artists.sort_unstable_by_key(|x| x.artist_name.to_lowercase());
        artists
    }

    pub fn list_artist_albums(&self, artist_id: ID) -> Vec<AlbumInfo> {
        let mut albums: Vec<AlbumInfo> = self
            .library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .iter()
            .map(|(_key, val)| val.album_info.clone())
            .collect();
        albums.sort_unstable_by_key(|x| x.album_name.to_lowercase());
        albums
    }

    pub fn list_artist_album_tracks(&self, artist_id: ID, album_id: ID) -> Vec<FullTrackMetadata> {
        let mut tracks: Vec<FullTrackMetadata> = self
            .library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .get(&album_id)
            .unwrap()
            .discs
            .values()
            .fold(Vec::new(), |mut total, current| {
                for track in current.tracks.values() {
                    total.push(track.clone());
                }
                total
            });
        tracks.sort_unstable_by_key(|track| track.track);
        tracks.sort_by_key(|track| track.disc);
        tracks
    }

    pub fn list_albums(&self) -> Vec<(ArtistInfo, AlbumInfo)> {
        let mut albums: Vec<(ArtistInfo, AlbumInfo)> = self
            .library
            .artists
            .values()
            .map(|x| {
                (
                    ArtistInfo {
                        artist_id: x.artist_info.artist_id.clone(),
                        artist_name: x.artist_info.artist_name.clone(),
                    },
                    x.albums
                        .values()
                        .map(|album| album.album_info.clone())
                        .collect::<Vec<AlbumInfo>>(),
                )
            })
            .fold(Vec::new(), |mut total, (artist, album_list)| {
                for album in album_list.into_iter() {
                    total.push((artist.clone(), album))
                }
                total
            });
        albums.sort_unstable_by_key(|(_artist, album)| album.album_name.clone().to_lowercase());
        albums.sort_by_key(|(artist, _album)| artist.artist_name.clone().to_lowercase());
        albums
    }

    pub fn list_tracks(&self) -> Vec<FullTrackMetadata> {
        let mut tracks =
            self.library
                .artists
                .values()
                .fold(Vec::new(), |mut total, artist_info| {
                    for album_info in artist_info.albums.values() {
                        for disc in album_info.discs.values() {
                            for track in disc.tracks.values() {
                                total.push(track.clone());
                            }
                        }
                    }
                    total
                });
        tracks.sort_unstable_by_key(|x| x.track.clone());
        tracks.sort_by_key(|x| x.album.clone().to_lowercase());
        tracks.sort_by_key(|x| x.disc.clone());
        tracks.sort_by_key(|x| x.album_artist.clone().to_lowercase());
        tracks
    }

    pub fn tree(&self) -> Vec<SortedArtistAlbums<FullTrackMetadata>> {
        let mut artist_info: Vec<SortedArtistAlbums<FullTrackMetadata>> = self
            .library
            .artists
            .values()
            .map(|artist| {
                let mut albums: Vec<SortedAlbumDiscs<FullTrackMetadata>> = artist
                    .albums
                    .values()
                    .map(|album| {
                        let mut discs = Vec::new();
                        for disc in album.discs.values() {
                            let mut tracks = Vec::new();
                            for track in disc.tracks.values() {
                                tracks.push(track.clone());
                            }
                            discs.push(SortedDiscTracks { tracks: tracks });
                        }
                        //tracks.sort_unstable_by_key(|track| track.track);
                        //tracks.sort_by_key(|track| track.disc);
                        SortedAlbumDiscs {
                            album_info: album.album_info.clone(),
                            path: album.album_info.path.clone(),
                            discs: discs,
                        }
                    })
                    .collect();
                albums.sort_by_key(|x| x.album_info.album_name.clone().to_lowercase());
                SortedArtistAlbums {
                    artist_info: artist.artist_info.clone(),
                    albums: albums,
                }
            })
            .collect();
        artist_info.sort_by_key(|x| x.artist_info.artist_name.clone().to_lowercase());
        artist_info
    }

    pub fn get_artist_from_id(&self, artist_id: ID) -> ArtistInfo {
        ArtistInfo {
            artist_id: artist_id.clone(),
            artist_name: self
                .library
                .artists
                .get(&artist_id)
                .unwrap()
                .artist_info
                .artist_name
                .to_string(),
        }
    }

    pub fn get_artist_album_from_id(&self, artist_id: ID, album_id: ID) -> AlbumInfo {
        self.library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .get(&album_id)
            .unwrap()
            .album_info
            .clone()
    }

    pub fn get_artist_album_track_name_from_id(
        &self,
        artist_id: ID,
        album_id: ID,
        disc_no: u64,
        track_no: u64,
    ) -> FullTrackMetadata {
        self.library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .get(&album_id)
            .unwrap()
            .discs
            .get(&disc_no)
            .unwrap()
            .tracks
            .get(&track_no)
            .unwrap()
            .clone()
    }

    pub fn get_artist_album_track_audio_from_id(
        &self,
        artist_id: ID,
        album_id: ID,
        disc_no: u64,
        track_no: u64,
    ) -> Vec<u8> {
        fs::read(
            self.library
                .artists
                .get(&artist_id)
                .unwrap()
                .albums
                .get(&album_id)
                .unwrap()
                .discs
                .get(&disc_no)
                .unwrap()
                .tracks
                .get(&track_no)
                .unwrap()
                .path
                .clone(),
        )
        .unwrap()
    }

    pub fn get_artist_album_cover(&self, artist_id: ID, album_id: ID) -> Vec<u8> {
        fs::read(
            self.library
                .artists
                .get(&artist_id)
                .unwrap()
                .albums
                .get(&album_id)
                .unwrap()
                .album_info
                .path
                .clone()
                .join("cover.jpg"),
        )
        .unwrap()
    }
}
