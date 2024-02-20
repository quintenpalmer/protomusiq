use rand::seq::SliceRandom;

use musiqlibrary::video;

use super::{augmented, common, sortkeys};

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
                        .map(|a| a.artist_info.artist_id)
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
                        .map(|a| a.artist_info.artist_id)
                        .collect(),
                )
            },
            by_album_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.album_count().cmp(&b.album_count()));

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id)
                        .collect(),
                )
            },
            by_track_count: {
                let mut unpaged_artists = organized.artists.values().collect::<Vec<_>>();

                unpaged_artists.sort_unstable_by(|a, b| a.track_count().cmp(&b.track_count()));

                common::ListAndReversed::new(
                    unpaged_artists
                        .iter()
                        .map(|a| a.artist_info.artist_id)
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
                        .map(|a| a.artist_info.artist_id)
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
                        .map(|a| a.artist_info.artist_id)
                        .collect(),
                )
            },
            random: {
                let mut rng = rand::thread_rng();
                let mut artist_ids: Vec<musiqlibrary::ID> = organized
                    .artists
                    .values()
                    .map(|x| x.artist_info.artist_id)
                    .collect();
                artist_ids.shuffle(&mut rng);
                common::ListAndReversed::new(artist_ids)
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &sortkeys::ArtistSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<musiqlibrary::ID> {
        match sort_key {
            sortkeys::ArtistSortKey::ByName => &self.by_name,
            sortkeys::ArtistSortKey::ByPlayCount => &self.by_play_count,
            sortkeys::ArtistSortKey::ByAlbumCount => &self.by_album_count,
            sortkeys::ArtistSortKey::ByTrackCount => &self.by_track_count,
            sortkeys::ArtistSortKey::ByTrackDuration => &self.by_track_duration,
            sortkeys::ArtistSortKey::ByPlayedDuration => &self.by_duration_played,
            sortkeys::ArtistSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
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
                        .map(|a| (a.0.artist_id, a.1.album_id))
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
                        .map(|a| (a.0.artist_id, a.1.album_id))
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
                        .map(|a| (a.0.artist_id, a.1.album_id))
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
                        .map(|a| (a.0.artist_id, a.1.album_id))
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
                        .map(|a| (a.0.artist_id, a.1.album_info.album_id))
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
                    augmented::album_total_play_count(a.1)
                        .cmp(&augmented::album_total_play_count(b.1))
                });

                common::ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id, a.1.album_info.album_id))
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
                    augmented::album_total_played_duration(a.1)
                        .cmp(&augmented::album_total_played_duration(b.1))
                });

                common::ListAndReversed::new(
                    unpaged_albums
                        .iter()
                        .map(|a| (a.0.artist_id, a.1.album_info.album_id))
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
                                .map(|x| (artist.artist_info.artist_id, x.album_info.album_id))
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
        sort_key: &sortkeys::AlbumSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<(musiqlibrary::ID, musiqlibrary::ID)> {
        match sort_key {
            sortkeys::AlbumSortKey::ByName => &self.by_name,
            sortkeys::AlbumSortKey::ByParent => &self.by_artist,
            sortkeys::AlbumSortKey::ByDate => &self.by_date,
            sortkeys::AlbumSortKey::ByDuration => &self.by_duration,
            sortkeys::AlbumSortKey::ByLastMod => &self.by_last_modified,
            sortkeys::AlbumSortKey::ByTotalPlayCount => &self.by_total_play_count,
            sortkeys::AlbumSortKey::ByTotalPlayedDuration => &self.by_total_played_duration,
            sortkeys::AlbumSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
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
    pub fn new(all_tracks: Vec<&augmented::AugmentedTrack>) -> Self {
        AlbumTrackSorts {
            by_name: {
                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| {
                    a.metadata
                        .title
                        .to_lowercase()
                        .cmp(&b.metadata.title.to_lowercase())
                });

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
            by_album: {
                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.metadata.album.cmp(&b.metadata.album));

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
            by_duration: {
                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.metadata.duration.cmp(&b.metadata.duration));

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
            by_total_play_count: {
                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.augmented.play_count.cmp(&b.augmented.play_count));

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
            by_total_played_duration: {
                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.played_seconds().cmp(&b.played_seconds()));

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
            random: {
                let mut rng = rand::thread_rng();

                let mut unpaged_tracks = all_tracks.clone();

                unpaged_tracks.shuffle(&mut rng);

                common::ListAndReversed::new(unpaged_tracks.into_iter().cloned().collect())
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &sortkeys::ArtistTrackSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<augmented::AugmentedTrack> {
        match sort_key {
            sortkeys::ArtistTrackSortKey::ByName => &self.by_name,
            sortkeys::ArtistTrackSortKey::ByParent => &self.by_album,
            sortkeys::ArtistTrackSortKey::ByDuration => &self.by_duration,
            sortkeys::ArtistTrackSortKey::ByTotalPlayCount => &self.by_total_play_count,
            sortkeys::ArtistTrackSortKey::ByTotalPlayedDuration => &self.by_total_played_duration,
            sortkeys::ArtistTrackSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
    }
}

pub struct AlbumFeaturedTrackSorts {
    pub by_name: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_album: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_duration: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_total_play_count: common::ListAndReversed<augmented::AugmentedTrack>,
    pub by_total_played_duration: common::ListAndReversed<augmented::AugmentedTrack>,
    pub random: common::ListAndReversed<augmented::AugmentedTrack>,
}

impl AlbumFeaturedTrackSorts {
    pub fn new(featured_artist_tracks: Vec<augmented::AugmentedTrack>) -> Self {
        AlbumFeaturedTrackSorts {
            by_name: {
                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| {
                    a.metadata
                        .title
                        .to_lowercase()
                        .cmp(&b.metadata.title.to_lowercase())
                });

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
            by_album: {
                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.metadata.album.cmp(&b.metadata.album));

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
            by_duration: {
                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.metadata.duration.cmp(&b.metadata.duration));

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
            by_total_play_count: {
                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.augmented.play_count.cmp(&b.augmented.play_count));

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
            by_total_played_duration: {
                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.sort_unstable_by(|a, b| a.metadata.track.cmp(&b.metadata.track));

                unpaged_tracks.sort_by(|a, b| a.played_seconds().cmp(&b.played_seconds()));

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
            random: {
                let mut rng = rand::thread_rng();

                let mut unpaged_tracks = featured_artist_tracks.clone();

                unpaged_tracks.shuffle(&mut rng);

                common::ListAndReversed::new(unpaged_tracks.into_iter().collect())
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &sortkeys::ArtistFeaturedTrackSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<augmented::AugmentedTrack> {
        match sort_key {
            sortkeys::ArtistFeaturedTrackSortKey::ByName => &self.by_name,
            sortkeys::ArtistFeaturedTrackSortKey::ByParent => &self.by_album,
            sortkeys::ArtistFeaturedTrackSortKey::ByDuration => &self.by_duration,
            sortkeys::ArtistFeaturedTrackSortKey::ByTotalPlayCount => &self.by_total_play_count,
            sortkeys::ArtistFeaturedTrackSortKey::ByTotalPlayedDuration => {
                &self.by_total_played_duration
            }
            sortkeys::ArtistFeaturedTrackSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
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
                        .map(|a| a.metadata.to_unique_id())
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
        sort_key: &sortkeys::TrackSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<musiqlibrary::TrackUniqueIdentifier> {
        match sort_key {
            sortkeys::TrackSortKey::ByName => &self.by_name,
            sortkeys::TrackSortKey::ByPlayCount => &self.by_play_count,
            sortkeys::TrackSortKey::ByDuration => &self.by_duration,
            sortkeys::TrackSortKey::ByPlayedAmount => &self.by_played_amount,
            sortkeys::TrackSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
    }
}

pub struct MovieSorts {
    pub by_title: common::ListAndReversed<video::MovieMetadata>,
    pub by_last_modified: common::ListAndReversed<video::MovieMetadata>,
    pub by_duration: common::ListAndReversed<video::MovieMetadata>,
    pub by_release: common::ListAndReversed<video::MovieMetadata>,
    pub random: common::ListAndReversed<video::MovieMetadata>,
}

impl MovieSorts {
    pub fn new(movies: &Vec<video::MovieMetadata>) -> Self {
        MovieSorts {
            by_title: {
                let mut movies_by_title = movies.clone();

                movies_by_title
                    .sort_unstable_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

                common::ListAndReversed::new(movies_by_title.to_vec())
            },

            by_last_modified: {
                let mut movies_by_last_mod = movies.clone();

                movies_by_last_mod.sort_unstable_by(|a, b| a.last_modified.cmp(&b.last_modified));

                common::ListAndReversed::new(movies_by_last_mod.to_vec())
            },

            by_duration: {
                let mut movies_by_duration = movies.clone();

                movies_by_duration.sort_unstable_by(|a, b| a.duration.cmp(&b.duration));

                common::ListAndReversed::new(movies_by_duration.to_vec())
            },

            by_release: {
                let mut movies_by_release = movies.clone();

                movies_by_release.sort_unstable_by(|a, b| {
                    a.extra
                        .as_ref()
                        .map(|x| x.release)
                        .cmp(&b.extra.as_ref().map(|x| x.release))
                });

                common::ListAndReversed::new(movies_by_release.to_vec())
            },

            random: {
                let mut rng = rand::thread_rng();

                let mut movies_by_title = movies.clone();

                movies_by_title.shuffle(&mut rng);
                common::ListAndReversed::new(movies_by_title)
            },
        }
    }

    pub fn from_sort_key(
        &self,
        sort_key: &sortkeys::MovieSortKey,
        sort_order: &sortkeys::SortOrder,
    ) -> &Vec<video::MovieMetadata> {
        match sort_key {
            sortkeys::MovieSortKey::ByTitle => &self.by_title,
            sortkeys::MovieSortKey::LastModified => &self.by_last_modified,
            sortkeys::MovieSortKey::ByDuration => &self.by_duration,
            sortkeys::MovieSortKey::ByRelease => &self.by_release,
            sortkeys::MovieSortKey::Random => &self.random,
        }
        .sort_ordered(sort_order)
    }
}
