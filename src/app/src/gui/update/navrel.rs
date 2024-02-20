use iced::Command;

use crate::gui::message::{self};
use crate::gui::state::{self, AppState, Page};
use crate::model;

use super::loaded;

pub fn handle_nav_relative(
    app: &mut AppState,
    parent_nav_message: message::NavRelMsg,
) -> Command<message::Message> {
    let maybe_new_nav_msg = match parent_nav_message {
        message::NavRelMsg::PagifiedMovement(nav_message) => match app.current_page {
            Page::AlbumList(state::AlbumListState {
                ref page,
                ref sort_key,
                ref sort_order,
            }) => {
                let entity_length = app.library.get_album_map().keys().len();

                let new_page = get_rel_page(
                    *page,
                    &nav_message,
                    app.library.grid_info.get_page_size_usize(),
                    entity_length,
                );
                Some(message::Message::Nav(message::NavMessage::AlbumList(
                    new_page,
                    sort_key.clone(),
                    sort_order.clone(),
                )))
            }
            Page::ArtistAlbumView(state::ArtistAlbumViewState {
                artist_id: ref _artist_id,
                album_id: ref _album_id,
                album_size: ref _album_size,
                maybe_selected_track: ref _maybe_selected_track,
                ref maybe_current_sort_order,
            }) => match maybe_current_sort_order {
                Some(model::AlbumSortPlacement {
                    index,
                    sort_key,
                    sort_order,
                }) => {
                    let albums_sorted_by_key =
                        app.library.album_sorts.from_sort_key(sort_key, sort_order);

                    let last_index = albums_sorted_by_key.len() - 1;

                    let new_index = match nav_message {
                        message::PagifiedMovementMsg::First => 0,
                        message::PagifiedMovementMsg::Backwards => {
                            if *index == 0 {
                                0
                            } else {
                                index - 1
                            }
                        }
                        message::PagifiedMovementMsg::Forwards => {
                            if *index == last_index {
                                last_index
                            } else {
                                index + 1
                            }
                        }
                        message::PagifiedMovementMsg::Last => last_index,
                    };

                    let (new_artist_id, new_album_id) =
                        albums_sorted_by_key.get(new_index).unwrap();

                    Some(message::user_nav_message(
                        message::NavMessage::ArtistAlbumView(
                            *new_artist_id,
                            *new_album_id,
                            model::AlbumSize::Regular,
                            None,
                            Some(model::AlbumSortPlacement {
                                index: new_index,
                                sort_key: sort_key.clone(),
                                sort_order: sort_order.clone(),
                            }),
                        ),
                    ))
                }
                None => None,
            },
            Page::TrackList(state::TrackListState {
                ref sort_key,
                ref sort_order,
                ref page,
            }) => {
                let mut total_tracks = 0;
                for (_, artist) in app.library.get_artist_map().iter() {
                    for (_, album) in artist.albums.iter() {
                        for (_, disc) in album.discs.iter() {
                            total_tracks += disc.tracks.len();
                        }
                    }
                }

                let new_page = get_rel_page(
                    *page,
                    &nav_message,
                    app.library.grid_info.get_track_page_size_usize(),
                    total_tracks,
                );

                Some(message::Message::Nav(message::NavMessage::TrackList(
                    new_page,
                    sort_key.clone(),
                    sort_order.clone(),
                )))
            }
            Page::ArtistList(state::ArtistListState {
                ref page,
                ref sort_key,
                ref sort_order,
            }) => {
                let entity_length = app.library.get_artist_map().keys().len();

                let new_page = get_rel_page(
                    *page,
                    &nav_message,
                    app.library.grid_info.get_page_size_usize(),
                    entity_length,
                );

                Some(message::Message::Nav(message::NavMessage::ArtistList(
                    new_page,
                    sort_key.clone(),
                    sort_order.clone(),
                )))
            }
            Page::MovieList(state::MovieListState {
                ref page,
                ref sort_key,
                ref sort_order,
            }) => {
                let entity_length = app.video_library.movies.movies.len();

                let new_page = get_rel_page(
                    *page,
                    &nav_message,
                    app.library.grid_info.get_page_size_usize(),
                    entity_length,
                );

                Some(message::Message::Nav(message::NavMessage::MovieList(
                    new_page,
                    sort_key.clone(),
                    sort_order.clone(),
                )))
            }
            _ => None,
        },
    };

    match maybe_new_nav_msg {
        Some(new_nav_msg) => loaded::update_state(app, new_nav_msg),
        None => Command::none(),
    }
}

fn get_rel_page(
    page: usize,
    nav_message: &message::PagifiedMovementMsg,
    page_size: usize,
    entity_length: usize,
) -> usize {
    match nav_message {
        message::PagifiedMovementMsg::First => 0,
        message::PagifiedMovementMsg::Backwards => {
            if page == 0 {
                0
            } else {
                page - 1
            }
        }
        message::PagifiedMovementMsg::Forwards => {
            if ((page + 1) * page_size) >= entity_length {
                page
            } else {
                page + 1
            }
        }
        message::PagifiedMovementMsg::Last => {
            let maybe_last_page = entity_length / page_size;
            if maybe_last_page * page_size >= entity_length {
                maybe_last_page - 1
            } else {
                maybe_last_page
            }
        }
    }
}
