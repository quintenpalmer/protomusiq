use iced::widget::{Column, Container, Row, Scrollable, Space, TextInput};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::components;
use super::super::super::elements::*;

pub fn search_page<'a>(
    library: &'a model::LibraryState,
    movie_library: &'a model::VideoLibraryState,
    app_images: &embedded::AppImages,
    state: &'a state::SearchPageState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::SearchPageState {
            query,
            domain_results,
        } => {
            let domain = match domain_results {
                state::SearchDomainResults::Music(ref _res) => message::SearchDomain::Music,
                state::SearchDomainResults::Movies(ref _res) => message::SearchDomain::Movies,
            };

            let mut breadcrumbs = vec![(
                "Search".to_string(),
                user_nav_message(NavMessage::SearchPage(
                    "".to_string(),
                    message::SearchDomain::Music,
                    false,
                )),
            )];

            match domain_results {
                state::SearchDomainResults::Music(results) => {
                    match results {
                        Some(_) => {
                            breadcrumbs.push((
                                format!("\"{}\"", query.clone()),
                                user_nav_message(NavMessage::SearchPage(
                                    query.clone(),
                                    message::SearchDomain::Music,
                                    true,
                                )),
                            ));
                        }
                        _ => (),
                    };
                }
                state::SearchDomainResults::Movies(results) => {
                    match results {
                        Some(_) => {
                            breadcrumbs.push((
                                format!("\"{}\"", query.clone()),
                                user_nav_message(NavMessage::SearchPage(
                                    query.clone(),
                                    message::SearchDomain::Movies,
                                    true,
                                )),
                            ));
                        }
                        _ => (),
                    };
                }
            };

            let domain_specific = match domain_results {
                state::SearchDomainResults::Music(results) => {
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
                                                        bright_paragraph(
                                                            result.album.album_name.clone(),
                                                        )
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
                                                        bright_paragraph(
                                                            result.artist_name.clone(),
                                                        )
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
                                                        bright_paragraph(
                                                            result.metadata.title.clone(),
                                                        )
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
                                                                result
                                                                    .metadata
                                                                    .album_artist_id
                                                                    .clone(),
                                                                result.metadata.album_id.clone(),
                                                            ),
                                                            model::AlbumSize::Micro,
                                                        ))
                                                        .push(
                                                            bright_paragraph(format!(
                                                                "{} ({})",
                                                                result.metadata.title.clone(),
                                                                result
                                                                    .metadata
                                                                    .track_artist
                                                                    .clone(),
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

                    let domain_specific = {
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
                            .height(Length::Fill)
                    };
                    domain_specific
                }
                state::SearchDomainResults::Movies(results) => {
                    let title_results = match results {
                        Some(results) => {
                            let album_results = Scrollable::new(results.titles.iter().fold(
                                Column::new(),
                                |column, result| {
                                    column.push(
                                        dark_button(
                                            line_row()
                                                .spacing(5)
                                                .push(movie_image(
                                                    movie_library
                                                        .get_movie_cover(
                                                            model::MovieSize::Micro,
                                                            model::MovieTitle::from_metadata(
                                                                result,
                                                            ),
                                                        )
                                                        .unwrap_or(
                                                            app_images.get_dvd_image().clone(),
                                                        ),
                                                    model::MovieSize::Micro,
                                                ))
                                                .push(
                                                    bright_paragraph(result.title.clone())
                                                        .width(Length::Fill),
                                                ),
                                        )
                                        .on_press(
                                            user_nav_message(NavMessage::MovieView(
                                                result.clone(),
                                                model::MovieSize::SemiLarge,
                                            )),
                                        ),
                                    )
                                },
                            ));
                            album_results
                        }
                        None => Scrollable::new(Space::with_width(Length::Fill)),
                    };
                    let domain_specific = Row::new()
                        .spacing(5)
                        .push(
                            Column::new()
                                .push(h2("Movies"))
                                .push(title_results)
                                .width(Length::FillPortion(1)),
                        )
                        .width(Length::Fill)
                        .height(Length::Fill);
                    domain_specific
                }
            };

            let body = Container::new(
                Column::new()
                    .spacing(10)
                    .push(h1("Search your Library"))
                    .push(
                        Row::new()
                            .push(dark_button(h2("Music")).on_press(user_nav_message(
                                NavMessage::SearchPage(
                                    query.clone(),
                                    message::SearchDomain::Music,
                                    true,
                                ),
                            )))
                            .push(dark_button(h2("Movies")).on_press(user_nav_message(
                                NavMessage::SearchPage(
                                    query.clone(),
                                    message::SearchDomain::Movies,
                                    true,
                                ),
                            ))),
                    )
                    .push(
                        Row::new()
                            .push(
                                TextInput::new("Search...", query)
                                    .on_input(|s| Message::Action(message::Action::UpdateText(s)))
                                    .on_submit(Message::Action(message::Action::PerformSearch(
                                        query.clone(),
                                        domain,
                                    )))
                                    .id(state::TEXT_INPUT_ID.clone()),
                            )
                            .width(Length::Fill),
                    )
                    .push(domain_specific),
            );

            (breadcrumbs, body)
        }
    }
}
