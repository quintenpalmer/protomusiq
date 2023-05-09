use std::collections::BTreeMap;

use rand::seq::SliceRandom;

use musiqlibrary;

use crate::datastore;
use crate::datastore::jsonbacked::playlists;

use super::common;

#[derive(Debug)]
pub struct PrehistoryRecord {
    pub source: String,
    pub key: musiqlibrary::TrackUniqueIdentifier,
    pub count: u32,
}

pub type AugmentedLibrary = musiqlibrary::Library<AugmentedTrack>;

pub fn augmented_from_raw(
    raw_library: musiqlibrary::RawLibrary,
    tracked_data: Box<dyn datastore::traits::LiveReadOnlyTrackCountReporter>,
    historical_data: Box<dyn datastore::traits::HistoricalTrackCountReporter>,
) -> AugmentedLibrary {
    raw_library.map_into(&|track| {
        let uniq_track_id = musiqlibrary::TrackUniqueIdentifier::from_track(&track);
        let live_play_count = tracked_data.get_live_track_count(&uniq_track_id);
        let historical_play_count = historical_data.get_historical_track_count(&uniq_track_id);
        let total_play_count = live_play_count + historical_play_count;

        AugmentedTrack {
            augmented: AugmentedData {
                play_count: total_play_count,
                tagged_genres: Vec::new(),
            },
            metadata: track,
        }
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AugmentedTrack {
    pub metadata: musiqlibrary::FullTrackMetadata,
    pub augmented: AugmentedData,
}

impl AugmentedTrack {
    pub fn played_seconds(&self) -> u64 {
        self.metadata.duration.as_secs() * (self.augmented.play_count as u64)
    }
}

impl Into<musiqlibrary::FullTrackMetadata> for AugmentedTrack {
    fn into(self) -> musiqlibrary::FullTrackMetadata {
        self.metadata
    }
}

pub fn album_total_play_count(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> usize {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.augmented.play_count;
        }
    }
    total
}

pub fn artist_total_play_count(artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>) -> usize {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_total_play_count(album);
    }
    total
}

pub fn album_total_played_duration(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> u64 {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.played_seconds();
        }
    }
    total
}

pub fn artist_total_played_duration(
    artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>,
) -> u64 {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_total_played_duration(album);
    }
    total
}

pub fn album_track_duration_total(album: &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>) -> u64 {
    let mut total = 0;
    for (_, disc) in album.discs.iter() {
        for (_, track) in disc.tracks.iter() {
            total += track.metadata.duration.as_secs();
        }
    }
    total
}

