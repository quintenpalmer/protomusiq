use iced::{self, button, Column, Container, Length, ProgressBar, Scrollable, Space};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state::{self, PlayerInfoState};

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn artist_track_view_state<'a>(
    library: &'a model::LibraryState,
    player_info: &'a PlayerInfoState,
    state: &'a mut state::ArtistTrackViewState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::ArtistTrackViewState {
            artist_list_breadcrumb,
            artist_view_breadcrumb,

            sort_by_name_button,
            sort_by_album_button,
            sort_by_play_count_button,
            sort_by_duration_button,
            sort_by_played_duration_button,
            sort_random_button,

            album_view_button,
            track_view_button,

            sort_order_regular_button,
            sort_order_reverse_button,

            track_buttons,
            track_scroll,

            artist_id,

            sort_key,
            sort_order,
        } => {
            let artist = library.get_artist_map().get(&artist_id).unwrap();

            let breadcrumbs = vec![
                (
                    artist_list_breadcrumb,
                    "Artists".to_string(),
                    user_nav_message(NavMessage::ArtistList(
                        0,
                        model::ArtistSortKey::ByName,
                        model::SortOrder::Regular,
                    )),
                ),
                (
                    artist_view_breadcrumb,
                    artist.artist_info.artist_name.clone(),
                    user_nav_message(NavMessage::ArtistView(artist_id.clone())),
                ),
            ];

            let artist_view_button_row = line_row()
                .push(
                    dark_button(album_view_button, h2("Albums"))
                        .on_press(user_nav_message(NavMessage::ArtistView(artist_id.clone()))),
                )
                .push(
                    dark_button(track_view_button, h2("Tracks")).on_press(user_nav_message(
                        NavMessage::ArtistTrackView(
                            artist_id.clone(),
                            model::ArtistTrackSortKey::ByTotalPlayCount,
                            model::SortOrder::Reversed,
                        ),
                    )),
                );

            let track_sorts = model::AlbumTrackSorts::new(&artist);

            let tracks = track_sorts.from_sort_key(&sort_key, &sort_order);

            let current_track = match player_info.current_playback {
                Some(ref o) => match o {
                    state::CurrentPlayback::Track(ref v) => Some(v.track.clone()),
                    _ => None,
                },
                None => None,
            };
            let greatest_play_count = tracks
                .iter()
                .map(|track| track.augmented.play_count)
                .max()
                .unwrap_or(0);

            let mut tracks_table = Column::new().padding(15);

            let mut stripe_marker = false;

            let sort_bar = line_row()
                .push(paragraph("Sort By: "))
                .push(
                    dark_button(sort_by_album_button, bright_paragraph("Album")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            model::ArtistTrackSortKey::ByParent,
                            sort_order.clone(),
                        )),
                    ),
                )
                .push(
                    dark_button(sort_by_name_button, bright_paragraph("Name")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            model::ArtistTrackSortKey::ByName,
                            sort_order.clone(),
                        )),
                    ),
                )
                .push(
                    dark_button(sort_by_play_count_button, bright_paragraph("Play Count"))
                        .on_press(user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            model::ArtistTrackSortKey::ByTotalPlayCount,
                            sort_order.clone(),
                        ))),
                )
                .push(
                    dark_button(sort_by_duration_button, bright_paragraph("Duration")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            model::ArtistTrackSortKey::ByDuration,
                            sort_order.clone(),
                        )),
                    ),
                )
                .push(
                    dark_button(
                        sort_by_played_duration_button,
                        bright_paragraph("Played Duration"),
                    )
                    .on_press(user_nav_message(NavMessage::ArtistTrackView(
                        artist.artist_info.artist_id.clone(),
                        model::ArtistTrackSortKey::ByTotalPlayedDuration,
                        sort_order.clone(),
                    ))),
                )
                .push(
                    dark_button(sort_random_button, bright_paragraph("Random")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            model::ArtistTrackSortKey::Random,
                            sort_order.clone(),
                        )),
                    ),
                );

            let sort_order_bar = line_row()
                .push(paragraph("Order: "))
                .push(
                    dark_button(sort_order_reverse_button, bright_paragraph("^")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            sort_key.clone(),
                            model::SortOrder::Reversed,
                        )),
                    ),
                )
                .push(
                    dark_button(sort_order_regular_button, bright_paragraph("v")).on_press(
                        user_nav_message(NavMessage::ArtistTrackView(
                            artist.artist_info.artist_id.clone(),
                            sort_key.clone(),
                            model::SortOrder::Regular,
                        )),
                    ),
                );

            for (track_button, track) in track_buttons.iter_mut().zip(tracks.into_iter()) {
                stripe_marker = !stripe_marker;

                let track_maybe_with_track_artist =
                    if track.metadata.album_artist == track.metadata.track_artist {
                        track.metadata.title.clone()
                    } else {
                        format!("{} ({})", track.metadata.title, track.metadata.track_artist)
                    };

                let row = Container::new(
                    dark_button(
                        track_button,
                        line_row()
                            .spacing(5)
                            .push(album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Micro,
                                    track.metadata.album_artist_id.clone(),
                                    track.metadata.album_id.clone(),
                                ),
                                model::AlbumSize::Micro,
                            ))
                            .push(
                                bright_paragraph(track.metadata.track.to_string())
                                    .width(Length::Units(40)),
                            )
                            .push(bright_paragraph(track_maybe_with_track_artist).width(Length::Fill))
                            .push(
                                bright_paragraph(track.augmented.play_count.to_string())
                                    .width(Length::Units(40)),
                            )
                            .push(
                                ProgressBar::new(
                                    0.0..=(greatest_play_count as f32),
                                    track.augmented.play_count as f32,
                                )
                                .width(Length::Units(50)),
                            )
                            .push({
                                let text_to_show =
                                    common::format_duration(track.metadata.duration.as_secs());
                                match current_track {
                                    Some(ref c) if (track == c) => bright_paragraph(text_to_show),
                                    _ => dark_paragraph(text_to_show),
                                }
                                .width(Length::Units(60))
                                .horizontal_alignment(iced::HorizontalAlignment::Right)
                            })
                            .push(Space::with_width(Length::Units(5))),
                    )
                    .on_press(user_nav_message(
                        NavMessage::ArtistAlbumView(
                            track.metadata.album_artist_id.clone(),
                            track.metadata.album_id.clone(),
                            model::AlbumSize::Regular,
                            Some(
                                musiqlibrary::TrackUniqueIdentifier::from_track(
                                    &track
                                    .metadata
                                 )
                            )
                        ),
                    )),
                )
                .style(style::get_potential_current_stripe_style(
                    stripe_marker,
                    &track,
                    &current_track,
                    &None,
                ));

                tracks_table = tracks_table.push(row);
            }

            let scrollable = Scrollable::new(track_scroll).push(tracks_table);

            let body = Container::new(
                Column::new()
                    .push(h1(artist.artist_info.artist_name.clone()))
                    .push(artist_view_button_row)
                    .push(sort_bar)
                    .push(sort_order_bar)
                    .push(scrollable),
            );

            (breadcrumbs, body)
        }
    }
}
