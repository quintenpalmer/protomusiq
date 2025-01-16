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
            app.page_state.current_page = Page::Home(state::HomeState {});
            Command::none()
        }
        NavMessage::Config => {
            app.page_state.current_page = Page::Config(state::ConfigState {});
            Command::none()
        }
        NavMessage::PlayQueueFocus => {
            app.page_state.current_page = Page::PlayQueue(state::PlayQueueState {});
            Command::none()
        }
        NavMessage::Playlist(message::PlaylistNavMessage::PlaylistView(playlist_id)) => {
            app.page_state.current_page =
                Page::PlaylistView(state::PlaylistViewState { playlist_id });
            Command::none()
        }
        NavMessage::Playlist(message::PlaylistNavMessage::PlaylistList(new_playlist_name)) => {
            app.page_state.current_page =
                Page::PlaylistList(state::PlaylistListState { new_playlist_name });
            Command::none()
        }
        NavMessage::SearchPage(query, domain, perform_search) => {
            let computed_results = match domain {
                model::SearchDomain::Music => {
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
                model::SearchDomain::Movies => {
                    state::SearchDomainResults::Movies(match perform_search {
                        true => {
                            let search_results = app.video_library.search_movies(query.clone());
                            Some(search_results)
                        }
                        false => None,
                    })
                }
            };

            app.page_state.current_page = Page::Search(state::SearchPageState {
                query,
                domain_results: computed_results,
            });
            text_input::focus(state::TEXT_INPUT_ID.clone())
        }
        NavMessage::Music(music_message) => match music_message {
            message::MusicNavMessage::MusicHome => {
                app.page_state.current_page = Page::MusicHome;
                Command::none()
            }
            message::MusicNavMessage::Genres(message::MusicGenreNavMessage::Home) => {
                app.page_state.current_page = Page::GenreHome;
                Command::none()
            }
            message::MusicNavMessage::TrackList(page, sort, sort_order) => {
                app.page_state.current_page = Page::TrackList(state::TrackListState {
                    page,
                    sort_key: sort,
                    sort_order,
                });
                Command::none()
            }
            message::MusicNavMessage::AlbumList(page, sort, sort_order) => {
                app.page_state.current_page = Page::AlbumList(state::AlbumListState {
                    page,
                    sort_key: sort,
                    sort_order,
                });
                Command::none()
            }
            message::MusicNavMessage::Artist(artist_message) => match artist_message {
                message::ArtistNavMessage::ArtistList(page, sort, sort_order) => {
                    app.page_state.current_page = Page::ArtistList(state::ArtistListState {
                        page,
                        sort_key: sort,
                        sort_order,
                    });
                    Command::none()
                }
                message::ArtistNavMessage::ArtistView(artist_id, type_) => match type_ {
                    message::ArtistViewType::ArtistAlbumsView => {
                        app.page_state.current_page =
                            Page::ArtistAlbumsView(state::ArtistViewState {
                                artist_id,
                                albums: app
                                    .library
                                    .get_artist_map()
                                    .get(&artist_id)
                                    .unwrap()
                                    .albums
                                    .keys()
                                    .cloned()
                                    .collect(),
                            });
                        Command::none()
                    }
                    message::ArtistViewType::ArtistInfo => {
                        app.page_state.current_page =
                            Page::ArtistInfoView(state::ArtistInfoState { artist_id });
                        Command::none()
                    }
                    message::ArtistViewType::ArtistTrackView(sort_key, sort_order) => {
                        app.page_state.current_page =
                            Page::ArtistTrackView(state::ArtistTrackViewState {
                                artist_id,

                                sort_key,
                                sort_order,
                            });
                        Command::none()
                    }
                    message::ArtistViewType::ArtistFeaturedTrackView(sort_key, sort_order) => {
                        app.page_state.current_page =
                            Page::ArtistFeaturedTrackView(state::ArtistFeaturedTrackViewState {
                                artist_id,

                                sort_key,
                                sort_order,
                            });
                        Command::none()
                    }
                    message::ArtistViewType::InPlaylist => {
                        let mut playlist_ids = BTreeSet::new();

                        for playlist in app.library.user_playlists.entries_as_vec() {
                            for track in playlist.tracks.iter() {
                                if artist_id == track.artist_id {
                                    playlist_ids.insert(playlist.id);
                                }
                            }
                        }

                        app.page_state.current_page =
                            Page::ArtistFeaturedInPlaylist(state::ArtistFeaturedInPlaylistState {
                                artist_id,

                                playlist_ids,
                            });
                        Command::none()
                    }
                },
                message::ArtistNavMessage::AlbumView(
                    artist_id,
                    album_id,
                    message::ArtistAlbumView::ArtistAlbumTrackView(
                        album_size,
                        maybe_selected_track,
                        maybe_current_sort_order,
                    ),
                ) => {
                    app.page_state.current_page =
                        Page::ArtistAlbumView(state::ArtistAlbumViewState {
                            artist_id,
                            album_id,
                            album_size,
                            maybe_selected_track,
                            maybe_current_sort_order,
                        });
                    Command::none()
                }
                message::ArtistNavMessage::AlbumView(
                    artist_id,
                    album_id,
                    message::ArtistAlbumView::InPlaylist,
                ) => {
                    let mut playlist_ids = BTreeSet::new();

                    for playlist in app.library.user_playlists.entries_as_vec() {
                        for track in playlist.tracks.iter() {
                            if artist_id == track.artist_id && album_id == track.album_id {
                                playlist_ids.insert(playlist.id);
                            }
                        }
                    }

                    app.page_state.current_page = Page::ArtistAlbumFeaturedInPlaylist(
                        state::ArtistAlbumFeaturedInPlaylistState {
                            artist_id,
                            album_id,
                            playlist_ids,
                        },
                    );
                    Command::none()
                }
            },
        },
        NavMessage::Movie(movie_message) => match movie_message {
            message::MovieNavMessage::MovieHome => {
                app.page_state.current_page = Page::MovieHome;
                Command::none()
            }
            message::MovieNavMessage::MovieList(page, sort, sort_order) => {
                app.page_state.current_page = Page::MovieList(state::MovieListState {
                    page,
                    sort_key: sort,
                    sort_order,
                });
                Command::none()
            }
            message::MovieNavMessage::MovieAttributes(maybe_attr) => {
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
                        model::MovieAttribute::Producers => {
                            let mut producer_set = BTreeSet::new();
                            for movie in app.video_library.movies.movies.values() {
                                match movie.extra {
                                    Some(ref extra) => {
                                        for prod in extra.producers.iter() {
                                            producer_set.insert(prod.clone());
                                        }
                                    }
                                    None => (),
                                }
                            }
                            Some(model::AttributesList::Producers(
                                producer_set.into_iter().collect(),
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

                app.page_state.current_page =
                    Page::MovieAttributes(state::MovieAttributeState { attribute_results });
                Command::none()
            }
            message::MovieNavMessage::MovieQuery(ref maybe_query) => {
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
                                            if found_production_company
                                                == queried_production_company
                                            {
                                                movie_keys.push(movie_key.clone())
                                            }
                                        }
                                    }
                                    None => (),
                                }
                            }
                            Some(movie_keys)
                        }
                        model::MovieQueryParams::Producers(ref queried_producers) => {
                            let mut movie_keys = Vec::new();
                            for (movie_key, movie) in app.video_library.movies.movies.iter() {
                                match movie.extra {
                                    Some(ref extra) => {
                                        for found_producer in extra.producers.iter() {
                                            if found_producer == queried_producers {
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

                app.page_state.current_page = Page::MovieQuery(state::MovieQueryState {
                    query: maybe_query.clone(),
                    matched_keys: movie_keys,
                });
                Command::none()
            }
            message::MovieNavMessage::MovieView(movie, movie_size, maybe_current_sort_order) => {
                app.page_state.current_page = Page::MovieView(state::MovieViewState {
                    movie,
                    movie_size,
                    maybe_current_sort_order,
                });
                Command::none()
            }
            message::MovieNavMessage::SeriesList => {
                app.page_state.current_page = Page::MovieSeriesList;
                Command::none()
            }
        },
        NavMessage::Game(game_message) => match game_message {
            message::GameNavMessage::GameHome => {
                app.page_state.current_page = Page::GameHome;
                Command::none()
            }
            message::GameNavMessage::GBList => {
                app.page_state.current_page = Page::GBList;
                Command::none()
            }
            message::GameNavMessage::GBCList => {
                app.page_state.current_page = Page::GBCList;
                Command::none()
            }
            message::GameNavMessage::GBAList => {
                app.page_state.current_page = Page::GBAList;
                Command::none()
            }
            message::GameNavMessage::SNESList => {
                app.page_state.current_page = Page::SNESList;
                Command::none()
            }
            message::GameNavMessage::N64List => {
                app.page_state.current_page = Page::N64List;
                Command::none()
            }
            message::GameNavMessage::NDSList => {
                app.page_state.current_page = Page::NDSList;
                Command::none()
            }
            message::GameNavMessage::GameCubeList => {
                app.page_state.current_page = Page::GameCubeList;
                Command::none()
            }
            message::GameNavMessage::WiiList => {
                app.page_state.current_page = Page::WiiList;
                Command::none()
            }
        },
        NavMessage::Shows(show_message) => match show_message {
            message::ShowNavMessage::Home => {
                app.page_state.current_page = Page::ShowHome;
                Command::none()
            }
            message::ShowNavMessage::ShowList => {
                app.page_state.current_page = Page::ShowList;
                Command::none()
            }
            message::ShowNavMessage::ShowSeries(series_key) => {
                app.page_state.current_page = Page::ShowSeriesView(series_key.clone());
                Command::none()
            }
            message::ShowNavMessage::ShowSeason(series_key, season_id) => {
                app.page_state.current_page = Page::ShowSeasonView(series_key.clone(), season_id);
                Command::none()
            }
            message::ShowNavMessage::ContinueWatching => {
                app.page_state.current_page = Page::ShowContinueWatching;
                Command::none()
            }
        },
    }
}
