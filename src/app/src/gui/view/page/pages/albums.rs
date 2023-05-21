use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{self, Element, Length};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state::{self, PlayQueueInfo};

use super::super::consts;

use super::super::super::common;
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
                let grid: Element<Message> = columns.into();

                let scrollable = Scrollable::new(grid);
                let first_page = 0;
                let back_page = {
                    if page == 0 {
                        0
                    } else {
                        page - 1
                    }
                };
                let forward_page = {
                    if ((page + 1) * library.grid_info.get_page_size_usize())
                        >= library.get_album_map().keys().len()
                    {
                        page
                    } else {
                        page + 1
                    }
                };
                let last_page = {
                    let maybe_last_page = library.get_album_map().keys().len()
                        / library.grid_info.get_page_size_usize();
                    if maybe_last_page * library.grid_info.get_page_size_usize()
                        >= library.get_album_map().keys().len()
                    {
                        maybe_last_page - 1
                    } else {
                        maybe_last_page
                    }
                };
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Albums"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(dark_button(bright_paragraph("Artist Name")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::ByParent,
                                            model::SortOrder::Regular,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Name")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::ByName,
                                            model::SortOrder::Regular,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Added")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::ByLastMod,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Duration")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::ByDuration,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(
                                        dark_button(bright_paragraph("Total Play Count")).on_press(
                                            user_nav_message(NavMessage::AlbumList(
                                                0,
                                                model::AlbumSortKey::ByTotalPlayCount,
                                                model::SortOrder::Reversed,
                                            )),
                                        ),
                                    )
                                    .push(
                                        dark_button(bright_paragraph("Total Played Duration"))
                                            .on_press(user_nav_message(NavMessage::AlbumList(
                                                0,
                                                model::AlbumSortKey::ByTotalPlayedDuration,
                                                model::SortOrder::Reversed,
                                            ))),
                                    )
                                    .push(dark_button(bright_paragraph("Date")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::ByDate,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Random")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            0,
                                            model::AlbumSortKey::Random,
                                            model::SortOrder::Regular,
                                        )),
                                    )),
                            ),
                        )
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Page: "))
                                    .push(dark_button(bright_paragraph("<<")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            first_page,
                                            sort_key.clone(),
                                            sort_order.clone(),
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("<")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            back_page,
                                            sort_key.clone(),
                                            sort_order.clone(),
                                        )),
                                    ))
                                    .push(bright_paragraph(page.to_string()))
                                    .push(dark_button(bright_paragraph(">")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            forward_page,
                                            sort_key.clone(),
                                            sort_order.clone(),
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph(">>")).on_press(
                                        user_nav_message(NavMessage::AlbumList(
                                            last_page,
                                            sort_key.clone(),
                                            sort_order.clone(),
                                        )),
                                    ))
                                    .push(Space::with_width(Length::Fill))
                                    .push(
                                        line_row()
                                            .push(paragraph("Order: "))
                                            .push(dark_button(bright_paragraph("^")).on_press(
                                                user_nav_message(NavMessage::AlbumList(
                                                    0,
                                                    sort_key.clone(),
                                                    model::SortOrder::Reversed,
                                                )),
                                            ))
                                            .push(dark_button(bright_paragraph("v")).on_press(
                                                user_nav_message(NavMessage::AlbumList(
                                                    0,
                                                    sort_key.clone(),
                                                    model::SortOrder::Regular,
                                                )),
                                            )),
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