pub fn artist_track_duration_total(
    artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>,
) -> u64 {
    let mut total = 0;
    for (_, album) in artist.albums.iter() {
        total += album_track_duration_total(album);
    }
    total
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AugmentedData {
    pub play_count: usize,
    pub tagged_genres: Vec<String>,
}

pub struct LibraryState {
    pub raw_library: AugmentedLibrary,

    pub user_playlists: playlists::PlaylistData,

    pub artist_sorts: ArtistSorts,
    pub album_sorts: AlbumSorts,
    pub track_sorts: TrackSorts,

    pub grid_info: GridInfo,

    pub album_art: AlbumArt,
}

pub struct GridInfo {
    layout_width: u32,
    layout_height: u32,
}

impl GridInfo {
    pub fn new(width: u32, height: u32) -> Self {
        GridInfo {
            layout_width: width,
            layout_height: height,
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
}

impl LibraryState {
    pub fn get_artist_map(
        &self,
    ) -> &BTreeMap<musiqlibrary::ID, musiqlibrary::KeyedArtistAlbums<AugmentedTrack>> {
        &self.raw_library.artists
    }

    pub fn get_album_map(
        &self,
    ) -> BTreeMap<
        musiqlibrary::AlbumUniqueIdentifier,
        &musiqlibrary::KeyedAlbumTracks<AugmentedTrack>,
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

    pub fn get_artist_album_tracks(
        &self,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> &musiqlibrary::KeyedAlbumTracks<AugmentedTrack> {
        &self
            .raw_library
            .artists
            .get(&artist_id)
            .unwrap()
            .albums
            .get(&album_id)
            .unwrap()
    }

    pub fn get_track(
        &self,
        track_identifier: &musiqlibrary::TrackUniqueIdentifier,
    ) -> &AugmentedTrack {
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

pub struct ArtistSorts {
    pub by_name: ListAndReversed<musiqlibrary::ID>,
    pub by_play_count: ListAndReversed<musiqlibrary::ID>,
    pub by_album_count: ListAndReversed<musiqlibrary::ID>,
    pub by_track_count: ListAndReversed<musiqlibrary::ID>,
    pub by_track_duration: ListAndReversed<musiqlibrary::ID>,
    pub by_duration_played: ListAndReversed<musiqlibrary::ID>,
    pub random: ListAndReversed<musiqlibrary::ID>,
}

impl ArtistSorts {
    pub fn new(organized: &AugmentedLibrary) -> Self {
        ArtistSorts {
            by_name: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    a.artist_info
                        .artist_name
                        .to_lowercase()
                        .cmp(&b.artist_info.artist_name.to_lowercase())
                });

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_play_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    artist_total_play_count(a).cmp(&artist_total_play_count(b))
                });

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_album_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.album_count().cmp(&b.album_count()));

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_track_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.track_count().cmp(&b.track_count()));

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_track_duration: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    artist_track_duration_total(a).cmp(&artist_track_duration_total(b))
                });

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_duration_played: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    artist_total_played_duration(a).cmp(&artist_total_played_duration(b))
                });

                ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            random: {
                let mut rng = rand::thread_rng();
                let mut artist_ids: Vec<musiqlibrary::ID> = organized
                    .artists
                    .values()
                    .map(|x| x.artist_info.artist_id.clone())
                    .collect();
                artist_ids.shuffle(&mut rng);
                ListAndReversed::new(artist_ids)
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &common::ArtistSortKey,
        sort_order: &common::SortOrder,
    ) -> &Vec<musiqlibrary::ID> {
        match sort_key {
            common::ArtistSortKey::ByName => &self.by_name,
            common::ArtistSortKey::ByPlayCount => &self.by_play_count,
            common::ArtistSortKey::ByAlbumCount => &self.by_album_count,
            common::ArtistSortKey::ByTrackCount => &self.by_track_count,
            common::ArtistSortKey::ByTrackDuration => &self.by_track_duration,
            common::ArtistSortKey::ByPlayedDuration => &self.by_duration_played,
            common::ArtistSortKey::Random => &self.random,
        }
        .sort_ordered(&sort_order)
    }
}

pub struct AlbumSorts {
    pub by_name: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_artist: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_date: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_last_modified: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_duration: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_total_play_count: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_total_played_duration: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub random: ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
}

impl AlbumSorts {
    pub fn new(organized: &AugmentedLibrary) -> Self {
        AlbumSorts {
            by_name: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x.album_info.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| {
                    a.1.album_name
                        .to_lowercase()
                        .cmp(&b.1.album_name.to_lowercase())
                });

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_id.clone()))
                        .collect(),
                )
            },
            by_date: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x.album_info.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| a.1.start_date.cmp(&b.1.start_date));

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_id.clone()))
                        .collect(),
                )
            },
            by_last_modified: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x.album_info.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| a.1.last_modified.cmp(&b.1.last_modified));

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_id.clone()))
                        .collect(),
                )
            },
            by_artist: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x.album_info.clone()))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| {
                    a.0.artist_name
                        .to_lowercase()
                        .cmp(&b.0.artist_name.to_lowercase())
                });

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_id.clone()))
                        .collect(),
                )
            },
            by_duration: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums
                    .sort_unstable_by(|a, b| a.1.duration_seconds().cmp(&b.1.duration_seconds()));

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_info.album_id.clone()))
                        .collect(),
                )
            },
            by_total_play_count: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| {
                    album_total_play_count(&a.1).cmp(&album_total_play_count(b.1))
                });

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_info.album_id.clone()))
                        .collect(),
                )
            },
            by_total_played_duration: {
                let mut unpaged_albums =
                    organized
                        .artists
                        .values()
                        .fold(Vec::new(), |mut total, artist| {
                            total.append(
                                &mut artist
                                    .albums
                                    .values()
                                    .map(|x| (artist.artist_info.clone(), x))
                                    .collect::<Vec<_>>(),
                            );
                            total
                        });

                unpaged_albums.sort_unstable_by(|a, b| {
                    album_total_played_duration(&a.1).cmp(&album_total_played_duration(b.1))
                });

                ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id.clone(), a.1.album_info.album_id.clone()))
                        .collect(),
                )
            },
            random: {
                let mut rng = rand::thread_rng();
                let mut album_ids: Vec<(musiqlibrary::ID, musiqlibrary::ID)> = organized
                    .artists
                    .values()
                    .fold(Vec::new(), |mut total, artist| {
                        total.append(
                            &mut artist
                                .albums
                                .values()
                                .map(|x| {
                                    (
                                        artist.artist_info.artist_id.clone(),
                                        x.album_info.album_id.clone(),
                                    )
                                })
                                .collect::<Vec<_>>(),
                        );
                        total
                    });
                album_ids.shuffle(&mut rng);
                ListAndReversed::new(album_ids)
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &common::AlbumSortKey,
        sort_order: &common::SortOrder,
    ) -> &Vec<(musiqlibrary::ID, musiqlibrary::ID)> {
        match sort_key {
            common::AlbumSortKey::ByName => &self.by_name,
            common::AlbumSortKey::ByParent => &self.by_artist,
            common::AlbumSortKey::ByDate => &self.by_date,
            common::AlbumSortKey::ByDuration => &self.by_duration,
            common::AlbumSortKey::ByLastMod => &self.by_last_modified,
            common::AlbumSortKey::ByTotalPlayCount => &self.by_total_play_count,
            common::AlbumSortKey::ByTotalPlayedDuration => &self.by_total_played_duration,
            common::AlbumSortKey::Random => &self.random,
        }
        .sort_ordered(&sort_order)
    }
}

