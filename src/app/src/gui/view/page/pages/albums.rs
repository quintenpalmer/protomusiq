use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{Alignment, Length};

use crate::model;

use crate::gui::message::{
    self, user_nav_message, Message, NavMessage, NavRelMsg, PagifiedMovementMsg,
};
use crate::state::{self, PlayQueueInfo};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn album_list<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &PlayQueueInfo,
    state: &'a state::AlbumListState,
) -> Container<'a, Message> {
    match state {
        state::AlbumListState {
            page,
            sort_key,
            sort_order,
        } => {
            let body = {
                let page: usize = *page;

                let base_album_total_index = page * library.grid_info.get_page_size_usize();

                let indices = common::get_page(
                    library.album_sorts.from_sort_key(sort_key, sort_order),
                    page,
                    library.grid_info.get_page_size_usize(),
                );

                let mut paged_albums: Vec<musiqlibrary::ArtistAlbumInfo> = Vec::new();
                for index in indices.iter() {
                    paged_albums.push(library.get_artist_album_info(index.0, index.1));
                }

                let mut buttons: Vec<Button<Message>> = Vec::new();

                for (album_index_offset, info) in paged_albums.into_iter().enumerate() {
                    let full_album_info = library
                        .get_artist_album_keyed_tracks(info.artist.artist_id, info.album.album_id);

                    let relevant_sub_header =
                        get_sub_header_from_sort(&info, &full_album_info, &sort_key);

                    buttons.push(
                        dark_button(bottom_label(
                            album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Small,
                                    info.artist.artist_id,
                                    info.album.album_id,
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
                                    relevant_sub_header,
                                    consts::ICON_STR_LENGTH,
                                ))),
                        ))
                        .on_press(
                            message::ArtistNavMessage::ArtistAlbumView(
                                info.artist.artist_id,
                                info.album.album_id,
                                model::AlbumSize::Regular,
                                None,
                                Some(model::AlbumSortPlacement {
                                    index: base_album_total_index + album_index_offset,
                                    sort_key: sort_key.clone(),
                                    sort_order: sort_order.clone(),
                                }),
                            )
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

                let page_nav_component = line_row()
                    .push(paragraph("Page: "))
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
                                .on_press(user_nav_message(
                                    NavMessage::AlbumList(
                                        0,
                                        sort_key.clone(),
                                        model::SortOrder::Reversed,
                                    ),
                                )),
                            )
                            .push(
                                dark_button({
                                    if sort_order == &model::SortOrder::Regular {
                                        bright_paragraph("v")
                                    } else {
                                        dark_paragraph("v")
                                    }
                                })
                                .on_press(user_nav_message(
                                    NavMessage::AlbumList(
                                        0,
                                        sort_key.clone(),
                                        model::SortOrder::Regular,
                                    ),
                                )),
                            ),
                    );

                let sort_order_component = line_row().push(
                    line_row()
                        .push(paragraph("Sort By: "))
                        .push(sort_button(
                            "Artist Name",
                            model::AlbumSortKey::ByParent,
                            model::AlbumSortKey::ByParent.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Name",
                            model::AlbumSortKey::ByName,
                            model::AlbumSortKey::ByName.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Added",
                            model::AlbumSortKey::ByLastMod,
                            model::AlbumSortKey::ByLastMod.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Length",
                            model::AlbumSortKey::ByDuration,
                            model::AlbumSortKey::ByDuration.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Total Play Count",
                            model::AlbumSortKey::ByTotalPlayCount,
                            model::AlbumSortKey::ByTotalPlayCount.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Total Played Duration",
                            model::AlbumSortKey::ByTotalPlayedDuration,
                            model::AlbumSortKey::ByTotalPlayedDuration.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Date",
                            model::AlbumSortKey::ByDate,
                            model::AlbumSortKey::ByDate.default_order(),
                            sort_key,
                        ))
                        .push(sort_button(
                            "Random",
                            model::AlbumSortKey::Random,
                            model::AlbumSortKey::Random.default_order(),
                            sort_key,
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
                                        .push(h1("Albums"))
                                        .push(Space::with_width(Length::Fixed(20.0)))
                                        .push(page_nav_component),
                                )
                                .push(sort_order_component),
                        )
                        .push(scrollable),
                )
            };

            body
        }
    }
}

fn get_sub_header_from_sort(
    info: &musiqlibrary::ArtistAlbumInfo,
    full_album_info: &musiqlibrary::KeyedAlbumTracks<model::AugmentedTrack>,
    sort_key: &model::AlbumSortKey,
) -> String {
    match sort_key {
        model::AlbumSortKey::ByParent => info.artist.artist_name.clone(),
        model::AlbumSortKey::ByName => info.artist.artist_name.clone(),
        model::AlbumSortKey::ByLastMod => {
            format!(
                "{}",
                chrono::DateTime::<chrono::Local>::from(info.album.last_modified)
                    .format("%Y/%m/%d")
            )
        }
        model::AlbumSortKey::ByDuration => {
            common::format_duration(info.album.total_duration.as_secs())
        }
        model::AlbumSortKey::ByDate => {
            common::format_date_range(info.album.start_date, info.album.end_date)
        }
        model::AlbumSortKey::ByTotalPlayCount => format!(
            "{} listens",
            model::album_total_play_count(&full_album_info)
        ),
        model::AlbumSortKey::ByTotalPlayedDuration => {
            common::format_duration(model::album_total_played_duration(&full_album_info))
        }
        model::AlbumSortKey::Random => info.artist.artist_name.clone(),
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
