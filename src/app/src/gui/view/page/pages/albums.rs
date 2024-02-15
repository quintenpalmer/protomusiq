use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{self, Length};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage, NavRelMsg, PagifiedMovementMsg};
use crate::state::{self, PlayQueueInfo};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn album_list<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &PlayQueueInfo,
    state: &'a state::AlbumListState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::AlbumListState {
            page,
            sort_key,
            sort_order,
        } => {
            let breadcrumbs = vec![(
                "Albums".to_string(),
                user_nav_message(NavMessage::AlbumList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )];

            let body = {
                let page: usize = page.clone();

                let indices = common::get_page(
                    library.album_sorts.from_sort_key(&sort_key, &sort_order),
                    page,
                    library.grid_info.get_page_size_usize(),
                );

                let mut paged_albums: Vec<musiqlibrary::ArtistAlbumInfo> = Vec::new();
                for index in indices.iter() {
                    paged_albums
                        .push(library.get_artist_album_info(index.0.clone(), index.1.clone()));
                }

                let mut buttons: Vec<Button<Message>> = Vec::new();

                for info in paged_albums.into_iter() {
                    buttons.push(
                        dark_button(bottom_label(
                            album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Small,
                                    info.artist.artist_id.clone(),
                                    info.album.album_id.clone(),
                                ),
                                model::AlbumSize::Small,
                            )
                            .into(),
                            Column::new()
                                .align_items(iced::Alignment::Center)
                                .push(bright_paragraph(common::abr_str(
                                    info.album.album_name.clone(),
                                    consts::ICON_STR_LENGTH,
                                )))
                                .push(paragraph(common::abr_str(
                                    info.artist.artist_name.clone(),
                                    consts::ICON_STR_LENGTH,
                                ))),
                        ))
                        .on_press(user_nav_message(
                            NavMessage::ArtistAlbumView(
                                info.artist.artist_id.clone(),
                                info.album.album_id.clone(),
                                model::AlbumSize::Regular,
                                None,
                            ),
                        )),
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

                let scrollable = Scrollable::new(columns.width(Length::Fill));
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Albums"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(sort_button(
                                        "Artist Name",
                                        model::AlbumSortKey::ByParent,
                                        model::SortOrder::Regular,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Name",
                                        model::AlbumSortKey::ByName,
                                        model::SortOrder::Regular,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Added",
                                        model::AlbumSortKey::ByLastMod,
                                        model::SortOrder::Reversed,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Length",
                                        model::AlbumSortKey::ByDuration,
                                        model::SortOrder::Reversed,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Total Play Count",
                                        model::AlbumSortKey::ByTotalPlayCount,
                                        model::SortOrder::Reversed,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Total Played Duration",
                                        model::AlbumSortKey::ByTotalPlayedDuration,
                                        model::SortOrder::Reversed,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Date",
                                        model::AlbumSortKey::ByDate,
                                        model::SortOrder::Reversed,
                                        &sort_key,
                                    ))
                                    .push(sort_button(
                                        "Random",
                                        model::AlbumSortKey::Random,
                                        model::SortOrder::Regular,
                                        &sort_key,
                                    )),
                            ),
                        )
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Page: "))
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
                                    ))
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
                                                .on_press(user_nav_message(NavMessage::AlbumList(
                                                    0,
                                                    sort_key.clone(),
                                                    model::SortOrder::Reversed,
                                                ))),
                                            )
                                            .push(
                                                dark_button({
                                                    if sort_order == &model::SortOrder::Regular {
                                                        bright_paragraph("v")
                                                    } else {
                                                        dark_paragraph("v")
                                                    }
                                                })
                                                .on_press(user_nav_message(NavMessage::AlbumList(
                                                    0,
                                                    sort_key.clone(),
                                                    model::SortOrder::Regular,
                                                ))),
                                            ),
                                    ),
                            ),
                        )
                        .push(scrollable),
                )
            };

            (breadcrumbs, body)
        }
    }
}

fn sort_button<'a>(
    display_text: &'static str,
    sort_key: model::AlbumSortKey,
    order: model::SortOrder,
    current_sort_key: &'a model::AlbumSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort_key {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element).on_press(user_nav_message(NavMessage::AlbumList(0, sort_key, order)))
}
