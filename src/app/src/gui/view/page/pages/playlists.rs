use iced::{self, button, Column, Container, Length, Row, Scrollable, TextInput};

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use crate::gui::view::components;

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
                let mut page = Column::new().push(h1("Playlists"));

                page = page.push(
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

                let mut playlists = Column::new();
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
                        dark_button(
                            link_to_playlist_button,
                            Row::new()
                                .push(
                                    components::compute_playlist_thumbnail(&library, &user_playlist.tracks)
                                )
                                .push(Column::new()
                                    .push(
                                        h2(user_playlist.name.clone()),
                                    )
                                    .push(
                                        bright_paragraph(format!("{} tracks", user_playlist.tracks.len()))
                                    )
                                )
                                .align_items(iced::Align::Center),
                        )
                        .on_press(user_nav_message(
                            message::NavMessage::PlaylistView(user_playlist.id),
                        ))
                        .padding(8)
                        .width(Length::Fill)
                    );
                    if library.user_playlists.is_default_playlist(user_playlist.id) {
                        row = row.push(bright_paragraph("*").width(Length::Units(15)));
                        row = row.push(bright_paragraph("(selected)").width(Length::Units(70)));
                    } else {
                        row = row.push(
                            dark_button(delete_playlist_button, bright_paragraph("-")).on_press(
                                message::Message::Action(message::Action::DeletePlaylist(
                                    user_playlist.id,
                                )),
                            ).width(Length::Units(15)),
                        );
                        row = row.push(
                            dark_button(make_default_button, bright_paragraph("Make\nDefault"))
                                .on_press(message::Message::Action(
                                    message::Action::MakePlaylistDefault(user_playlist.id),
                                ))
                                .width(Length::Units(70)),
                        );
                    }

                    stripe_marker = !stripe_marker;
                    let striped_container = Container::new(row)
                        .style(style::get_stripe_style(
                            stripe_marker,
                        ));

                    playlists = playlists.push(striped_container);
                }
                page = page.push(Scrollable::new(playlist_scroll).push(playlists));

                Container::new(page)
            },
        ),
    }
}
