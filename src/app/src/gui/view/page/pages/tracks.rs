use iced::widget::{Button, Column, Container, ProgressBar, Scrollable, Space};
use iced::{Element, Length};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::common;
use super::super::super::elements::*;

pub fn track_list<'a>(
    library: &'a model::LibraryState,
    state: &'a state::TrackListState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::TrackListState {
            page,
            sort_key,
            sort_order,
        } => {
            let breadcrumbs = vec![(
                "Tracks".to_string(),
                user_nav_message(NavMessage::TrackList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )];
            let body = {
                let page: usize = page.clone();

                let greatest_play_count = library.get_track_max_play_count();

                let indices = common::get_page(
                    library.track_sorts.from_sort_key(&sort_key, &sort_order),
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
                                        info.metadata.album_artist_id.clone(),
                                        info.metadata.album_id.clone(),
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
                        .on_press(user_nav_message(
                            NavMessage::ArtistAlbumView(
                                info.metadata.album_artist_id.clone(),
                                info.metadata.album_id.clone(),
                                model::AlbumSize::Regular,
                                Some(musiqlibrary::TrackUniqueIdentifier::from_track(
                                    &info.metadata,
                                )),
                            ),
                        )),
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

                let scrollable = Scrollable::new(grid);

                let mut total_tracks = 0;
                for (_, artist) in library.get_artist_map().iter() {
                    for (_, album) in artist.albums.iter() {
                        for (_, disc) in album.discs.iter() {
                            total_tracks = total_tracks + disc.tracks.len();
                        }
                    }
                }

                let first_page = 0;
                let back_page = {
                    if page == 0 {
                        0
                    } else {
                        page - 1
                    }
                };
                let forward_page = {
                    if ((page + 1) * library.grid_info.get_track_page_size_usize()) >= total_tracks
                    {
                        page
                    } else {
                        page + 1
                    }
                };
                let last_page = {
                    let maybe_last_page =
                        total_tracks / library.grid_info.get_track_page_size_usize();
                    if maybe_last_page * library.grid_info.get_track_page_size_usize()
                        >= total_tracks
                    {
                        maybe_last_page - 1
                    } else {
                        maybe_last_page
                    }
                };
                Container::new(
                    Column::new()
                        .spacing(6)
                        .push(h1("Tracks"))
                        .push(
                            line_row().push(
                                line_row()
                                    .push(paragraph("Sort By: "))
                                    .push(dark_button(bright_paragraph("Name")).on_press(
                                        user_nav_message(NavMessage::TrackList(
                                            0,
                                            model::TrackSortKey::ByName,
                                            model::SortOrder::Regular,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Play Count")).on_press(
                                        user_nav_message(NavMessage::TrackList(
                                            0,
                                            model::TrackSortKey::ByPlayCount,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(dark_button(bright_paragraph("Duration")).on_press(
                                        user_nav_message(NavMessage::TrackList(
                                            0,
                                            model::TrackSortKey::ByDuration,
                                            model::SortOrder::Reversed,
                                        )),
                                    ))
                                    .push(
                                        dark_button(bright_paragraph("Played Duration")).on_press(
                                            user_nav_message(NavMessage::TrackList(
                                                0,
                                                model::TrackSortKey::ByPlayedAmount,
                                                model::SortOrder::Reversed,
                                            )),
                                        ),
                                    )
                                    .push(dark_button(bright_paragraph("Random")).on_press(
                                        user_nav_message(NavMessage::TrackList(
                                            0,
                                            model::TrackSortKey::ByRandom,
                                            model::SortOrder::Regular,
                                        )),
                                    )),
                            ),
                        )
                        .push(
                            line_row()
                                .push(
                                    line_row()
                                        .push(paragraph("Page: "))
                                        .push(dark_button(bright_paragraph("<<")).on_press(
                                            user_nav_message(NavMessage::TrackList(
                                                first_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph("<")).on_press(
                                            user_nav_message(NavMessage::TrackList(
                                                back_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(bright_paragraph(page.to_string()))
                                        .push(dark_button(bright_paragraph(">")).on_press(
                                            user_nav_message(NavMessage::TrackList(
                                                forward_page,
                                                sort_key.clone(),
                                                sort_order.clone(),
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph(">>")).on_press(
                                            user_nav_message(NavMessage::TrackList(
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
                                            user_nav_message(NavMessage::TrackList(
                                                0,
                                                sort_key.clone(),
                                                model::SortOrder::Reversed,
                                            )),
                                        ))
                                        .push(dark_button(bright_paragraph("v")).on_press(
                                            user_nav_message(NavMessage::TrackList(
                                                0,
                                                sort_key.clone(),
                                                model::SortOrder::Regular,
                                            )),
                                        )),
                                ),
                        )
                        .push(scrollable),
                )
            };
            (breadcrumbs, body)
        }
    }
}
