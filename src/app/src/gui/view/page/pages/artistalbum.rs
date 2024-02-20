use iced::widget::{Checkbox, Column, Container, ProgressBar, Row, Scrollable, Space};
use iced::{Element, Length};

use crate::model;
use crate::shared;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state::{self, ActionState, PlayerInfo};
use crate::util::shuffle;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;
use super::super::super::style;

pub fn artist_album_view_state<'a>(
    library: &'a model::LibraryState,
    action_state: &'a ActionState,
    player_info: &'a PlayerInfo,
    state: &'a state::ArtistAlbumViewState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::ArtistAlbumViewState {
            artist_id,
            album_id,
            album_size,
            maybe_selected_track,
            maybe_current_sort_order,
        } => {
            let artist = library.get_artist_map().get(artist_id).unwrap();
            let album = artist.albums.get(album_id).unwrap();
            let discs = album.discs.values();
            let tracks = album.discs.values().fold(Vec::new(), |mut total, current| {
                for track in current.tracks.values() {
                    total.push(track.clone());
                }
                total
            });
            let should_shuffle = action_state.group_buttons_shuffle;

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
                    artist.artist_info.artist_name.clone(),
                    user_nav_message(NavMessage::ArtistAlbumsView(*artist_id)),
                ),
                (
                    common::abr_str(album.album_info.album_name.clone(), consts::NAV_STR_LENGTH),
                    user_nav_message(NavMessage::ArtistAlbumView(
                        *artist_id,
                        *album_id,
                        model::AlbumSize::Regular,
                        None,
                        maybe_current_sort_order.clone(),
                    )),
                ),
            ];

            let mut body_column = Column::new()
                    .push(
                        Row::new()
                            .push({
                                let (current, toggle_to) = match album_size {
                                    model::AlbumSize::Micro => (model::AlbumSize::Micro, model::AlbumSize::Mini),
                                    model::AlbumSize::Mini => (model::AlbumSize::Mini, model::AlbumSize::Small),
                                    model::AlbumSize::Small => (model::AlbumSize::Small, model::AlbumSize::Regular),
                                    model::AlbumSize::Regular => (model::AlbumSize::Regular, model::AlbumSize::Large),
                                    model::AlbumSize::Large => (model::AlbumSize::Large, model::AlbumSize::Micro),
                                };
                                dark_button(album_image(
                                    library.get_album_cover(
                                        current.clone(),
                                        artist.artist_info.artist_id,
                                        album.album_info.album_id,
                                    ),
                                    current,
                                )).on_press(user_nav_message(NavMessage::ArtistAlbumView(
                                    *artist_id,
                                    *album_id,
                                    toggle_to,
                                    maybe_selected_track.clone(),
                                    maybe_current_sort_order.clone(),
                                )))
                            })
                            .push(
                                Column::new()
                                    .padding(10)
                                    .spacing(10)
                                    .push(h1(album.album_info.album_name.clone()))
                                    .push(h2(format!(
                                        "{} - {}",
                                        artist.artist_info.artist_name,
                                        common::format_date_range(
                                            album.album_info.start_date,
                                            album.album_info.end_date
                                        ),
                                    )))
                                    .push(h3(format!(
                                        "{} - {}",
                                        common::fold_strings(
                                            &album.album_info.genres.iter().collect(),
                                        ),
                                        common::format_duration(
                                            album.album_info.total_duration.as_secs(),
                                        ),
                                    )))
                                    .push(
                                        Row::new()
                                            .spacing(5)
                                            .push(
                                                dark_button(
                                                    bright_paragraph("> Play All"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    shared::PlaybackRequest::PlaySongs(
                                                        if should_shuffle {
                                                            shuffle::shuffle(tracks.clone())
                                                        } else {
                                                            tracks.clone()
                                                        }
                                                    ),
                                                )),
                                            )
                                            .push(
                                                dark_button(
                                                    bright_paragraph(">| Insert All Next"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    shared::PlaybackRequest::InsertSongs(
                                                        if should_shuffle {
                                                            shuffle::shuffle(tracks.clone())
                                                        } else {
                                                            tracks.clone()
                                                        },
                                                        false,
                                                    ),
                                                )),
                                            )
                                            .push(
                                                dark_button(
                                                    bright_paragraph("|> Append All"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    shared::PlaybackRequest::AppendSongs(
                                                        if should_shuffle {
                                                            shuffle::shuffle(tracks.clone())
                                                        } else {
                                                            tracks.clone()
                                                        },
                                                        false,
                                                    ),
                                                )),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(5)
                                            .push(
                                                match library.user_playlists.get_default_playlist_id() {
                                                    Some(default_playlist_id) => {
                                                        let default_playlist = library.user_playlists.get_playlist(default_playlist_id).unwrap();
                                                        Container::new(dark_button(
                                                                bright_paragraph(format!("+ Add All to:\n\"{}\"", default_playlist.name)),
                                                            )
                                                            .on_press(Message::Action(message::Action::AddTracksToPlaylist(
                                                                default_playlist_id,
                                                                tracks
                                                                    .iter()
                                                                    .map(|track|
                                                                        musiqlibrary::TrackUniqueIdentifier::from_track(&track.metadata)
                                                                    ).collect()
                                                                ))
                                                            )
                                                        )
                                                    },
                                                    None => Container::new(bright_paragraph("(Create Default Playlist)")),
                                                }
                                            )
                                            .push(
                                                Row::new()
                                                    .push(
                                                        Checkbox::new(
                                                            "",
                                                            should_shuffle,
                                                        ).on_toggle(
                                                            |_| Message::Action(message::Action::ToggleShuffleOnAdd),
                                                        )
                                                    )
                                                    .push(
                                                        bright_paragraph("Shuffle (on add)")
                                                    )
                                            )
                                    ),
                            ),
                    )
                    .push(Space::new(Length::Fill, Length::Fixed(20.0)))
                    .push({
                        let mut column = Column::new().padding(15);
                        let mut stripe_marker = false;
                        let disc_count = discs.len();

                        for disc in discs.into_iter() {
                            if disc_count > 1 {
                                column = column.push(
                                        Row::new()
                                            .push(
                                                Row::new()
                                                    .push(
                                                        dark_button(
                                                            bright_paragraph(">"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            shared::PlaybackRequest::PlaySongs(
                                                                disc.tracks.values()
                                                                    .cloned()
                                                                    .collect(),
                                                            ),
                                                        )),
                                                    )
                                            )
                                            .push(Space::with_width(Length::Fixed(5.0)))
                                            .push(h2(format!("Disc {}", disc.disc_no)))
                                            .push(Space::with_width(Length::Fill))
                                            .push(
                                                Row::new()
                                                    .push(
                                                        dark_button(
                                                            bright_paragraph(">|"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            shared::PlaybackRequest::InsertSongs(
                                                                disc.tracks.values()
                                                                    .cloned()
                                                                    .collect(),
                                                                false,
                                                            ),
                                                        )),
                                                    )
                                                    .push(
                                                        dark_button(
                                                            bright_paragraph("|>"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            shared::PlaybackRequest::AppendSongs(
                                                                disc.tracks.values()
                                                                    .cloned()
                                                                    .collect(),
                                                                false,
                                                            ),
                                                        )),
                                                    )
                                                    .push(
                                                        match library.user_playlists.get_default_playlist_id() {
                                                            Some(default_playlist_id) => Container::new(dark_button(
                                                                    bright_paragraph("+"),
                                                                )
                                                                .on_press(Message::Action(message::Action::AddTracksToPlaylist(
                                                                    default_playlist_id,
                                                                    disc.tracks.values()
                                                                        .map(|track|
                                                                            musiqlibrary::TrackUniqueIdentifier::from_track(&track.metadata)
                                                                        ).collect()
                                                                )))),
                                                            None => Container::new(bright_paragraph(" ")),
                                                        }
                                                    ),
                                                ).push(
                                                    Space::with_width(Length::Fixed(75.0))
                                                ),
                                );
                            }
                            for track in disc.tracks.values() {
                                stripe_marker = !stripe_marker;

                                let maybe_track_artist = track.metadata.get_maybe_track_artist();

                                column = column.push(
                                    Container::new(
                                        line_row()
                                            .spacing(5)
                                            .push(if disc_count > 1 {
                                                Space::with_width(5.0)
                                                } else {
                                                    Space::with_width(0.0)
                                                }
                                            )
                                            .push(
                                                dark_button(bright_paragraph(">"))
                                                    .on_press(Message::PlaybackRequest(
                                                        shared::PlaybackRequest::PlaySongs(
                                                            vec![track.clone()],
                                                        ),
                                                    )),
                                            )
                                            .push(
                                                dark_button(bright_paragraph(">..."))
                                                    .on_press(Message::PlaybackRequest(
                                                        shared::PlaybackRequest::PlaySongs(
                                                            model::functions::tracks_after_including(&tracks, track),
                                                        ),
                                                    )),
                                            )
                                            .push(
                                                bright_paragraph(track.metadata.track.to_string())
                                                    .width(Length::Fixed(40.0)),
                                            )
                                            .push(
                                                bright_paragraph(track.metadata.title.clone())
                                                    .width(Length::Fill),
                                            )
                                            .push(
                                                maybe_track_artist
                                                    .map(|track_artist| {
                                                             let ret: Element<Message> = dark_text_like_button(paragraph(format!(
                                                                " ({})",
                                                                track_artist
                                                            )))
                                                            .on_press(
                                                                user_nav_message(NavMessage::ArtistFeaturedTrackView(
                                                                    musiqlibrary::ID::new(&track_artist),
                                                                    model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                                                                    model::SortOrder::Reversed,
                                                                )),
                                                            ).into();
                                                            ret
                                                        }
                                                    )
                                                    .unwrap_or(paragraph("".to_string()).into())
                                            )
                                            .push(
                                                bright_paragraph(track.augmented.play_count.to_string())
                                                    .width(Length::Fixed(40.0)),
                                            )
                                            .push(
                                                ProgressBar::new(0.0..=(greatest_play_count as f32), track.augmented.play_count as f32).width(Length::Fixed(50.0)),
                                            )
                                            .push(Row::new()
                                                .spacing(0)
                                                .padding(0)
                                                .push(
                                                    dark_button(
                                                        bright_paragraph(">|"),
                                                    )
                                                    .on_press(Message::PlaybackRequest(
                                                        shared::PlaybackRequest::InsertSongs(
                                                            vec![track.clone()],
                                                            false,
                                                        ),
                                                    )),
                                                )
                                                .push(
                                                    dark_button(
                                                        bright_paragraph("|>"),
                                                    )
                                                    .on_press(Message::PlaybackRequest(
                                                        shared::PlaybackRequest::AppendSongs(
                                                            vec![track.clone()],
                                                            false,
                                                        ),
                                                    )),
                                                )
                                                .push(
                                                    match library.user_playlists.get_default_playlist_id() {
                                                        Some(default_playlist_id) => Container::new(dark_button(
                                                                bright_paragraph("+"),
                                                            )
                                                            .on_press(Message::Action(message::Action::AddTracksToPlaylist(
                                                                default_playlist_id,
                                                                vec![musiqlibrary::TrackUniqueIdentifier::from_track(&track.metadata)],
                                                            )))),
                                                        None => Container::new(bright_paragraph(" ")),
                                                    }
                                                ),
                                            )
                                            .push({
                                                let text_to_show = common::format_duration(track.metadata.duration.as_secs());
                                                match current_track {
                                                    Some(ref c) if (track == c) => {
                                                        bright_paragraph(text_to_show)
                                                    },
                                                    _ => match maybe_selected_track {
                                                        Some(ref selected) if (musiqlibrary::TrackUniqueIdentifier::from_track(&track.metadata) == *selected) => {
                                                            bright_paragraph(text_to_show)
                                                        },
                                                        _ => dark_paragraph(text_to_show),
                                                    },
                                                }.width(Length::Fixed(60.0))
                                                .horizontal_alignment(iced::alignment::Horizontal::Right)
                                            })
                                            .push(Space::with_width(Length::Fixed(5.0)))
                                    )
                                    .style(
                                        iced::theme::Container::Custom(
                                            style::get_potential_current_stripe_style(
                                                stripe_marker,
                                                track,
                                                &current_track,
                                                maybe_selected_track,
                                            ),
                                        ),
                                   )
                                )
                            }
                        }

                        Container::new(
                            Container::new(Scrollable::new(column).height(Length::Fill)).height(Length::Fill)
                                .padding(10)
                                .style(iced::theme::Container::Custom(
                                    Box::new(style::ContainerDarkInset{})
                                ))
                        )
                        .padding(10).height(Length::Fill)
                    });

            match maybe_current_sort_order {
                Some(current_sort_order) => {
                    let sort_nav_row =
                        line_row()
                            .spacing(8)
                            .push(dark_button(bright_paragraph("<")).on_press(
                                Message::NavRelative(message::NavRelMsg::PagifiedMovement(
                                    message::PagifiedMovementMsg::Backwards,
                                )),
                            ))
                            .push(h3(format!("{}", current_sort_order.index)))
                            .push(dark_button(bright_paragraph(">")).on_press(
                                Message::NavRelative(message::NavRelMsg::PagifiedMovement(
                                    message::PagifiedMovementMsg::Forwards,
                                )),
                            ))
                            .push(h3(format!(
                                "{} ({})",
                                current_sort_order.sort_key.display_text(),
                                current_sort_order.sort_order.display_text(),
                            )));
                    body_column = body_column.push(sort_nav_row);
                }
                None => (),
            }

            let body = Container::new(body_column);

            (breadcrumbs, body)
        }
    }
}
