use iced::Command;

use crate::gui::message::{self};
use crate::gui::state::{self, AppState, Page};

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
                    page.clone(),
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
                    page.clone(),
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
                    page.clone(),
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
                    page.clone(),
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
