use std::collections::BTreeMap;

use rand::seq::SliceRandom;

use musiqlibrary;

use crate::datastore::jsonbacked::playlists;

use super::{augmented, common};

#[derive(Debug)]
pub struct PrehistoryRecord {
    pub source: String,
    pub key: musiqlibrary::TrackUniqueIdentifier,
    pub count: u32,
}

pub struct LibraryState {
    pub raw_library: augmented::AugmentedLibrary,

    pub user_playlists: playlists::PlaylistData,

    pub artist_sorts: ArtistSorts,
    pub album_sorts: AlbumSorts,
    pub track_sorts: TrackSorts,

    pub grid_info: GridInfo,

    pub album_art: common::AlbumArt,
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

pub struct ArtistSorts {
    pub by_name: common::ListAndReversed<musiqlibrary::ID>,
    pub by_play_count: common::ListAndReversed<musiqlibrary::ID>,
    pub by_album_count: common::ListAndReversed<musiqlibrary::ID>,
    pub by_track_count: common::ListAndReversed<musiqlibrary::ID>,
    pub by_track_duration: common::ListAndReversed<musiqlibrary::ID>,
    pub by_duration_played: common::ListAndReversed<musiqlibrary::ID>,
    pub random: common::ListAndReversed<musiqlibrary::ID>,
}

impl ArtistSorts {
    pub fn new(organized: &augmented::AugmentedLibrary) -> Self {
        ArtistSorts {
            by_name: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    a.artist_info
                        .artist_name
                        .to_lowercase()
                        .cmp(&b.artist_info.artist_name.to_lowercase())
                });

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_play_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    augmented::artist_total_play_count(a)
                        .cmp(&augmented::artist_total_play_count(b))
                });

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_album_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.album_count().cmp(&b.album_count()));

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_track_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.track_count().cmp(&b.track_count()));

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_track_duration: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    augmented::artist_track_duration_total(a)
                        .cmp(&augmented::artist_track_duration_total(b))
                });

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id.clone())
                        .collect(),
                )
            },
            by_duration_played: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| {
                    augmented::artist_total_played_duration(a)
                        .cmp(&augmented::artist_total_played_duration(b))
                });

                common::ListAndReversed::new(
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
                common::ListAndReversed::new(artist_ids)
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
    pub by_name: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_artist: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_date: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_last_modified: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_duration: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_total_play_count: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub by_total_played_duration: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
    pub random: common::ListAndReversed<(musiqlibrary::ID, musiqlibrary::ID)>,
}

impl AlbumSorts {
    pub fn new(organized: &augmented::AugmentedLibrary) -> Self {
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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
                    augmented::album_total_play_count(&a.1)
                        .cmp(&augmented::album_total_play_count(b.1))
                });

                common::ListAndReversed::new(
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
                    augmented::album_total_played_duration(&a.1)
                        .cmp(&augmented::album_total_played_duration(b.1))
                });

                common::ListAndReversed::new(
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
                common::ListAndReversed::new(album_ids)
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
    pub by_name: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_album: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_duration: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_total_play_count: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_total_played_duration: common::ListAndReversed<augmented::AugmentedTrack>,
    pub random: common::ListAndReversed<augmented::AugmentedTrack>,
}

impl AlbumTrackSorts {
    pub fn new(artist: &musiqlibrary::KeyedArtistAlbums<augmented::AugmentedTrack>) -> Self {
        AlbumTrackSorts {
            by_name: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.sort_unstable_by(|a, b| {
                    a.metadata
                        .title
                        .to_lowercase()
                        .cmp(&b.metadata.title.to_lowercase())
                });

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
            by_album: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.metadata.date_number.cmp(&b.metadata.date_number));

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
            by_duration: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.metadata.duration.cmp(&b.metadata.duration));

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
            by_total_play_count: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks
                    .sort_unstable_by(|a, b| a.augmented.play_count.cmp(&b.augmented.play_count));

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
            by_total_played_duration: {
                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.sort_unstable_by(|a, b| a.played_seconds().cmp(&b.played_seconds()));

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
            random: {
                let mut rng = rand::thread_rng();

                let mut unpaged_tracks = artist.get_all_tracks();

                unpaged_tracks.shuffle(&mut rng);

                common::ListAndReversed::new(
                    unpaged_tracks.into_iter().map(|a| a.clone()).collect(),
                )
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &common::ArtistTrackSortKey,
        sort_order: &common::SortOrder,
    ) -> &Vec<augmented::AugmentedTrack> {
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
    pub by_name: common::ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_play_count: common::ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_duration: common::ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub by_played_amount: common::ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
    pub random: common::ListAndReversed<musiqlibrary::TrackUniqueIdentifier>,
}

impl TrackSorts {
    pub fn new(organized: &augmented::AugmentedLibrary) -> Self {
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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

                common::ListAndReversed::new(
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
                common::ListAndReversed::new(unpaged_track_ids)
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
