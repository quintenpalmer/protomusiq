use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message, NavRelMsg, PagifiedMovementMsg};
use crate::state::{self, PlayQueueInfo};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn artist_list<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &PlayQueueInfo,
    state: &'a state::ArtistListState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::ArtistListState {
            page,
            sort_key,
            sort_order,
        } => (
            vec![(
                "Artists".to_string(),
                message::ArtistNavMessage::ArtistList(0, sort_key.clone(), sort_order.clone())
                    .into_message(),
            )],
            {
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
                                artist.artist_name.clone(),
                                consts::ICON_STR_LENGTH,
                            )),
                        ))
                        .on_press(
                            message::ArtistNavMessage::ArtistAlbumsView(artist.artist_id)
                                .into_message(),
                        ),
                    );
                }

                let mut columns: Column<Message> = Column::new();
                if play_queue_info.play_queue_visible {
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

                let scrollable = Scrollable::new(columns.width(Length::Fill)).height(Length::Fill);
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Artists"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(sort_button(
                                        "Name",
                                        model::ArtistSortKey::ByName,
                                        model::ArtistSortKey::ByName.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Play Count",
                                        model::ArtistSortKey::ByPlayCount,
                                        model::ArtistSortKey::ByPlayCount.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Album Count",
                                        model::ArtistSortKey::ByAlbumCount,
                                        model::ArtistSortKey::ByAlbumCount.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Track Count",
                                        model::ArtistSortKey::ByTrackCount,
                                        model::ArtistSortKey::ByTrackCount.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Track Length",
                                        model::ArtistSortKey::ByTrackDuration,
                                        model::ArtistSortKey::ByTrackDuration.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Duration Played",
                                        model::ArtistSortKey::ByPlayedDuration,
                                        model::ArtistSortKey::ByPlayedDuration.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Random",
                                        model::ArtistSortKey::Random,
                                        model::ArtistSortKey::Random.default_order(),
                                        sort_key,
                                    )),
                            ),
                        )
                        .push(
                            line_row()
                                .push(
                                    line_row()
                                        .push(dark_button(bright_paragraph("<<")).on_press(
                                            Message::NavRelative(NavRelMsg::PagifiedMovement(
                                                PagifiedMovementMsg::First,
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph("<")).on_press(
                                            Message::NavRelative(NavRelMsg::PagifiedMovement(
                                                PagifiedMovementMsg::Backwards,
                                            )),
                                        ))
                                        .push(bright_paragraph(page.to_string()))
                                        .push(dark_button(bright_paragraph(">")).on_press(
                                            Message::NavRelative(NavRelMsg::PagifiedMovement(
                                                PagifiedMovementMsg::Forwards,
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph(">>")).on_press(
                                            Message::NavRelative(NavRelMsg::PagifiedMovement(
                                                PagifiedMovementMsg::Last,
                                            )),
                                        )),
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
                                ),
                        )
                        .push(scrollable),
                )
            },
        ),
    }
}

fn sort_button<'a>(
    display_text: &'static str,
    sort_key: model::ArtistSortKey,
    order: model::SortOrder,
    current_sort: &'a model::ArtistSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element)
        .on_press(message::ArtistNavMessage::ArtistList(0, sort_key, order).into_message())
}
