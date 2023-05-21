use iced::widget::{Column, Container, Row, Scrollable, TextInput};
use iced::Length;

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use crate::gui::view::components;

use super::super::super::elements::*;
use super::super::super::style;

pub fn playlist_list_view<'a>(
    library: &'a model::LibraryState,
    state: &'a state::PlaylistListState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::PlaylistListState { new_playlist_name } => (
            vec![(
                "Playlists".to_string(),
                user_nav_message(NavMessage::PlaylistList("".to_string())),
            )],
            {
                let mut page = Column::new().push(h1("Playlists"));

                page = page.push(
                    Row::new()
                        .push(
                            TextInput::new("Create New Playlist...", new_playlist_name)
                                .on_input(|s| Message::Action(message::Action::UpdateText(s)))
                                .on_submit(message::Message::Action(
                                    message::Action::CreateNewPlaylist(new_playlist_name.clone()),
                                )),
                        )
                        .push(dark_button(bright_paragraph("+")).on_press(
                            message::Message::Action(message::Action::CreateNewPlaylist(
                                new_playlist_name.clone(),
                            )),
                        )),
                );

                let mut playlists = Column::new();
                let mut stripe_marker = false;
                for user_playlist in library.user_playlists.to_vec().iter().rev() {
                    let mut row = line_row();
                    row = row.push(
                        dark_button(
                            Row::new()
                                .push(components::compute_playlist_thumbnail(
                                    &library,
                                    &user_playlist.tracks,
                                    components::PlaylistIconSize::Small,
                                ))
                                .push(Column::new().push(h2(user_playlist.name.clone())).push(
                                    bright_paragraph(format!(
                                        "{} tracks",
                                        user_playlist.tracks.len()
                                    )),
                                ))
                                .align_items(iced::Alignment::Center),
                        )
                        .on_press(user_nav_message(message::NavMessage::PlaylistView(
                            user_playlist.id,
                        )))
                        .padding(8)
                        .width(Length::Fill),
                    );
                    if library.user_playlists.is_default_playlist(user_playlist.id) {
                        row = row.push(bright_paragraph("*").width(Length::Fixed(15.0)));
                        row = row.push(bright_paragraph("(selected)").width(Length::Fixed(70.0)));
                    } else {
                        row = row.push(
                            dark_button(bright_paragraph("-"))
                                .on_press(message::Message::Action(
                                    message::Action::DeletePlaylist(user_playlist.id),
                                ))
                                .width(Length::Fixed(15.0)),
                        );
                        row = row.push(
                            dark_button(bright_paragraph("Make\nDefault"))
                                .on_press(message::Message::Action(
                                    message::Action::MakePlaylistDefault(user_playlist.id),
                                ))
                                .width(Length::Fixed(70.0)),
                        );
                    }

                    stripe_marker = !stripe_marker;
                    let striped_container = Container::new(row).style(
                        iced::theme::Container::Custom(style::get_stripe_style(stripe_marker)),
                    );

                    playlists = playlists.push(striped_container);
                }
                page = page.push(Scrollable::new(playlists));

                Container::new(page)
            },
        ),
    }
}
