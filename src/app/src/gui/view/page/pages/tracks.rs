use iced::widget::{Button, Column, Container, ProgressBar, Scrollable, Space};
use iced::{Element, Length};

use crate::model;

use crate::gui::message::{
    self, user_nav_message, Message, NavMessage, NavRelMsg, PagifiedMovementMsg,
};
use crate::state;

use super::super::super::common;
use super::super::super::elements::*;

pub fn track_list<'a>(
    library: &'a model::LibraryState,
    state: &'a state::TrackListState,
) -> Container<'a, Message> {
    match state {
        state::TrackListState {
            page,
            sort_key,
            sort_order,
        } => {
            let body = {
                let page: usize = *page;

                let greatest_play_count = library.get_track_max_play_count();

                let indices = common::get_page(
                    library.track_sorts.from_sort_key(sort_key, sort_order),
                    page,
                    library.grid_info.get_track_page_size_usize(),
                );

                let mut paged_tracks: Vec<model::AugmentedTrack> = Vec::new();
                for index in indices.iter() {
                    paged_tracks.push(library.get_track(index).clone());
                }

                let mut buttons: Vec<Button<Message>> = Vec::new();

                for info in paged_tracks.into_iter() {
                    buttons.push(
                        dark_button(
                            line_row()
                                .spacing(5)
                                .push(album_image(
                                    library.get_album_cover(
                                        model::AlbumSize::Micro,
                                        info.metadata.album_artist_id,
                                        info.metadata.album_id,
                                    ),
                                    model::AlbumSize::Micro,
                                ))
                                .push(
                                    bright_paragraph(info.augmented.play_count.to_string())
                                        .width(Length::Fixed(40.0)),
                                )
                                .push(
                                    ProgressBar::new(
                                        0.0..=(greatest_play_count as f32),
                                        info.augmented.play_count as f32,
                                    )
                                    .width(Length::Fixed(50.0)),
                                )
                                .push(
                                    bright_paragraph(info.metadata.title.clone())
                                        .width(Length::Fill),
                                )
                                .push(
                                    bright_paragraph(common::format_duration(
                                        info.metadata.duration.as_secs(),
                                    ))
                                    .width(Length::Fixed(60.0))
                                    .horizontal_alignment(iced::alignment::Horizontal::Right),
                                )
                                .push(Space::with_width(Length::Fixed(5.0))),
                        )
                        .on_press(
                            message::ArtistNavMessage::ArtistAlbumView(
                                info.metadata.album_artist_id,
                                info.metadata.album_id,
                                model::AlbumSize::Regular,
                                Some(musiqlibrary::TrackUniqueIdentifier::from_track(
                                    &info.metadata,
                                )),
                                None,
                            )
                            .into_message(),
                        ),
                    );
                }

                let mut columns: Column<Message> = Column::new();
                for _i in 0..library.grid_info.get_track_page_size_usize() {
                    if buttons.len() > 0 {
                        let button = buttons.remove(0);
                        columns = columns.push(button);
                    }
                }

                let grid: Element<Message> = columns.into();

                let scrollable = Scrollable::new(grid).height(Length::Fill);

                Container::new(
                    Column::new()
                        .spacing(6)
                        .push(h1("Tracks"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(sort_button(
                                        "Name",
                                        model::TrackSortKey::ByName,
                                        model::TrackSortKey::ByName.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Play Count",
                                        model::TrackSortKey::ByPlayCount,
                                        model::TrackSortKey::ByPlayCount.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Played Duration",
                                        model::TrackSortKey::ByPlayedAmount,
                                        model::TrackSortKey::ByPlayedAmount.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Length",
                                        model::TrackSortKey::ByDuration,
                                        model::TrackSortKey::ByDuration.default_order(),
                                        sort_key,
                                    ))
                                    .push(sort_button(
                                        "Random",
                                        model::TrackSortKey::Random,
                                        model::TrackSortKey::Random.default_order(),
                                        sort_key,
                                    )),
                            ),
                        )
                        .push(
                            line_row()
                                .push(
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
                                            .on_press(user_nav_message(NavMessage::TrackList(
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
                                            .on_press(user_nav_message(NavMessage::TrackList(
                                                0,
                                                sort_key.clone(),
                                                model::SortOrder::Regular,
                                            ))),
                                        ),
                                ),
                        )
                        .push(scrollable),
                )
            };
            body
        }
    }
}

fn sort_button<'a>(
    display_text: &'static str,
    sort_key: model::TrackSortKey,
    order: model::SortOrder,
    current_sort: &'a model::TrackSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element).on_press(user_nav_message(NavMessage::TrackList(0, sort_key, order)))
}
