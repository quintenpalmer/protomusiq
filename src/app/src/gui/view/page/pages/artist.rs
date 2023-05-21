use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{Element, Length};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state::{self, PlayQueueInfoState};

use super::super::consts;

use super::super::super::common;
use super::super::super::elements::*;

pub fn artist_list<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &PlayQueueInfoState,
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
                user_nav_message(NavMessage::ArtistList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )],
            {
                let page: usize = page.clone();

                let indices = common::get_page(
                    library.artist_sorts.from_sort_key(&sort_key, &sort_order),
                    page,
                    library.grid_info.get_page_size_usize(),
                );

                let mut paged_artists: Vec<musiqlibrary::ArtistInfo> = Vec::new();
                for index in indices.iter() {
                    paged_artists.push(library.get_artist_info(index.clone()));
                }

                let mut buttons: Vec<Button<Message>> = Vec::new();

                for artist in paged_artists.into_iter() {
                    buttons.push(
                        dark_button(bottom_label(
                            album_image(
                                library.get_artists_first_album_cover(
                                    model::AlbumSize::Small,
                                    artist.artist_id.clone(),
                                ),
                                model::AlbumSize::Small,
                            )
                            .into(),
                            bright_paragraph(common::abr_str(
                                artist.artist_name.clone(),
                                consts::ICON_STR_LENGTH,
                            )),
                        ))
                        .on_press(user_nav_message(
                            NavMessage::ArtistView(artist.artist_id.clone()),
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
                        >= library.get_artist_map().keys().len()
                    {
                        page
                    } else {
                        page + 1
                    }
                };
                let last_page = {
                    let maybe_last_page = library.get_artist_map().keys().len()
                        / library.grid_info.get_page_size_usize();
                    if maybe_last_page * library.grid_info.get_page_size_usize()
                        >= library.get_artist_map().keys().len()
                    {
                        maybe_last_page - 1
                    } else {
                        maybe_last_page
                    }
                };
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Artists"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(dark_button(bright_paragraph("Name")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::ByName,
                                            model::SortOrder::Regular,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Random")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::Random,
                                            model::SortOrder::Regular,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Play Count")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::ByPlayCount,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Album Count")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::ByAlbumCount,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Track Count")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::ByTrackCount,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Track Duration")).on_press(
                                        user_nav_message(NavMessage::ArtistList(
                                            0,
                                            model::ArtistSortKey::ByTrackDuration,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(
                                        dark_button(bright_paragraph("Duration Played")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                0,
                                                model::ArtistSortKey::ByPlayedDuration,
                                                model::SortOrder::Reversed,
                                            )),
                                        ),
                                    ),
                            ),
                        )
                        .push(
                            line_row()
                                .push(
                                    line_row()
                                        .push(dark_button(bright_paragraph("<<")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                first_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph("<")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                back_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(bright_paragraph(page.to_string()))
                                        .push(dark_button(bright_paragraph(">")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                forward_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph(">>")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                last_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        )),
                                )
                                .push(Space::with_width(Length::Fill))
                                .push(
                                    line_row()
                                        .push(paragraph("Order: "))
                                        .push(dark_button(bright_paragraph("^")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                0,
                                                sort_key.clone(),
                                                model::SortOrder::Reversed,
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph("v")).on_press(
                                            user_nav_message(NavMessage::ArtistList(
                                                0,
                                                sort_key.clone(),
                                                model::SortOrder::Regular,
                                            )),
                                        )),
                                ),
                        )
                        .push(scrollable),
                )
            },
        ),
    }
}
