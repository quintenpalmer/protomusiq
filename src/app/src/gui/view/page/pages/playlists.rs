use std::collections::BTreeMap;

use iced::{self, button, Column, Container, Length, Row, Scrollable, Space, TextInput};

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;
use super::super::super::style;

pub fn playlist_list_view<'a>(
    library: &'a model::LibraryState,
    state: &'a mut state::PlaylistListState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::PlaylistListState {
            playlist_scroll,
            playlist_list_breadcrumb,
            playlist_make_default_buttons,
            new_playlist_name,
            new_playlist_text_input,
            new_playlist_button,
        } => (
            vec![(
                playlist_list_breadcrumb,
                "Playlists".to_string(),
                user_nav_message(NavMessage::PlaylistList("".to_string())),
            )],
            {
                let mut playlists = Column::new().push(h1("Playlists"));
                let mut stripe_marker = false;
                for (
                    user_playlist,
                    state::PlaylistListButtons {
                        link_to_playlist_button,
                        delete_playlist_button,
                        make_default_button,
                    },
                ) in library
                    .user_playlists
                    .to_vec()
                    .iter()
                    .zip(playlist_make_default_buttons.iter_mut())
                    .rev()
                {
                    let mut row = line_row();
                    row = row.push(
                        Row::new()
                            .push(
                                compute_playlist_thumbnail(&library, &user_playlist.tracks)
                            )
                            .push(Column::new()
                                .push(dark_button(
                                    link_to_playlist_button,
                                    h2(user_playlist.name.clone()),
                                )
                                .on_press(user_nav_message(
                                    message::NavMessage::PlaylistView(user_playlist.id),
                                )))
                                .push(
                                    bright_paragraph(format!("{} tracks", user_playlist.tracks.len()))
                                )
                            )
                            .align_items(iced::Align::Center),
                    );
                    row = row.push(Space::with_width(Length::Fill));
                    if library.user_playlists.is_default_playlist(user_playlist.id) {
                        row = row.push(bright_paragraph("-"));
                        row = row.push(bright_paragraph("* (selected)").width(Length::Units(150)));
                    } else {
                        row = row.push(
                            dark_button(delete_playlist_button, bright_paragraph("-")).on_press(
                                message::Message::Action(message::Action::DeletePlaylist(
                                    user_playlist.id,
                                )),
                            ),
                        );
                        row = row.push(
                            dark_button(make_default_button, bright_paragraph("Make\nDefault"))
                                .on_press(message::Message::Action(
                                    message::Action::MakePlaylistDefault(user_playlist.id),
                                ))
                                .width(Length::Units(150)),
                        );
                    }

                    stripe_marker = !stripe_marker;
                    let striped_container = Container::new(row)
                        .style(style::get_stripe_style(
                            stripe_marker,
                        ));

                    playlists = playlists.push(striped_container);
                }
                playlists = playlists.push(
                    Row::new()
                        .push(
                            TextInput::new(
                                new_playlist_text_input,
                                "Create New Playlist...",
                                new_playlist_name,
                                |s| Message::Action(message::Action::UpdateText(s)),
                            )
                            .on_submit(message::Message::Action(
                                message::Action::CreateNewPlaylist(new_playlist_name.clone()),
                            )),
                        )
                        .push(
                            dark_button(new_playlist_button, bright_paragraph("+")).on_press(
                                message::Message::Action(message::Action::CreateNewPlaylist(
                                    new_playlist_name.clone(),
                                )),
                            ),
                        ),
                );
                Container::new(Scrollable::new(playlist_scroll).push(playlists))
            },
        ),
    }
}

pub fn compute_playlist_thumbnail<'a>(
    library: &'a model::LibraryState,
    tracks: &Vec<musiqlibrary::TrackUniqueIdentifier>,
) -> iced::Container<'a, Message> {
    if tracks.len() > 0 {
        let mut album_counts = BTreeMap::new();
        for track in tracks.iter() {
            *album_counts
                .entry(musiqlibrary::AlbumUniqueIdentifier::new(track.artist_id.clone(), track.album_id.clone()))
                .or_insert(0) += 1;
        }

        let mut counts_for_album = BTreeMap::new();
        for (key, count) in album_counts.into_iter() {
            counts_for_album
                .entry(count)
                .or_insert(Vec::new()).push(key)
        }

        let mut keys_sorted_by_count = Vec::new();
        for (_count, mut keys) in counts_for_album.into_iter().rev() {
            keys_sorted_by_count.append(&mut keys);
        }

        let first_key = keys_sorted_by_count[0].clone();

        let second_key = if keys_sorted_by_count.len() > 1 {
            Some(keys_sorted_by_count[1].clone())
        } else {
            None
        };

        let third_key = if keys_sorted_by_count.len() > 2 {
            Some(keys_sorted_by_count[2].clone())
        } else {
            None
        };

        let fourth_key = if keys_sorted_by_count.len() > 3 {
            Some(keys_sorted_by_count[3].clone())
        } else {
            None
        };

        let row = match (&second_key, &third_key, &fourth_key) {
            (None, None, None) => {
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Mini,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Mini,
                ));
                Column::new().spacing(3).push(row)
            },
            (Some(second_key), None, None) => {
                let mut column = Column::new()
                    .spacing(3);
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                row = row.push(empty_album_space(model::AlbumSize::Micro));
                column = column.push(row);

                let mut second_row = Row::new()
                    .spacing(3);
                second_row = second_row.push(empty_album_space(model::AlbumSize::Micro));
                second_row = second_row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        second_key.artist_id.clone(),
                        second_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                column = column.push(second_row);

                column
            }
            _ => {
                let mut column = Column::new()
                    .spacing(3);
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                match second_key {
                    Some(second_key) => {
                        row = row.push(album_image(
                            library.get_album_cover(
                                model::AlbumSize::Micro,
                                second_key.artist_id.clone(),
                                second_key.album_id.clone(),
                            ),
                            model::AlbumSize::Micro,
                        ));

                        column = column.push(row);

                        match third_key {
                            Some(third_key) => {
                            let mut second_row = Row::new()
                                .spacing(3);
                                second_row = second_row.push(album_image(
                                    library.get_album_cover(
                                        model::AlbumSize::Micro,
                                        third_key.artist_id.clone(),
                                        third_key.album_id.clone(),
                                    ),
                                    model::AlbumSize::Micro,
                                ));

                                match fourth_key {
                                    Some(fourth_key) => {
                                        second_row = second_row.push(album_image(
                                            library.get_album_cover(
                                                model::AlbumSize::Micro,
                                                fourth_key.artist_id.clone(),
                                                fourth_key.album_id.clone(),
                                            ),
                                            model::AlbumSize::Micro,
                                        ));

                                        column = column.push(second_row);
                                    }
                                    None => {
                                        column = column.push(second_row);
                                    },
                                }
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
                column
            }
        };

        Container::new(row)
    } else {
        Container::new(bright_paragraph("<?>"))
    }
    .align_x(iced::Align::Center)
    .align_y(iced::Align::Center)
    .width(Length::Units(80))
    .height(Length::Units(80))
}

fn empty_album_space(album_size: model::AlbumSize) -> iced::Space {
    iced::Space::new(
        iced::Length::Units(album_size.width()),
        iced::Length::Units(album_size.height()),
    )
}
