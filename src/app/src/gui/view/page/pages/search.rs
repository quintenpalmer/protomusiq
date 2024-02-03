use iced::widget::{Column, Container, Row, Scrollable, Space, TextInput};
use iced::Length;

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::components;
use super::super::super::elements::*;

pub fn search_page<'a>(
    library: &'a model::LibraryState,
    state: &'a state::SearchPageState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::SearchPageState { query, results } => {
            let mut breadcrumbs = vec![(
                "Search".to_string(),
                user_nav_message(NavMessage::SearchPage("".to_string(), false)),
            )];
            match results {
                Some(_) => {
                    breadcrumbs.push((
                        format!("\"{}\"", query.clone()),
                        user_nav_message(NavMessage::SearchPage(query.clone(), true)),
                    ));
                }
                _ => (),
            };

            let (album_results, artist_results, track_results, featured_artist_results) =
                match results {
                    Some(results) => {
                        let album_results = Scrollable::new(results.albums.iter().fold(
                            Column::new(),
                            |column, result| {
                                column.push(
                                    dark_button(
                                        line_row()
                                            .spacing(5)
                                            .push(album_image(
                                                library.get_album_cover(
                                                    model::AlbumSize::Micro,
                                                    result.artist.artist_id.clone(),
                                                    result.album.album_id.clone(),
                                                ),
                                                model::AlbumSize::Micro,
                                            ))
                                            .push(
                                                bright_paragraph(result.album.album_name.clone())
                                                    .width(Length::Fill),
                                            ),
                                    )
                                    .on_press(
                                        user_nav_message(NavMessage::ArtistAlbumView(
                                            result.artist.artist_id.clone(),
                                            result.album.album_id.clone(),
                                            model::AlbumSize::Regular,
                                            None,
                                        )),
                                    ),
                                )
                            },
                        ));
                        let artist_results = Scrollable::new(results.artists.iter().fold(
                            Column::new(),
                            |column, result| {
                                column.push(
                                    dark_button(
                                        line_row()
                                            .spacing(5)
                                            .push(album_image(
                                                library.get_artists_first_album_cover(
                                                    model::AlbumSize::Micro,
                                                    result.artist_id.clone(),
                                                ),
                                                model::AlbumSize::Micro,
                                            ))
                                            .push(
                                                bright_paragraph(result.artist_name.clone())
                                                    .width(Length::Fill),
                                            ),
                                    )
                                    .on_press(
                                        user_nav_message(NavMessage::ArtistAlbumsView(
                                            result.artist_id.clone(),
                                        )),
                                    ),
                                )
                            },
                        ));
                        let track_results = Scrollable::new(results.tracks.iter().fold(
                            Column::new(),
                            |column, result| {
                                column.push(
                                    dark_button(
                                        line_row()
                                            .spacing(5)
                                            .push(album_image(
                                                library.get_album_cover(
                                                    model::AlbumSize::Micro,
                                                    result.metadata.album_artist_id.clone(),
                                                    result.metadata.album_id.clone(),
                                                ),
                                                model::AlbumSize::Micro,
                                            ))
                                            .push(
                                                bright_paragraph(result.metadata.title.clone())
                                                    .width(Length::Fill),
                                            ),
                                    )
                                    .on_press(components::track_link(&result.metadata)),
                                )
                            },
                        ));
                        let featured_artist_results =
                            Scrollable::new(results.track_artists.iter().fold(
                                Column::new(),
                                |column, result| {
                                    column.push(
                                        dark_button(
                                            line_row()
                                                .spacing(5)
                                                .push(album_image(
                                                    library.get_album_cover(
                                                        model::AlbumSize::Micro,
                                                        result.metadata.album_artist_id.clone(),
                                                        result.metadata.album_id.clone(),
                                                    ),
                                                    model::AlbumSize::Micro,
                                                ))
                                                .push(
                                                    bright_paragraph(format!(
                                                        "{} ({})",
                                                        result.metadata.title.clone(),
                                                        result.metadata.track_artist.clone(),
                                                    ))
                                                    .width(Length::Fill),
                                                ),
                                        )
                                        .on_press(components::track_link(&result.metadata)),
                                    )
                                },
                            ));
                        (
                            album_results,
                            artist_results,
                            track_results,
                            featured_artist_results,
                        )
                    }
                    None => (
                        Scrollable::new(Space::with_width(Length::Fill)),
                        Scrollable::new(Space::with_width(Length::Fill)),
                        Scrollable::new(Space::with_width(Length::Fill)),
                        Scrollable::new(Space::with_width(Length::Fill)),
                    ),
                };

            let body = {
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(h1("Search your Library"))
                        .push(
                            Row::new()
                                .push(
                                    TextInput::new("Search...", query)
                                        .on_input(|s| {
                                            Message::Action(message::Action::UpdateText(s))
                                        })
                                        .on_submit(Message::Action(message::Action::PerformSearch(
                                            query.clone(),
                                        )))
                                        .id(state::TEXT_INPUT_ID.clone()),
                                )
                                .width(Length::Fill),
                        )
                        .push(
                            Row::new()
                                .spacing(5)
                                .push(
                                    Column::new()
                                        .push(h2("Artists"))
                                        .push(artist_results)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Albums"))
                                        .push(album_results)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Tracks"))
                                        .push(track_results)
                                        .width(Length::FillPortion(1)),
                                )
                                .push(
                                    Column::new()
                                        .push(h2("Artist for Track"))
                                        .push(featured_artist_results)
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