pub struct AlbumTrackSorts {
    pub by_name: ListAndReversed<AugmentedTrack>,
    pub by_album: ListAndReversed<AugmentedTrack>,
    pub by_duration: ListAndReversed<AugmentedTrack>,
    pub by_total_play_count: ListAndReversed<AugmentedTrack>,
    pub by_total_played_duration: ListAndReversed<AugmentedTrack>,
    pub random: ListAndReversed<AugmentedTrack>,
}

impl AlbumTrackSorts {
    pub fn new(artist: &musiqlibrary::KeyedArtistAlbums<AugmentedTrack>) -> Self {
        AlbumTrackSorts {
            by_name: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.sort_unstable_by(|a, b| {
                    a.metadata
                        .title
                        .to_lowercase()
                        .cmp(&b.metadata.title.to_lowercase())
                });

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
            by_album: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.metadata.date_number.cmp(&b.metadata.date_number));

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
            by_duration: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.metadata.duration.cmp(&b.metadata.duration));

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
            by_total_play_count: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.augmented.play_count.cmp(&b.augmented.play_count));

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
            by_total_played_duration: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.sort_unstable_by(|a, b| a.played_seconds().cmp(&b.played_seconds()));

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
            random: {
                let mut rng = rand::thread_rng();

                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.shuffle(&mut rng);

                ListAndReversed::new(unpaged_tracks.into_iter().map(|a| a.clone()).collect())
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &common::ArtistTrackSortKey,
        sort_order: &common::SortOrder,
    ) -> &Vec<AugmentedTrack> {
        match sort_key {
            common::ArtistTrackSortKey::ByName => &self.by_name,
            common::ArtistTrackSortKey::ByParent => &self.by_album,
            common::ArtistTrackSortKey::ByDuration => &self.by_duration,
            common::ArtistTrackSortKey::ByTotalPlayCount => &self.by_total_play_count,
            common::ArtistTrackSortKey::ByTotalPlayedDuration => &self.by_total_played_duration,
            common::ArtistTrackSortKey::Random => &self.random,
        }
        .sort_ordered(&sort_order)
    }
}

pub struct TrackSorts {
    pub by_name: ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_play_count: ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_duration: ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_played_amount: ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub random: ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
}

