use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{Alignment, Length};

use crate::model;

use crate::gui::message::{self, Message, NavRelMsg, PagifiedMovementMsg};
use crate::state;

use crate::gui::view::components;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn artist_list<'a>(
    library: &'a model::LibraryState,
    play_queue_visible: bool,
    state: &'a state::ArtistListState,
) -> Container<'a, Message> {
    match state {
        state::ArtistListState {
            page,
            sort_key,
            sort_order,
        } => {
            let page: usize = *page;

            let indices = common::get_page(
                library.artist_sorts.from_sort_key(sort_key, sort_order),
                page,
                library.grid_info.get_page_size_usize(),
            );

            let mut paged_artists: Vec<musiqlibrary::ArtistInfo> = Vec::new();
            for index in indices.iter() {
                paged_artists.push(library.get_artist_info(*index));
            }

            let mut buttons: Vec<Button<Message>> = Vec::new();

            for artist in paged_artists.into_iter() {
                let full_artist_info = library.get_artist_albums(&artist.artist_id);

                let relevant_sub_header =
                    get_sub_header_from_sort(&artist, &full_artist_info, &sort_key);

                buttons.push(
                    dark_button(bottom_label(
                        album_image(
                            library.get_artists_first_album_cover(
                                model::AlbumSize::Small,
                                artist.artist_id,
                            ),
                            model::AlbumSize::Small,
                        )
                        .into(),
                        bright_paragraph(common::abr_str(
                            relevant_sub_header,
                            consts::ICON_STR_LENGTH,
                        )),
                    ))
                    .on_press(
                        message::ArtistViewType::ArtistAlbumsView.into_message(artist.artist_id),
                    ),
                );
            }

            let mut columns: Column<Message> = Column::new();
            if play_queue_visible {
                for _i in 0..(library.grid_info.get_layout_height() * 2) {
                    let mut rows = Row::new();
                    for _j in 0..(library.grid_info.get_layout_width() / 2) {
                        if buttons.len() > 0 {
                            let button = buttons.remove(0);
                            rows = rows.push(button);
                        }
                    }
                    columns = columns.push(rows);
                }
            } else {
                for _i in 0..library.grid_info.get_layout_height() {
                    let mut rows = Row::new();
                    for _j in 0..library.grid_info.get_layout_width() {
                        if buttons.len() > 0 {
                            let button = buttons.remove(0);
                            rows = rows.push(button);
                        }
                    }
                    columns = columns.push(rows);
                }
            }

            let page_nav_component = line_row()
                .push(
                    line_row()
                        .push(
                            dark_button(bright_paragraph("<<")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::First),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph("<")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Backwards),
                            )),
                        )
                        .push(bright_paragraph(page.to_string()))
                        .push(
                            dark_button(bright_paragraph(">")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Forwards),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph(">>")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Last),
                            )),
                        ),
                )
                .push(Space::with_width(Length::Fill))
                .push(
                    line_row()
                        .push(paragraph("Order: "))
                        .push(
                            dark_button({
                                if sort_order == &model::SortOrder::Reversed {
                                    bright_paragraph("^")
                                } else {
                                    dark_paragraph("^")
                                }
                            })
                            .on_press(
                                message::ArtistNavMessage::ArtistList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Reversed,
                                )
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button({
                                if sort_order == &model::SortOrder::Regular {
                                    bright_paragraph("v")
                                } else {
                                    dark_paragraph("v")
                                }
                            })
                            .on_press(
                                message::ArtistNavMessage::ArtistList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Regular,
                                )
                                .into_message(),
                            ),
                        ),
                );

            let sort_order_component = line_row().push(
                line_row()
                    .push(paragraph("Sort By: "))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByName.display_text(),
                        model::ArtistSortKey::ByName,
                        model::ArtistSortKey::ByName.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByPlayCount.display_text(),
                        model::ArtistSortKey::ByPlayCount,
                        model::ArtistSortKey::ByPlayCount.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByAlbumCount.display_text(),
                        model::ArtistSortKey::ByAlbumCount,
                        model::ArtistSortKey::ByAlbumCount.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByTrackCount.display_text(),
                        model::ArtistSortKey::ByTrackCount,
                        model::ArtistSortKey::ByTrackCount.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByTrackDuration.display_text(),
                        model::ArtistSortKey::ByTrackDuration,
                        model::ArtistSortKey::ByTrackDuration.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::ByPlayedDuration.display_text(),
                        model::ArtistSortKey::ByPlayedDuration,
                        model::ArtistSortKey::ByPlayedDuration.default_order(),
                        sort_key,
                        message_builder,
                    ))
                    .push(components::sort_button(
                        model::ArtistSortKey::Random.display_text(),
                        model::ArtistSortKey::Random,
                        model::ArtistSortKey::Random.default_order(),
                        sort_key,
                        message_builder,
                    )),
            );

            let scrollable = Scrollable::new(columns.width(Length::Fill)).height(Length::Fill);
            Container::new(
                Column::new()
                    .spacing(10)
                    .push(
                        Column::new()
                            .spacing(10)
                            .push(
                                Row::new()
                                    .align_items(Alignment::End)
                                    .push(h1("Artists"))
                                    .push(Space::with_width(Length::Fixed(20.0)))
                                    .push(page_nav_component),
                            )
                            .push(sort_order_component),
                    )
                    .push(scrollable),
            )
        }
    }
}

fn get_sub_header_from_sort(
    info: &musiqlibrary::ArtistInfo,
    full_artist_info: &musiqlibrary::KeyedArtistAlbums<model::AugmentedTrack>,
    sort_key: &model::ArtistSortKey,
) -> String {
    match sort_key {
        model::ArtistSortKey::ByName => info.artist_name.clone(),
        model::ArtistSortKey::ByPlayCount => format!(
            "{} listens",
            model::artist_total_play_count(&full_artist_info)
        ),
        model::ArtistSortKey::ByAlbumCount => format!("{}", full_artist_info.albums.len(),),
        model::ArtistSortKey::ByTrackCount => format!(
            "{}",
            full_artist_info
                .albums
                .values()
                .fold(0, |total, album| total
                    + album
                        .discs
                        .values()
                        .fold(0, |inner_total, disc| inner_total
                            + disc.tracks.len())),
        ),
        model::ArtistSortKey::ByTrackDuration => {
            common::format_duration(full_artist_info.albums.values().fold(0, |total, album| {
                total + album.album_info.total_duration.as_secs()
            }))
        }
        model::ArtistSortKey::ByPlayedDuration => {
            common::format_duration(model::artist_total_played_duration(&full_artist_info))
        }
        model::ArtistSortKey::Random => info.artist_name.clone(),
    }
}

fn message_builder(sort_key: model::ArtistSortKey, order: model::SortOrder) -> message::Message {
    message::ArtistNavMessage::ArtistList(0, sort_key, order).into_message()
}
