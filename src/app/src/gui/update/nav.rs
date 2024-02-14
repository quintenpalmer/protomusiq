use std::collections::BTreeSet;

use iced::widget::text_input;
use iced::Command;

use crate::model;

use super::super::message::{self, NavMessage};
use super::super::state::{self, AppState, Page};

pub fn handle_nav(
    app: &mut AppState,
    nav_message: message::NavMessage,
) -> Command<message::Message> {
    match nav_message {
        NavMessage::Home => {
            app.current_page = Page::Home(state::HomeState {});
            Command::none()
        }
        NavMessage::Config => {
            app.current_page = Page::Config(state::ConfigState {});
            Command::none()
        }
        NavMessage::PlayQueueFocus => {
            app.current_page = Page::PlayQueue(state::PlayQueueState {});
            Command::none()
        }
        NavMessage::PlaylistView(playlist_id) => {
            app.current_page = Page::PlaylistView(state::PlaylistViewState {
                playlist_id: playlist_id,
            });
            Command::none()
        }
        NavMessage::PlaylistList(new_playlist_name) => {
            app.current_page = Page::PlaylistList(state::PlaylistListState {
                new_playlist_name: new_playlist_name,
            });
            Command::none()
        }
        NavMessage::SearchPage(query, domain, perform_search) => {
            let computed_results = match domain {
                message::SearchDomain::Music => {
                    state::SearchDomainResults::Music(match perform_search {
                        true => {
                            let search_results = app.library.search(query.clone());
                            let mapped_search_results = model::SimpleSearchResults {
                                artists: search_results.artists.into_iter().collect(),
                                albums: search_results.albums.into_iter().collect(),
                                tracks: search_results.tracks.into_iter().collect(),
                                track_artists: search_results.track_artists.into_iter().collect(),
                            };
                            Some(mapped_search_results)
                        }
                        false => None,
                    })
                }
                message::SearchDomain::Movies => {
                    state::SearchDomainResults::Movies(match perform_search {
                        true => {
                            let search_results = app.video_library.search_movies(query.clone());
                            Some(search_results)
                        }
                        false => None,
                    })
                }
            };

            app.current_page = Page::Search(state::SearchPageState {
                query: query,
                domain_results: computed_results,
            });
            text_input::focus(state::TEXT_INPUT_ID.clone())
        }
        NavMessage::TrackList(page, sort, sort_order) => {
            app.current_page = Page::TrackList(state::TrackListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::AlbumList(page, sort, sort_order) => {
            app.current_page = Page::AlbumList(state::AlbumListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistList(page, sort, sort_order) => {
            app.current_page = Page::ArtistList(state::ArtistListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistAlbumsView(artist_id) => {
            app.current_page = Page::ArtistAlbumsView(state::ArtistViewState {
                artist_id: artist_id.clone(),
                albums: app
                    .library
                    .get_artist_map()
                    .get(&artist_id)
                    .unwrap()
                    .albums
                    .keys()
                    .map(|k| k.clone())
                    .collect(),
            });
            Command::none()
        }
        NavMessage::ArtistTrackView(artist_id, sort_key, sort_order) => {
            app.current_page = Page::ArtistTrackView(state::ArtistTrackViewState {
                artist_id: artist_id.clone(),

                sort_key: sort_key,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistFeaturedTrackView(artist_id, sort_key, sort_order) => {
            app.current_page = Page::ArtistFeaturedTrackView(state::ArtistFeaturedTrackViewState {
                artist_id: artist_id.clone(),

                sort_key: sort_key,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistAlbumView(artist_id, album_id, album_size, maybe_selected_track) => {
            app.current_page = Page::ArtistAlbumView(state::ArtistAlbumViewState {
                artist_id: artist_id.clone(),
                album_id: album_id.clone(),
                album_size: album_size,
                maybe_selected_track: maybe_selected_track,
            });
            Command::none()
        }
        NavMessage::MovieList(page, sort, sort_order) => {
            app.current_page = Page::MovieList(state::MovieListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::MovieAttributes(maybe_attr) => {
            let attribute_results = match maybe_attr {
                Some(attr) => match attr {
                    model::MovieAttribute::Genres => {
                        let mut genre_set = BTreeSet::new();
                        for movie in app.video_library.movies.movies.values() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for genre in extra.genres.iter() {
                                        genre_set.insert(genre.clone());
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(model::AttributesList::Genre(
                            genre_set.into_iter().collect(),
                        ))
                    }
                    model::MovieAttribute::Production => {
                        let mut production_set = BTreeSet::new();
                        for movie in app.video_library.movies.movies.values() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for prod in extra.production.iter() {
                                        production_set.insert(prod.clone());
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(model::AttributesList::Production(
                            production_set.into_iter().collect(),
                        ))
                    }
                    model::MovieAttribute::Directors => {
                        let mut director_set = BTreeSet::new();
                        for movie in app.video_library.movies.movies.values() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for prod in extra.directors.iter() {
                                        director_set.insert(prod.clone());
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(model::AttributesList::Director(
                            director_set.into_iter().collect(),
                        ))
                    }
                    model::MovieAttribute::Screenplay => {
                        let mut result_set = BTreeSet::new();
                        for movie in app.video_library.movies.movies.values() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for writer in extra.writers.iter() {
                                        result_set.insert(writer.clone());
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(model::AttributesList::Screenplay(
                            result_set.into_iter().collect(),
                        ))
                    }
                    model::MovieAttribute::CastMembers => {
                        let mut result_set = BTreeSet::new();
                        for movie in app.video_library.movies.movies.values() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for actor in extra.cast.iter() {
                                        result_set.insert(actor.clone());
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(model::AttributesList::CastMember(
                            result_set.into_iter().collect(),
                        ))
                    }
                },
                None => None,
            };

            app.current_page =
                Page::MovieAttributes(state::MovieAttributeState { attribute_results });
            Command::none()
        }
        NavMessage::MovieQuery(ref maybe_query) => {
            let movie_keys = match maybe_query {
                Some(query) => match query {
                    model::MovieQueryParams::Genre(ref queried_genre) => {
                        let mut movie_keys = Vec::new();
                        for (movie_key, movie) in app.video_library.movies.movies.iter() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for found_genre in extra.genres.iter() {
                                        if found_genre == queried_genre {
                                            movie_keys.push(movie_key.clone())
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(movie_keys)
                    }
                    model::MovieQueryParams::Production(ref queried_production_company) => {
                        let mut movie_keys = Vec::new();
                        for (movie_key, movie) in app.video_library.movies.movies.iter() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for found_production_company in extra.production.iter() {
                                        if found_production_company == queried_production_company {
                                            movie_keys.push(movie_key.clone())
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(movie_keys)
                    }
                    model::MovieQueryParams::Director(ref queried_director) => {
                        let mut movie_keys = Vec::new();
                        for (movie_key, movie) in app.video_library.movies.movies.iter() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for found_director in extra.directors.iter() {
                                        if found_director == queried_director {
                                            movie_keys.push(movie_key.clone())
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(movie_keys)
                    }
                    model::MovieQueryParams::Screenplay(ref queried_writer) => {
                        let mut movie_keys = Vec::new();
                        for (movie_key, movie) in app.video_library.movies.movies.iter() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for found_writer in extra.writers.iter() {
                                        if found_writer == queried_writer {
                                            movie_keys.push(movie_key.clone())
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(movie_keys)
                    }
                    model::MovieQueryParams::CastMember(ref queried_actor) => {
                        let mut movie_keys = Vec::new();
                        for (movie_key, movie) in app.video_library.movies.movies.iter() {
                            match movie.extra {
                                Some(ref extra) => {
                                    for found_actor in extra.cast.iter() {
                                        if found_actor == queried_actor {
                                            movie_keys.push(movie_key.clone())
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        Some(movie_keys)
                    }
                },
                None => None,
            };

            app.current_page = Page::MovieQuery(state::MovieQueryState {
                query: maybe_query.clone(),
                matched_keys: movie_keys,
            });
            Command::none()
        }
        NavMessage::MovieView(movie, movie_size) => {
            app.current_page = Page::MovieView(state::MovieViewState { movie, movie_size });
            Command::none()
        }
    }
}
