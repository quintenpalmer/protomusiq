use iced::widget::{Button, Column, Container, ProgressBar, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state::{self, PlayerInfo};

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn artist_featured_track_view_state<'a>(
    library: &'a model::LibraryState,
    player_info: &'a PlayerInfo,
    state: &'a state::ArtistFeaturedTrackViewState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::ArtistFeaturedTrackViewState {
            artist_id,

            sort_key,
            sort_order,
        } => {
            let artist = library.get_artist_info(*artist_id);

            let featured_tracks = library.get_featured_tracks_for_artist(artist_id);

            let breadcrumbs = vec![
                (
                    "Artists".to_string(),
                    user_nav_message(NavMessage::ArtistList(
                        0,
                        model::ArtistSortKey::ByName,
                        model::SortOrder::Regular,
                    )),
                ),
                (
                    artist.artist_name.clone(),
                    user_nav_message(NavMessage::ArtistAlbumsView(*artist_id)),
                ),
            ];

            let artist_view_button_row = line_row()
                .push(
                    dark_button(dark(h2("Albums")))
                        .on_press(user_nav_message(NavMessage::ArtistAlbumsView(*artist_id))),
                )
                .push(dark_button(dark(h2("Tracks"))).on_press(user_nav_message(
                    NavMessage::ArtistTrackView(
                        *artist_id,
                        model::ArtistTrackSortKey::ByTotalPlayCount,
                        model::SortOrder::Reversed,
                    ),
                )))
                .push(dark_button(h2("Featured")).on_press(user_nav_message(
                    NavMessage::ArtistFeaturedTrackView(
                        *artist_id,
                        model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                        model::SortOrder::Reversed,
                    ),
                )));

            let track_sorts = model::AlbumFeaturedTrackSorts::new(featured_tracks);

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
                    &artist.artist_id,
                    "Album",
                    model::ArtistFeaturedTrackSortKey::ByParent,
                    model::SortOrder::Regular,
                    sort_key,
                ))
                .push(sort_button(
                    &artist.artist_id,
                    "Name",
                    model::ArtistFeaturedTrackSortKey::ByName,
                    model::SortOrder::Regular,
                    sort_key,
                ))
                .push(sort_button(
                    &artist.artist_id,
                    "Play Count",
                    model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                    model::SortOrder::Reversed,
                    sort_key,
                ))
                .push(sort_button(
                    &artist.artist_id,
                    "Length",
                    model::ArtistFeaturedTrackSortKey::ByDuration,
                    model::SortOrder::Reversed,
                    sort_key,
                ))
                .push(sort_button(
                    &artist.artist_id,
                    "Played Duration",
                    model::ArtistFeaturedTrackSortKey::ByTotalPlayedDuration,
                    model::SortOrder::Reversed,
                    sort_key,
                ))
                .push(sort_button(
                    &artist.artist_id,
                    "Random",
                    model::ArtistFeaturedTrackSortKey::Random,
                    model::SortOrder::Regular,
                    sort_key,
                ));

            let sort_order_bar = line_row()
                .push(paragraph("Order: "))
                .push(
                    dark_button(bright_paragraph("^")).on_press(user_nav_message(
                        NavMessage::ArtistFeaturedTrackView(
                            artist.artist_id,
                            sort_key.clone(),
                            model::SortOrder::Reversed,
                        ),
                    )),
                )
                .push(
                    dark_button(bright_paragraph("v")).on_press(user_nav_message(
                        NavMessage::ArtistFeaturedTrackView(
                            artist.artist_id,
                            sort_key.clone(),
                            model::SortOrder::Regular,
                        ),
                    )),
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
                    .on_press(user_nav_message(NavMessage::ArtistAlbumView(
                        track.metadata.album_artist_id,
                        track.metadata.album_id,
                        model::AlbumSize::Regular,
                        Some(musiqlibrary::TrackUniqueIdentifier::from_track(
                            &track.metadata,
                        )),
                        None,
                    ))),
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

            let scrollable = Scrollable::new(tracks_table);

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
    artist_id: &musiqlibrary::ID,
    display_text: &'static str,
    sort_key: model::ArtistFeaturedTrackSortKey,
    order: model::SortOrder,
    current_sort: &'a model::ArtistFeaturedTrackSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element).on_press(user_nav_message(NavMessage::ArtistFeaturedTrackView(
        *artist_id, sort_key, order,
    )))
}
