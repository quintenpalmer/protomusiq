use iced::{self, button, Column, Container, Length, Row, Scrollable, Space, TextInput};

use crate::model;

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;
use super::super::super::components;

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
            search_breadcrumb,
            search_result_breadcrumb,
            artist_scroll,
            album_scroll,
            track_scroll,
            track_artist_scroll,
            input_state,
            results,
        } => {
            let mut breadcrumbs = vec![(
                    search_breadcrumb,
                    "Search".to_string(),
                    user_nav_message(NavMessage::SearchPage("".to_string(), false)),
                )];
            match results {
                Some(_) => {
                    breadcrumbs.push(
                        (
                            search_result_breadcrumb,
                            query.clone(),
                            user_nav_message(NavMessage::SearchPage(query.clone(), true))
                        )
                    );
                },
                _ => (),
            };

            let (album_results, artist_results, track_results, featured_artist_results) = match results {
                Some(results) => {
                    let album_results = Scrollable::new(album_scroll).push(
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
                                                                None,
                                                            ),
                                                        )),
                                                    )
                                                },
                                            ),
                                        );
                    let artist_results = Scrollable::new(artist_scroll).push(
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
                                        );
                    let track_results = Scrollable::new(track_scroll).push(
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
                                                        .on_press(
                                                            components::track_link(&result.first.metadata)
                                                        ),
                                                    )
                                                },
                                            ),
                                        );
                    let featured_artist_results = Scrollable::new(track_artist_scroll).push(
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
                                                        .on_press(
                                                            components::track_link(&result.first.metadata)
                                                        ),
                                                    )
                                                },
                                            ),
                                        );
                    (album_results, artist_results, track_results, featured_artist_results)
                },
                None => (
                    Scrollable::new(album_scroll).push(Space::with_width(Length::Fill)),
                    Scrollable::new(artist_scroll).push(Space::with_width(Length::Fill)),
                    Scrollable::new(track_scroll).push(Space::with_width(Length::Fill)),
                    Scrollable::new(track_artist_scroll).push(Space::with_width(Length::Fill)),
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
