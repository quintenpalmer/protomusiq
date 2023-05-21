use iced::widget::{Button, Column, Container, Row, Scrollable};

use musiqlibrary;

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state::{self, PlayQueueInfo};

use super::super::super::common;
use super::super::super::elements::*;

use super::super::consts;

pub fn artist_album_list<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &PlayQueueInfo,
    state: &'a state::ArtistViewState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::ArtistViewState { artist_id, albums } => {
            let artist = library.get_artist_map().get(&artist_id).unwrap();

            let breadcrumbs = vec![
                (
                    "Artists".to_string(),
                    user_nav_message(NavMessage::ArtistList(
                        0,
                        model::ArtistSortKey::ByName,
                        model::SortOrder::Regular,
                    )),
                ),
                (
                    artist.artist_info.artist_name.clone(),
                    user_nav_message(NavMessage::ArtistView(artist_id.clone())),
                ),
            ];
            let body = {
                let mut buttons: Vec<(musiqlibrary::AlbumInfo, Button<Message>)> = Vec::new();
                for album_id in albums.iter() {
                    let album = library
                        .get_artist_map()
                        .get(&artist_id)
                        .unwrap()
                        .albums
                        .get(&album_id)
                        .unwrap();
                    buttons.push((
                        album.album_info.clone(),
                        dark_button(bottom_label(
                            album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Small,
                                    artist.artist_info.artist_id.clone(),
                                    album.album_info.album_id.clone(),
                                ),
                                model::AlbumSize::Small,
                            )
                            .into(),
                            Column::new()
                                .align_items(iced::Alignment::Center)
                                .push(bright_paragraph(common::abr_str(
                                    album.album_info.album_name.clone(),
                                    consts::ICON_STR_LENGTH,
                                )))
                                .push(paragraph(common::format_date_range(
                                    album.album_info.start_date,
                                    album.album_info.end_date,
                                ))),
                        ))
                        .on_press(user_nav_message(
                            NavMessage::ArtistAlbumView(
                                artist.artist_info.artist_id.clone(),
                                album.album_info.album_id.clone(),
                                model::AlbumSize::Regular,
                                None,
                            ),
                        )),
                    ))
                }

                buttons.sort_unstable_by(|(a, _), (b, _)| b.start_date.cmp(&a.start_date));
                /* Name Sort
                buttons.sort_unstable_by(|(a, _), (b, _)| {
                    a.album_name
                        .to_lowercase()
                        .cmp(&b.album_name.to_lowercase())
                });
                */

                let mut album_grid_columns: Column<Message> = Column::new();
                let mut album_grid_rows = Row::new();
                let mut row_length = 0;
                loop {
                    let desired_length = if play_queue_info.play_queue_visible {
                        library.grid_info.get_layout_width() / 2
                    } else {
                        library.grid_info.get_layout_width()
                    };
                    if row_length == desired_length {
                        album_grid_columns = album_grid_columns.push(album_grid_rows);
                        row_length = 0;
                        album_grid_rows = Row::new();
                    } else {
                        if buttons.len() > 0 {
                            row_length += 1;
                            let (_, button) = buttons.remove(0);
                            album_grid_rows = album_grid_rows.push(button);
                        } else {
                            album_grid_columns = album_grid_columns.push(album_grid_rows);
                            break;
                        }
                    }
                }

                let scrollable = Scrollable::new(album_grid_columns);

                let artist_view_button_row = line_row()
                    .push(
                        dark_button(h2("Albums"))
                            .on_press(user_nav_message(NavMessage::ArtistView(artist_id.clone()))),
                    )
                    .push(dark_button(h2("Tracks")).on_press(user_nav_message(
                        NavMessage::ArtistTrackView(
                            artist_id.clone(),
                            model::ArtistTrackSortKey::ByTotalPlayCount,
                            model::SortOrder::Reversed,
                        ),
                    )));

                Container::new(
                    Column::new()
                        .push(h1(artist.artist_info.artist_name.clone()))
                        .push(artist_view_button_row)
                        .push(scrollable),
                )
            };

            (breadcrumbs, body)
        }
    }
}
