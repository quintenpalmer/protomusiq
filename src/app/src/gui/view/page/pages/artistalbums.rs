use iced::widget::{Button, Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};
use crate::state;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn artist_album_list<'a>(
    library: &'a model::LibraryState,
    play_queue_visible: bool,
    state: &'a state::ArtistViewState,
) -> Container<'a, Message> {
    match state {
        state::ArtistViewState { artist_id, albums } => {
            let artist = library.get_artist_info(*artist_id);

            let body = {
                let mut buttons: Vec<(musiqlibrary::AlbumInfo, Button<Message>)> = Vec::new();
                for album_id in albums.iter() {
                    let album = library
                        .get_artist_map()
                        .get(artist_id)
                        .unwrap()
                        .albums
                        .get(album_id)
                        .unwrap();
                    buttons.push((
                        album.album_info.clone(),
                        dark_button(bottom_label(
                            album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Small,
                                    artist.artist_id,
                                    album.album_info.album_id,
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
                        .on_press(
                            message::ArtistNavMessage::AlbumView(
                                artist.artist_id,
                                album.album_info.album_id,
                                message::ArtistAlbumView::ArtistAlbumTrackView(
                                    model::AlbumSize::Regular,
                                    None,
                                    None,
                                ),
                            )
                            .into_message(),
                        ),
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
                    let desired_length = if play_queue_visible {
                        library.grid_info.get_layout_width() / 2
                    } else {
                        library.grid_info.get_layout_width()
                    };
                    if row_length == desired_length {
                        album_grid_columns = album_grid_columns.push(album_grid_rows);
                        row_length = 0;
                        album_grid_rows = Row::new();
                    } else if buttons.len() > 0 {
                        row_length += 1;
                        let (_, button) = buttons.remove(0);
                        album_grid_rows = album_grid_rows.push(button);
                    } else {
                        album_grid_columns = album_grid_columns.push(album_grid_rows);
                        break;
                    }
                }

                let scrollable =
                    Scrollable::new(album_grid_columns.width(Length::Fill)).height(Length::Fill);

                let artist_view_button_row = line_row()
                    .push(dark_button(h2("Albums")).on_press(
                        message::ArtistViewType::ArtistAlbumsView.into_message(*artist_id),
                    ))
                    .push(
                        dark_button(dark(h2("Tracks"))).on_press(
                            message::ArtistViewType::ArtistTrackView(
                                model::ArtistTrackSortKey::ByTotalPlayCount,
                                model::ArtistTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(dark(h2("Featured"))).on_press(
                            message::ArtistViewType::ArtistFeaturedTrackView(
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(dark(h2("In Playlists")))
                            .on_press(message::ArtistViewType::InPlaylist.into_message(*artist_id)),
                    )
                    .push(
                        dark_button(dark(h2("Info")))
                            .on_press(message::ArtistViewType::ArtistInfo.into_message(*artist_id)),
                    );

                Container::new(
                    Column::new()
                        .push(h1(artist.artist_name.clone()))
                        .push(artist_view_button_row)
                        .push(scrollable),
                )
            };

            body
        }
    }
}