impl TrackSorts {
    pub fn new(organized: &AugmentedLibrary) -> Self {
        TrackSorts {
            by_name: {
                let mut unpaged_tracks = Vec::new();
                for artist in organized.artists.values() {
                    for album in artist.albums.values() {
                        for disc_tracks in album.discs.values() {
                            for track in disc_tracks.tracks.values() {
                                unpaged_tracks.push(track.clone());
                            }
                        }
                    }
                }

                unpaged_tracks.sort_unstable_by(|a, b| {
                    a.metadata
                        .title
                        .to_lowercase()
                        .cmp(&b.metadata.title.to_lowercase())
                });

                ListAndReversed::new(
                    unpaged_tracks
                        .iter()
                        .map(|a| musiqlibrary::TrackUniqueIdentifier::from_track(&a.metadata))
                        .collect(),
                )
            },
            by_play_count: {
                let mut unpaged_tracks = Vec::new();
                for artist in organized.artists.values() {
                    for album in artist.albums.values() {
                        for disc_tracks in album.discs.values() {
                            for track in disc_tracks.tracks.values() {
                                unpaged_tracks.push(track.clone());
                            }
                        }
                    }
                }

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.augmented.play_count.cmp(&b.augmented.play_count));

                ListAndReversed::new(
                    unpaged_tracks
                        .iter()
                        .map(|a| musiqlibrary::TrackUniqueIdentifier::from_track(&a.metadata))
                        .collect(),
                )
            },
            by_duration: {
                let mut unpaged_tracks = Vec::new();
                for artist in organized.artists.values() {
                    for album in artist.albums.values() {
                        for disc_tracks in album.discs.values() {
                            for track in disc_tracks.tracks.values() {
                                unpaged_tracks.push(track.clone());
                            }
                        }
                    }
                }

                unpaged_tracks.sort_unstable_by(|a, b| {
                    a.metadata
                        .duration
                        .as_secs()
                        .cmp(&b.metadata.duration.as_secs())
                });

                ListAndReversed::new(
                    unpaged_tracks
                        .iter()
                        .map(|a| musiqlibrary::TrackUniqueIdentifier::from_track(&a.metadata))
                        .collect(),
                )
            },
            by_played_amount: {
                let mut unpaged_tracks = Vec::new();
                for artist in organized.artists.values() {
                    for album in artist.albums.values() {
                        for disc_tracks in album.discs.values() {
                            for track in disc_tracks.tracks.values() {
                                unpaged_tracks.push(track.clone());
                            }
                        }
                    }
                }

                unpaged_tracks.sort_unstable_by(|a, b| a.played_seconds().cmp(&b.played_seconds()));

                ListAndReversed::new(
                    unpaged_tracks
                        .iter()
                        .map(|a| musiqlibrary::TrackUniqueIdentifier::from_track(&a.metadata))
                        .collect(),
                )
            },
            random: {
                let mut rng = rand::thread_rng();

                let mut unpaged_track_ids = Vec::new();
                for artist in organized.artists.values() {
                    for album in artist.albums.values() {
                        for disc_tracks in album.discs.values() {
                            for track in disc_tracks.tracks.values() {
                                unpaged_track_ids.push(
                                    musiqlibrary::TrackUniqueIdentifier::from_track(
                                        &track.metadata,
                                    ),
                                );
                            }
                        }
                    }
                }

                unpaged_track_ids.shuffle(&mut rng);
                ListAndReversed::new(unpaged_track_ids)
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &common::TrackSortKey,
        sort_order: &common::SortOrder,
    ) -> &Vec<musiqlibrary::TrackUniqueIdentifier> {
        match sort_key {
            common::TrackSortKey::ByName => &self.by_name,
            common::TrackSortKey::ByPlayCount => &self.by_play_count,
            common::TrackSortKey::ByDuration => &self.by_duration,
            common::TrackSortKey::ByPlayedAmount => &self.by_played_amount,
            common::TrackSortKey::ByRandom => &self.random,
        }
        .sort_ordered(&sort_order)
    }
}

pub struct ListAndReversed<T> {
    pub regular: Vec<T>,
    pub reversed: Vec<T>,
}

impl<T: Clone> ListAndReversed<T> {
    pub fn new(regular: Vec<T>) -> Self {
        let mut reversed = regular.clone();
        reversed.reverse();

        ListAndReversed {
            regular: regular,
            reversed: reversed,
        }
    }

    pub fn sort_ordered(&self, sort_order: &common::SortOrder) -> &Vec<T> {
        match sort_order {
            common::SortOrder::Regular => &self.regular,
            common::SortOrder::Reversed => &self.reversed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlbumArt {
    pub large_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub small_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub mini_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub micro_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
    pub orig_album_covers: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, Vec<u8>>,
}

impl AlbumArt {
    pub fn get_album_cover(
        &self,
        album_size: common::AlbumSize,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> Vec<u8> {
        match album_size {
            common::AlbumSize::Large => &self.large_album_covers,
            common::AlbumSize::Regular => &self.album_covers,
            common::AlbumSize::Small => &self.small_album_covers,
            common::AlbumSize::Mini => &self.mini_album_covers,
            common::AlbumSize::Micro => &self.micro_album_covers,
        }
        .get(&musiqlibrary::AlbumUniqueIdentifier::new(
            artist_id, album_id,
        ))
        .unwrap()
        .clone()
    }
}
