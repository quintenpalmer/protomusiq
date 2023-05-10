use iced::{self, button, Column, Container, Length, ProgressBar, Row, Scrollable, Space};

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state::{self, PlayerInfoState};

use crate::gui::view::components;

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

pub fn playlist_view<'a>(
    library: &'a model::LibraryState,
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

            let greatest_play_count = playlist.tracks
                .iter()
                .map(|track| library.get_track(track).augmented.play_count)
                .max()
                .unwrap_or(0);

            let mut column = Column::new()
                .spacing(10)
                .push(h1("Playlist"))
                .push(
                    line_row()
                      .push(
                          components::compute_playlist_thumbnail(&library, &playlist.tracks)
                      )
                      .push(h2(playlist.name.clone()))
                )
                .push(
                    Row::new()
                        .push(
                            dark_button(
                                &mut playlist_play_queue_buttons.play_button,
                                bright_paragraph("> Play All Songs"),
                            )
                            .on_press(
                                Message::PlaybackRequest(message::PlaybackRequest::PlaySongs(
                                    playlist
                                        .tracks
                                        .iter()
                                        .map(|track_id| library.get_track(&track_id).clone())
                                        .collect(),
                                )),
                            ),
                        )
                        .push(
                            dark_button(
                                &mut playlist_play_queue_buttons.insert_button,
                                bright_paragraph(">| Insert All into Play Queue"),
                            )
                            .on_press(
                                Message::PlaybackRequest(
                                    message::PlaybackRequest::InsertSongs(
                                        playlist
                                            .tracks
                                            .iter()
                                            .map(|track_id| {
                                                library.get_track(&track_id).clone()
                                            })
                                            .collect(),
                                        false,
                                    ),
                                ),
                            ),
                        )
                        .push(
                            dark_button(
                                &mut playlist_play_queue_buttons.append_button,
                                bright_paragraph("|> Append All to Play Queue"),
                            )
                            .on_press(
                                Message::PlaybackRequest(
                                    message::PlaybackRequest::AppendSongs(
                                        playlist
                                            .tracks
                                            .iter()
                                            .map(|track_id| {
                                                library.get_track(&track_id).clone()
                                            })
                                            .collect(),
                                        false,
                                    ),
                                ),
                            ),
                        ),
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
                            bright_paragraph(track.metadata.title.clone()).width(Length::Fill),
                        )
                        .push(
                            line_row()
                                .spacing(5)

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
                                        bright_paragraph("-"),
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
