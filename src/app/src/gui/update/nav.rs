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
        NavMessage::SearchPage(query, perform_search) => {
            let computed_results = match perform_search {
                true => {
                    let search_results = app.library.search(query.clone());
                    let mapped_search_results = model::SearchResults {
                        artists: search_results
                            .artists
                            .into_iter()
                            .map(|artist| model::Pair {
                                first: artist.first,
                                second: (),
                            })
                            .collect(),
                        albums: search_results
                            .albums
                            .into_iter()
                            .map(|artist_album| model::Pair {
                                first: artist_album.first,
                                second: (),
                            })
                            .collect(),
                        tracks: search_results
                            .tracks
                            .into_iter()
                            .map(|track| model::Pair {
                                first: track.first,
                                second: (),
                            })
                            .collect(),
                        track_artists: search_results
                            .track_artists
                            .into_iter()
                            .map(|track_artist| model::Pair {
                                first: track_artist.first,
                                second: (),
                            })
                            .collect(),
                    };
                    Some(mapped_search_results)
                }
                false => None,
            };

            app.current_page = Page::Search(state::SearchPageState {
                query: query,
                results: computed_results,
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
        NavMessage::ArtistView(artist_id) => {
            app.current_page = Page::ArtistView(state::ArtistViewState {
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
        NavMessage::ArtistAlbumView(artist_id, album_id, album_size, maybe_selected_track) => {
            app.current_page = Page::ArtistAlbumView(state::ArtistAlbumViewState {
                artist_id: artist_id.clone(),
                album_id: album_id.clone(),
                album_size: album_size,
                maybe_selected_track: maybe_selected_track,
            });
            Command::none()
        }
    }
}
