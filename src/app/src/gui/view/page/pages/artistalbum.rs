use iced::{self, button, Checkbox, Column, Container, Length, ProgressBar, Row, Scrollable, Space};

use musiqlibrary;

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state::{self, ActionState, PlayerInfoState};

use super::super::super::common;
use super::super::super::elements::*;
use super::super::super::style;

use super::super::consts;

pub fn artist_album_view_state<'a>(
    library: &'a model::LibraryState,
    action_state: &'a ActionState,
    player_info: &'a PlayerInfoState,
    state: &'a mut state::ArtistAlbumViewState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::ArtistAlbumViewState {
            artist_list_breadcrumb,
            artist_view_breadcrumb,
            artist_album_view_breadcrumb,
            artist_id,
            album_id,
            album_size,
            maybe_selected_track,
            toggle_image_size_button,
            entire_track_list_buttons,
            all_disc_buttons,
            track_play_buttons,
            scroll,
        } => {
            let artist = library.get_artist_map().get(&artist_id).unwrap();
            let album = artist.albums.get(&album_id).unwrap();
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
                (
                    artist_album_view_breadcrumb,
                    common::abr_str(album.album_info.album_name.clone(), consts::NAV_STR_LENGTH),
                    user_nav_message(NavMessage::ArtistAlbumView(
                        artist_id.clone(),
                        album_id.clone(),
                        model::AlbumSize::Regular,
                        None,
                    )),
                ),
            ];

            let body = Container::new(
                Column::new()
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
                                dark_button(toggle_image_size_button, album_image(
                                    library.get_album_cover(
                                        current.clone(),
                                        artist.artist_info.artist_id.clone(),
                                        album.album_info.album_id.clone(),
                                    ),
                                    current,
                                )).on_press(user_nav_message(NavMessage::ArtistAlbumView(
                                    artist_id.clone(),
                                    album_id.clone(),
                                    toggle_to,
                                    maybe_selected_track.clone(),
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
                                                    &mut entire_track_list_buttons.play_button,
                                                    bright_paragraph("> Play All"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    message::PlaybackRequest::PlaySongs(
                                                        tracks
                                                            .iter()
                                                            .map(|track| track.clone())
                                                            .collect(),
                                                    ),
                                                )),
                                            )
                                            .push(
                                                dark_button(
                                                    &mut entire_track_list_buttons.insert_button,
                                                    bright_paragraph(">| Insert All Next"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    message::PlaybackRequest::InsertSongs(
                                                        tracks
                                                            .iter()
                                                            .map(|track| track.clone())
                                                            .collect(),
                                                        false,
                                                    ),
                                                )),
                                            )
                                            .push(
                                                dark_button(
                                                    &mut entire_track_list_buttons.append_button,
                                                    bright_paragraph("|> Append All"),
                                                )
                                                .on_press(Message::PlaybackRequest(
                                                    message::PlaybackRequest::AppendSongs(
                                                        tracks
                                                            .iter()
                                                            .map(|track| track.clone())
                                                            .collect(),
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
                                                        let default_playlist = library.user_playlists.get(default_playlist_id).unwrap();
                                                        Container::new(dark_button(
                                                                &mut entire_track_list_buttons.add_to_default_playlist_button,
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
                                                            should_shuffle,
                                                            "",
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
                    .push(Space::new(Length::Fill, Length::Units(20)))
                    .push({
                        let mut column = Column::new().padding(15);
                        let mut stripe_marker = false;
                        let disc_count = discs.len();

                        for (disc, (disc_buttons, tracks_buttons)) in
                            discs.into_iter().zip(all_disc_buttons.iter_mut().zip(track_play_buttons.iter_mut()))
                        {
                            if disc_count > 1 {
                                column = column.push(
                                        Row::new()
                                            .push(h2(format!("Disc {}", disc.disc_no)).width(Length::Fill))
                                            .push(
                                                Row::new()
                                                    .push(
                                                        dark_button(
                                                            &mut disc_buttons.play_button,
                                                            bright_paragraph(">"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            message::PlaybackRequest::PlaySongs(
                                                                disc.tracks.values()
                                                                    .map(|track| track.clone())
                                                                    .collect(),
                                                            ),
                                                        )),
                                                    )
                                                    .push(
                                                        dark_button(
                                                            &mut disc_buttons.insert_button,
                                                            bright_paragraph(">|"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            message::PlaybackRequest::InsertSongs(
                                                                disc.tracks.values()
                                                                    .map(|track| track.clone())
                                                                    .collect(),
                                                                false,
                                                            ),
                                                        )),
                                                    )
                                                    .push(
                                                        dark_button(
                                                            &mut disc_buttons.append_button,
                                                            bright_paragraph("|>"),
                                                        )
                                                        .on_press(Message::PlaybackRequest(
                                                            message::PlaybackRequest::AppendSongs(
                                                                disc.tracks.values()
                                                                    .map(|track| track.clone())
                                                                    .collect(),
                                                                false,
                                                            ),
                                                        )),
                                                    )
                                                    .push(
                                                        match library.user_playlists.get_default_playlist_id() {
                                                            Some(default_playlist_id) => Container::new(dark_button(
                                                                    &mut disc_buttons.add_to_default_playlist_button,
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
                                                    Space::with_width(Length::Units(60))
                                                ),
                                );
                            }
                            for (track, state::TrackLineItemButtons{
                                    play_button,
                                    play_all_from_here_button,
                                    add_to_default_playlist_button,
                                    insert_button,
                                    append_button,
                                }) in
                                disc.tracks.values().zip(tracks_buttons)
                            {
                                stripe_marker = !stripe_marker;

                                let track_maybe_with_track_artist = if
                                        track.metadata.album_artist == track.metadata.track_artist
                                    {
                                        track.metadata.title.clone()
                                    } else {
                                        format!("{} ({})", track.metadata.title, track.metadata.track_artist)
                                    };

                                column = column.push(
                                    Container::new(
                                        line_row()
                                            .spacing(5)
                                            .push(
                                                dark_button(play_button, bright_paragraph(">"))
                                                    .on_press(Message::PlaybackRequest(
                                                        message::PlaybackRequest::PlaySongs(
                                                            vec![track.clone()],
                                                        ),
                                                    )),
                                            )
                                            .push(
                                                dark_button(play_all_from_here_button, bright_paragraph(">..."))
                                                    .on_press(Message::PlaybackRequest(
                                                        message::PlaybackRequest::PlaySongs(
                                                            model::tracks_after_including(&tracks, &track),
                                                        ),
                                                    )),
                                            )
                                            .push(
                                                bright_paragraph(track.metadata.track.to_string())
                                                    .width(Length::Units(40)),
                                            )
                                            .push(
                                                bright_paragraph(track_maybe_with_track_artist)
                                                    .width(Length::Fill),
                                            )
                                            .push(
                                                bright_paragraph(track.augmented.play_count.to_string())
                                                    .width(Length::Units(40)),
                                            )
                                            .push(
                                                ProgressBar::new(0.0..=(greatest_play_count as f32), track.augmented.play_count as f32).width(Length::Units(50)),
                                            )
                                            .push(Row::new()
                                                .spacing(0)
                                                .padding(0)
                                                .push(
                                                    dark_button(
                                                        insert_button,
                                                        bright_paragraph(">|"),
                                                    )
                                                    .on_press(Message::PlaybackRequest(
                                                        message::PlaybackRequest::InsertSongs(
                                                            vec![track.clone()],
                                                            false,
                                                        ),
                                                    )),
                                                )
                                                .push(
                                                    dark_button(
                                                        append_button,
                                                        bright_paragraph("|>"),
                                                    )
                                                    .on_press(Message::PlaybackRequest(
                                                        message::PlaybackRequest::AppendSongs(
                                                            vec![track.clone()],
                                                            false,
                                                        ),
                                                    )),
                                                )
                                                .push(
                                                    match library.user_playlists.get_default_playlist_id() {
                                                        Some(default_playlist_id) => Container::new(dark_button(
                                                                add_to_default_playlist_button,
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
                                                }.width(Length::Units(60))
                                                .horizontal_alignment(iced::HorizontalAlignment::Right)
                                            })
                                            .push(Space::with_width(Length::Units(5)))
                                    )
                                    .style(
                                        style::get_potential_current_stripe_style(
                                            stripe_marker,
                                            &track,
                                            &current_track,
                                            &maybe_selected_track,
                                        ),
                                    ),
                                )
                            }
                        }

                        Container::new(Scrollable::new(scroll).push(column))
                    }),
            );

            (breadcrumbs, body)
        }
    }
}
