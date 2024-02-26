use iced::widget::{Button, Column, Container, ProgressBar, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};
use crate::state::{self, PlayerInfo};

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn artist_track_view_state<'a>(
    library: &'a model::LibraryState,
    player_info: &'a PlayerInfo,
    state: &'a state::ArtistTrackViewState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::ArtistTrackViewState {
            artist_id,

            sort_key,
            sort_order,
        } => {
            let artist = library.get_artist_info(*artist_id);

            let breadcrumbs = vec![
                (
                    "Artists".to_string(),
                    message::ArtistNavMessage::ArtistList(
                        0,
                        model::ArtistSortKey::ByName,
                        model::ArtistSortKey::ByName.default_order(),
                    )
                    .into_message(),
                ),
                (
                    artist.artist_name.clone(),
                    message::ArtistNavMessage::ArtistAlbumsView(*artist_id).into_message(),
                ),
            ];

            let artist_view_button_row = line_row()
                .push(dark_button(dark(h2("Albums"))).on_press(
                    message::ArtistNavMessage::ArtistAlbumsView(*artist_id).into_message(),
                ))
                .push(
                    dark_button(h2("Tracks")).on_press(
                        message::ArtistNavMessage::ArtistTrackView(
                            *artist_id,
                            model::ArtistTrackSortKey::ByTotalPlayCount,
                            model::ArtistTrackSortKey::ByTotalPlayCount.default_order(),
                        )
                        .into_message(),
                    ),
                )
                .push(
                    dark_button(dark(h2("Featured"))).on_press(
                        message::ArtistNavMessage::ArtistFeaturedTrackView(
                            *artist_id,
                            model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                            model::ArtistFeaturedTrackSortKey::ByTotalPlayCount.default_order(),
                        )
                        .into_message(),
                    ),
                );

            let artist_tracks = library.get_artist_tracks(artist_id);

            let track_sorts = model::AlbumTrackSorts::new(artist_tracks);

            let tracks = track_sorts.from_sort_key(sort_key, sort_order);

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
                .push(sort_button(
                    "Album",
                    artist.artist_id,
                    model::ArtistTrackSortKey::ByParent,
                    model::ArtistTrackSortKey::ByParent.default_order(),
                    sort_key,
                ))
                .push(sort_button(
                    "Name",
                    artist.artist_id,
                    model::ArtistTrackSortKey::ByName,
                    model::ArtistTrackSortKey::ByName.default_order(),
                    sort_key,
                ))
                .push(sort_button(
                    "Play Count",
                    artist.artist_id,
                    model::ArtistTrackSortKey::ByTotalPlayCount,
                    model::ArtistTrackSortKey::ByTotalPlayCount.default_order(),
                    sort_key,
                ))
                .push(sort_button(
                    "Length",
                    artist.artist_id,
                    model::ArtistTrackSortKey::ByDuration,
                    model::ArtistTrackSortKey::ByDuration.default_order(),
                    sort_key,
                ))
                .push(sort_button(
                    "Played Duration",
                    artist.artist_id,
                    model::ArtistTrackSortKey::ByTotalPlayedDuration,
                    model::ArtistTrackSortKey::ByTotalPlayedDuration.default_order(),
                    sort_key,
                ))
                .push(sort_button(
                    "Random",
                    artist.artist_id,
                    model::ArtistTrackSortKey::Random,
                    model::ArtistTrackSortKey::Random.default_order(),
                    sort_key,
                ));

            let sort_order_bar = line_row()
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
                        message::ArtistNavMessage::ArtistTrackView(
                            artist.artist_id,
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
                        message::ArtistNavMessage::ArtistTrackView(
                            artist.artist_id,
                            sort_key.clone(),
                            model::SortOrder::Regular,
                        )
                        .into_message(),
                    ),
                );

            for track in tracks.iter() {
                stripe_marker = !stripe_marker;

                let track_maybe_with_track_artist =
                    if track.metadata.album_artist == track.metadata.track_artist {
                        track.metadata.title.clone()
                    } else {
                        format!("{} ({})", track.metadata.title, track.metadata.track_artist)
                    };

                let row = Container::new(
                    dark_button(
                        line_row()
                            .spacing(5)
                            .push(album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Micro,
                                    track.metadata.album_artist_id,
                                    track.metadata.album_id,
                                ),
                                model::AlbumSize::Micro,
                            ))
                            .push(
                                bright_paragraph(track.metadata.track.to_string())
                                    .width(Length::Fixed(40.0)),
                            )
                            .push(
                                bright_paragraph(track_maybe_with_track_artist).width(Length::Fill),
                            )
                            .push(
                                bright_paragraph(track.augmented.play_count.to_string())
                                    .width(Length::Fixed(40.0)),
                            )
                            .push(
                                ProgressBar::new(
                                    0.0..=(greatest_play_count as f32),
                                    track.augmented.play_count as f32,
                                )
                                .width(Length::Fixed(50.0)),
                            )
                            .push({
                                let text_to_show =
                                    common::format_duration(track.metadata.duration.as_secs());
                                match current_track {
                                    Some(ref c) if (track == c) => bright_paragraph(text_to_show),
                                    _ => dark_paragraph(text_to_show),
                                }
                                .width(Length::Fixed(60.0))
                                .horizontal_alignment(iced::alignment::Horizontal::Right)
                            })
                            .push(Space::with_width(Length::Fixed(5.0))),
                    )
                    .on_press(
                        message::ArtistNavMessage::ArtistAlbumView(
                            track.metadata.album_artist_id,
                            track.metadata.album_id,
                            model::AlbumSize::Regular,
                            Some(musiqlibrary::TrackUniqueIdentifier::from_track(
                                &track.metadata,
                            )),
                            None,
                        )
                        .into_message(),
                    ),
                )
                .style(iced::theme::Container::Custom(
                    style::get_potential_current_stripe_style(
                        stripe_marker,
                        track,
                        &current_track,
                        &None,
                    ),
                ));

                tracks_table = tracks_table.push(row);
            }

            let scrollable = Scrollable::new(tracks_table).height(Length::Fill);

            let body = Container::new(
                Column::new()
                    .push(h1(artist.artist_name.clone()))
                    .push(artist_view_button_row)
                    .push(sort_bar)
                    .push(sort_order_bar)
                    .push(scrollable),
            );

            (breadcrumbs, body)
        }
    }
}

fn sort_button<'a>(
    display_text: &'static str,
    artist_id: musiqlibrary::ID,
    sort_key: model::ArtistTrackSortKey,
    order: model::SortOrder,
    current_sort: &'a model::ArtistTrackSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element).on_press(
        message::ArtistNavMessage::ArtistTrackView(artist_id, sort_key, order).into_message(),
    )
}
