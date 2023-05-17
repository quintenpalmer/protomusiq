use iced::{self, button, Checkbox, Column, Container, Length, ProgressBar, Row, Scrollable, Space};

use crate::model;

use crate::util::shuffle;
use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state::{self, ActionState, PlayerInfoState};

use crate::gui::view::components;

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn playlist_view<'a>(
    library: &'a model::LibraryState,
    action_state: &'a ActionState,
    player_info: &'a PlayerInfoState,
    state: &'a mut state::PlaylistViewState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::PlaylistViewState {
            playlist_play_queue_buttons,
            track_play_buttons,
            playlist_list_breadcrumb,
            this_playlist_breadcrumb,
            track_scroll,
            playlist_id,
        } => {
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

            let greatest_play_count = playlist.tracks
                .iter()
                .map(|track| library.get_track(track).augmented.play_count)
                .max()
                .unwrap_or(0);

            let mut column = Column::new()
                .spacing(10)
                .push(h2("Playlist"))
                .push(
                    Container::new(
                    line_row()
                      .push(
                          components::compute_playlist_thumbnail(&library, &playlist.tracks, components::PlaylistIconSize::Large)
                      )
                      .push(
                          Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(h1(playlist.name.clone()))
                            .push(bright_paragraph(common::format_duration(model::compute_track_list_duration(&full_tracks).as_secs())))
                            .push(
                                Row::new()
                                    .push(
                                        dark_button(
                                            &mut playlist_play_queue_buttons.play_button,
                                            bright_paragraph("> Play All"),
                                        )
                                        .on_press(
                                            Message::PlaybackRequest(message::PlaybackRequest::PlaySongs(
                                                if should_shuffle {
                                                    shuffle::shuffle(augmented_tracks.clone())
                                                } else {
                                                    augmented_tracks.clone()
                                                }
                                            )),
                                        ),
                                    )
                                    .push(
                                        dark_button(
                                            &mut playlist_play_queue_buttons.insert_button,
                                            bright_paragraph(">| Insert All Next"),
                                        )
                                        .on_press(
                                            Message::PlaybackRequest(
                                                message::PlaybackRequest::InsertSongs(
                                                    if should_shuffle {
                                                        shuffle::shuffle(augmented_tracks.clone())
                                                    } else {
                                                        augmented_tracks.clone()
                                                    },
                                                    false,
                                                ),
                                            ),
                                        ),
                                    )
                                    .push(
                                        dark_button(
                                            &mut playlist_play_queue_buttons.append_button,
                                            bright_paragraph("|> Append All"),
                                        )
                                        .on_press(
                                            Message::PlaybackRequest(
                                                message::PlaybackRequest::AppendSongs(
                                                    if should_shuffle {
                                                        shuffle::shuffle(augmented_tracks.clone())
                                                    } else {
                                                        augmented_tracks.clone()
                                                    },
                                                    false,
                                                ),
                                            ),
                                        ),
                                    )
                          )
                      )
                      .push(
                           Row::new()
                               .push(
                                   Checkbox::new(
                                       should_shuffle,
                                       "",
                                       |_| Message::Action(message::Action::ToggleShuffleOnAdd),
                                   )
                               )
                               .push(
                                   bright_paragraph("Shuffle (on add)")
                               )
                      )
                      )
                      .padding(10)
                      .width(Length::Fill)
                      .style(
                          style::ContainerPopForward,
                      )
                );
            let mut stripe_marker = true;
            let current_track = match player_info.current_playback {
                Some(ref o) => match o {
                    state::CurrentPlayback::Track(ref v) => Some(v.track.clone()),
                    _ => None,
                }
                None => None,
            };

            let mut tracks_column = Column::new();

            for (
                track_id,
                state::PlaylistTrackLineItemButtons {
                    play_button,
                    link_button,
                    remove_from_playlist_button,
                    move_down_in_playlist_button,
                    move_up_in_playlist_button,
                    insert_button,
                    append_button,
                },
            ) in playlist.tracks.iter().zip(track_play_buttons.iter_mut())
            {
                stripe_marker = !stripe_marker;
                let track = library.get_track(&track_id);
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
                        .push(dark_button(play_button, bright_paragraph("> ")).on_press(
                            Message::PlaybackRequest(message::PlaybackRequest::PlaySongs(
                                vec![track.clone()],
                            )),
                        ))
                        .push(
                            dark_button(
                                link_button,
                                bright_paragraph(track.metadata.title.clone()),
                            )
                            .on_press(
                                components::track_link(&track.metadata)
                            )
                            .width(Length::Fill)
                        )
                        .push(
                            line_row()
                                .spacing(5)
                                .push(
                                    bright_paragraph(track.augmented.play_count.to_string())
                                        .width(Length::Units(40)),
                                )
                                .push(
                                    ProgressBar::new(0.0..=(greatest_play_count as f32), track.augmented.play_count as f32).width(Length::Units(50)),
                                )
                                .push(
                                    dark_button(insert_button, bright_paragraph(">|"))
                                        .on_press(Message::PlaybackRequest(
                                            message::PlaybackRequest::InsertSongs(
                                                vec![track.clone()],
                                                false,
                                            ),
                                        )),
                                )
                                .push(
                                    dark_button(append_button, bright_paragraph("|>"))
                                        .on_press(Message::PlaybackRequest(
                                            message::PlaybackRequest::AppendSongs(
                                                vec![track.clone()],
                                                false,
                                            ),
                                        )),
                                ),
                        )
                        .push({
                            let text_to_show =
                                common::format_duration(track.metadata.duration.as_secs());

                            match player_info.get_maybe_current_playback_track() {
                                Some(c) if (*track == c.clone()) => {
                                    bright_paragraph(text_to_show)
                                }
                                _ => dark_paragraph(text_to_show),
                            }
                            .width(Length::Units(60))
                            .horizontal_alignment(iced::HorizontalAlignment::Right)
                        })
                        .push(
                            line_row()
                                .push(
                                    dark_button(
                                        move_up_in_playlist_button,
                                        bright_paragraph("^"),
                                    )
                                    .on_press(
                                        Message::Action(
                                            message::Action::MoveTrackInPlaylist(
                                                *playlist_id,
                                                message::Direction::Up,
                                                musiqlibrary::TrackUniqueIdentifier::from_track(
                                                    &track.metadata,
                                                ),
                                            ),
                                        ),
                                    ),
                                )
                                .push(
                                    dark_button(
                                        move_down_in_playlist_button,
                                        bright_paragraph("v"),
                                    )
                                    .on_press(
                                        Message::Action(
                                            message::Action::MoveTrackInPlaylist(
                                                *playlist_id,
                                                message::Direction::Down,
                                                musiqlibrary::TrackUniqueIdentifier::from_track(
                                                    &track.metadata,
                                                ),
                                            ),
                                        ),
                                    ),
                                )

                                .push(
                                    dark_button(
                                        remove_from_playlist_button,
                                        bright_paragraph(" - "),
                                    )
                                    .on_press(
                                        Message::Action(
                                            message::Action::RemoveTrackFromPlaylist(
                                                *playlist_id,
                                                musiqlibrary::TrackUniqueIdentifier::from_track(
                                                    &track.metadata,
                                                ),
                                            ),
                                        ),
                                    ),
                                )
                        )
                        .push(Space::with_width(Length::Units(5))),
                )
                .style(style::get_potential_current_stripe_style(
                    stripe_marker,
                    &track,
                    &current_track,
                    &None,
                ));
                tracks_column = tracks_column.push(row);
            }

            column = column.push(Scrollable::new(track_scroll).push(tracks_column));
            (
                vec![
                    (
                        playlist_list_breadcrumb,
                        "Playlists".to_string(),
                        user_nav_message(NavMessage::PlaylistList("".to_string())),
                    ),
                    (
                        this_playlist_breadcrumb,
                        playlist.name.clone(),
                        user_nav_message(NavMessage::PlaylistView(playlist_id.clone())),
                    ),
                ],
                Container::new(column)
            )
        },
    }
}
