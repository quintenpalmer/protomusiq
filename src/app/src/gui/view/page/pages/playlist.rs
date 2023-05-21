use iced::widget::{Checkbox, Column, Container, ProgressBar, Row, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state::{self, ActionState, PlayerInfoState};
use crate::util::shuffle;

use crate::gui::view::components;

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn playlist_view<'a>(
    library: &'a model::LibraryState,
    action_state: &'a ActionState,
    player_info: &'a PlayerInfoState,
    state: &'a state::PlaylistViewState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::PlaylistViewState { playlist_id } => {
            let playlist = library.user_playlists.get(*playlist_id).unwrap();

            let should_shuffle = action_state.group_buttons_shuffle;

            let augmented_tracks: Vec<_> = playlist
                .tracks
                .iter()
                .map(|track| library.get_track(track).clone())
                .collect();

            let full_tracks = augmented_tracks
                .iter()
                .map(|track| track.metadata.clone())
                .collect();

            let greatest_play_count = playlist
                .tracks
                .iter()
                .map(|track| library.get_track(track).augmented.play_count)
                .max()
                .unwrap_or(0);

            let mut column = Column::new().spacing(10).push(h2("Playlist")).push(
                Container::new(
                    line_row()
                        .push(components::compute_playlist_thumbnail(
                            &library,
                            &playlist.tracks,
                            components::PlaylistIconSize::Large,
                        ))
                        .push(
                            Column::new()
                                .padding(10)
                                .spacing(10)
                                .push(h1(playlist.name.clone()))
                                .push(bright_paragraph(common::format_duration(
                                    model::compute_track_list_duration(&full_tracks).as_secs(),
                                )))
                                .push(
                                    Row::new()
                                        .push(dark_button(bright_paragraph("> Play All")).on_press(
                                            Message::PlaybackRequest(
                                                message::PlaybackRequest::PlaySongs(
                                                    if should_shuffle {
                                                        shuffle::shuffle(augmented_tracks.clone())
                                                    } else {
                                                        augmented_tracks.clone()
                                                    },
                                                ),
                                            ),
                                        ))
                                        .push(
                                            dark_button(bright_paragraph(">| Insert All Next"))
                                                .on_press(Message::PlaybackRequest(
                                                    message::PlaybackRequest::InsertSongs(
                                                        if should_shuffle {
                                                            shuffle::shuffle(
                                                                augmented_tracks.clone(),
                                                            )
                                                        } else {
                                                            augmented_tracks.clone()
                                                        },
                                                        false,
                                                    ),
                                                )),
                                        )
                                        .push(
                                            dark_button(bright_paragraph("|> Append All"))
                                                .on_press(Message::PlaybackRequest(
                                                    message::PlaybackRequest::AppendSongs(
                                                        if should_shuffle {
                                                            shuffle::shuffle(
                                                                augmented_tracks.clone(),
                                                            )
                                                        } else {
                                                            augmented_tracks.clone()
                                                        },
                                                        false,
                                                    ),
                                                )),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .push(Checkbox::new("", should_shuffle, |_| {
                                            Message::Action(message::Action::ToggleShuffleOnAdd)
                                        }))
                                        .push(bright_paragraph("Shuffle (on add)")),
                                ),
                        ),
                )
                .padding(10)
                .width(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(
                    style::ContainerPopForward,
                ))),
            );
            let mut stripe_marker = true;
            let current_track = match player_info.current_playback {
                Some(ref o) => match o {
                    state::CurrentPlayback::Track(ref v) => Some(v.track.clone()),
                    _ => None,
                },
                None => None,
            };

            let mut tracks_column = Column::new();

            let tracks: Vec<_> = playlist
                .tracks
                .iter()
                .map(|track_id| library.get_track(&track_id))
                .collect();

            for track in tracks.clone().into_iter() {
                stripe_marker = !stripe_marker;
                let row = Container::new(
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
                        .push(dark_button(bright_paragraph("> ")).on_press(
                            Message::PlaybackRequest(message::PlaybackRequest::PlaySongs(vec![
                                track.clone(),
                            ])),
                        ))
                        .push(
                            dark_button(bright_paragraph(track.metadata.title.clone()))
                                .on_press(components::track_link(&track.metadata))
                                .width(Length::Fill),
                        )
                        .push(
                            line_row()
                                .spacing(5)
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
                                .push(dark_button(bright_paragraph(">|")).on_press(
                                    Message::PlaybackRequest(
                                        message::PlaybackRequest::InsertSongs(
                                            vec![track.clone()],
                                            false,
                                        ),
                                    ),
                                ))
                                .push(dark_button(bright_paragraph("|>")).on_press(
                                    Message::PlaybackRequest(
                                        message::PlaybackRequest::AppendSongs(
                                            vec![track.clone()],
                                            false,
                                        ),
                                    ),
                                )),
                        )
                        .push({
                            let text_to_show =
                                common::format_duration(track.metadata.duration.as_secs());

                            match player_info.get_maybe_current_playback_track() {
                                Some(c) if (*track == c.clone()) => bright_paragraph(text_to_show),
                                _ => dark_paragraph(text_to_show),
                            }
                            .width(Length::Fixed(60.0))
                            .horizontal_alignment(iced::alignment::Horizontal::Right)
                        })
                        .push(
                            line_row()
                                .push(dark_button(bright_paragraph("^")).on_press(Message::Action(
                                    message::Action::MoveTrackInPlaylist(
                                        *playlist_id,
                                        message::Direction::Up,
                                        musiqlibrary::TrackUniqueIdentifier::from_track(
                                            &track.metadata,
                                        ),
                                    ),
                                )))
                                .push(dark_button(bright_paragraph("v")).on_press(Message::Action(
                                    message::Action::MoveTrackInPlaylist(
                                        *playlist_id,
                                        message::Direction::Down,
                                        musiqlibrary::TrackUniqueIdentifier::from_track(
                                            &track.metadata,
                                        ),
                                    ),
                                )))
                                .push(dark_button(bright_paragraph(" - ")).on_press(
                                    Message::Action(message::Action::RemoveTrackFromPlaylist(
                                        *playlist_id,
                                        musiqlibrary::TrackUniqueIdentifier::from_track(
                                            &track.metadata,
                                        ),
                                    )),
                                )),
                        )
                        .push(Space::with_width(Length::Fixed(5.0))),
                )
                .style(iced::theme::Container::Custom(
                    style::get_potential_current_stripe_style(
                        stripe_marker,
                        &track,
                        &current_track,
                        &None,
                    ),
                ));
                tracks_column = tracks_column.push(row);
            }

            column = column.push(Scrollable::new(tracks_column));
            (
                vec![
                    (
                        "Playlists".to_string(),
                        user_nav_message(NavMessage::PlaylistList("".to_string())),
                    ),
                    (
                        playlist.name.clone(),
                        user_nav_message(NavMessage::PlaylistView(playlist_id.clone())),
                    ),
                ],
                Container::new(column),
            )
        }
    }
}
