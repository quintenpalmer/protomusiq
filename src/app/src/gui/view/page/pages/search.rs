use iced::{self, button, Column, Container, Length, Row, Scrollable, TextInput};

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;

pub fn search_page<'a>(
    library: &'a model::LibraryState,
    state: &'a mut state::SearchPageState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::SearchPageState {
            query,
            artist_scroll,
            album_scroll,
            track_scroll,
            track_artist_scroll,
            input_state,
            results,
        } => {
            let breadcrumbs = Vec::new();
            let body = {
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Search your Library"))
                        .push(
                            Row::new()
                                .push(
                                    TextInput::new(input_state, "Search...", query, |s| {
                                        Message::Action(message::Action::UpdateText(s))
                                    })
                                    .on_submit(Message::Action(
                                        message::Action::PerformSearch(query.clone()),
                                    )),
                                )
                                .width(Length::Fill),
                        )
                        .push(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Column::new()
                                        .push(h2("Artists"))
                                        .push(Scrollable::new(artist_scroll).push(
                                            results.artists.iter_mut().fold(
                                                Column::new(),
                                                |column, result| {
                                                    column.push(
                                                        dark_button(
                                                            &mut result.second,
                                                            line_row()
                                                                .spacing(5)
                                                                .push(album_image(
                                                                    library
                                                                        .get_artists_first_album_cover(
                                                                            model::AlbumSize::Micro,
                                                                            result
                                                                                .first
                                                                                .artist_id
                                                                                .clone(),
                                                                        ),
                                                                    model::AlbumSize::Micro,
                                                                ))
                                                                .push(
                                                                    bright_paragraph(
                                                                        result
                                                                            .first
                                                                            .artist_name
                                                                            .clone(),
                                                                    )
                                                                    .width(Length::Fill),
                                                                ),
                                                        )
                                                        .on_press(user_nav_message(
                                                            NavMessage::ArtistView(
                                                                result.first.artist_id.clone(),
                                                            ),
                                                        )),
                                                    )
                                                },
                                            ),
                                        ))
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Albums"))
                                        .push(Scrollable::new(album_scroll).push(
                                            results.albums.iter_mut().fold(
                                                Column::new(),
                                                |column, result| {
                                                    column.push(
                                                        dark_button(
                                                            &mut result.second,
                                                            line_row()
                                                                .spacing(5)
                                                                .push(album_image(
                                                                    library.get_album_cover(
                                                                        model::AlbumSize::Micro,
                                                                        result
                                                                            .first
                                                                            .artist
                                                                            .artist_id
                                                                            .clone(),
                                                                        result
                                                                            .first
                                                                            .album
                                                                            .album_id
                                                                            .clone(),
                                                                    ),
                                                                    model::AlbumSize::Micro,
                                                                ))
                                                                .push(
                                                                    bright_paragraph(
                                                                        result
                                                                            .first
                                                                            .album
                                                                            .album_name
                                                                            .clone(),
                                                                    )
                                                                    .width(Length::Fill),
                                                                ),
                                                        )
                                                        .on_press(user_nav_message(
                                                            NavMessage::ArtistAlbumView(
                                                                result.first.artist.artist_id.clone(),
                                                                result.first.album.album_id.clone(),
                                                                model::AlbumSize::Regular,
                                                            ),
                                                        )),
                                                    )
                                                },
                                            ),
                                        ))
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Tracks"))
                                        .push(Scrollable::new(track_scroll).push(
                                            results.tracks.iter_mut().fold(
                                                Column::new(),
                                                |column, result| {
                                                    column.push(
                                                        dark_button(
                                                            &mut result.second,
                                                            line_row()
                                                                .spacing(5)
                                                                .push(album_image(
                                                                    library.get_album_cover(
                                                                        model::AlbumSize::Micro,
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .album_artist_id
                                                                            .clone(),
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .album_id
                                                                            .clone(),
                                                                    ),
                                                                    model::AlbumSize::Micro,
                                                                ))
                                                                .push(
                                                                    bright_paragraph(
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .title
                                                                            .clone(),
                                                                    )
                                                                    .width(Length::Fill),
                                                                ),
                                                        )
                                                        .on_press(user_nav_message(
                                                            NavMessage::ArtistAlbumView(
                                                                result
                                                                    .first
                                                                    .metadata
                                                                    .album_artist_id
                                                                    .clone(),
                                                                result.first.metadata.album_id.clone(),
                                                                model::AlbumSize::Regular,
                                                            ),
                                                        )),
                                                    )
                                                },
                                            ),
                                        ))
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Artist for Track"))
                                        .push(Scrollable::new(track_artist_scroll).push(
                                            results.track_artists.iter_mut().fold(
                                                Column::new(),
                                                |column, result| {
                                                    column.push(
                                                        dark_button(
                                                            &mut result.second,
                                                            line_row()
                                                                .spacing(5)
                                                                .push(album_image(
                                                                    library.get_album_cover(
                                                                        model::AlbumSize::Micro,
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .album_artist_id
                                                                            .clone(),
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .album_id
                                                                            .clone(),
                                                                    ),
                                                                    model::AlbumSize::Micro,
                                                                ))
                                                                .push(
                                                                    bright_paragraph(format!(
                                                                        "{} ({})",
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .title
                                                                            .clone(),
                                                                        result
                                                                            .first
                                                                            .metadata
                                                                            .track_artist
                                                                            .clone(),
                                                                    ))
                                                                    .width(Length::Fill),
                                                                ),
                                                        )
                                                        .on_press(user_nav_message(
                                                            NavMessage::ArtistAlbumView(
                                                                result
                                                                    .first
                                                                    .metadata
                                                                    .album_artist_id
                                                                    .clone(),
                                                                result.first.metadata.album_id.clone(),
                                                                model::AlbumSize::Regular,
                                                            ),
                                                        )),
                                                    )
                                                },
                                            ),
                                        ))
                                        .width(Length::FillPortion(1)),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill),
                        ),
                )
            };
            (breadcrumbs, body)
        }
    }
}
