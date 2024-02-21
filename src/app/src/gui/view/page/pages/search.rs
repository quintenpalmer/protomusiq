use iced::widget::{Button, Column, Container, Row, Scrollable, Space, TextInput};
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
                state::SearchDomainResults::Music(ref _res) => model::SearchDomain::Music,
                state::SearchDomainResults::Movies(ref _res) => model::SearchDomain::Movies,
            };

            let mut breadcrumbs = vec![(
                "Search".to_string(),
                user_nav_message(NavMessage::SearchPage(
                    "".to_string(),
                    model::SearchDomain::Music,
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
                                    model::SearchDomain::Music,
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
                                    model::SearchDomain::Movies,
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
                                                            result.artist.artist_id,
                                                            result.album.album_id,
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
                                                    result.artist.artist_id,
                                                    result.album.album_id,
                                                    model::AlbumSize::Regular,
                                                    None,
                                                    None,
                                                )),
                                            ),
                                        )
                                    },
                                ))
                                .height(Length::Fill);
                                let artist_results =
                                    Scrollable::new(results.artists.iter().fold(
                                        Column::new(),
                                        |column, result| {
                                            column.push(
                                                dark_button(
                                                    line_row()
                                                        .spacing(5)
                                                        .push(album_image(
                                                            library.get_artists_first_album_cover(
                                                                model::AlbumSize::Micro,
                                                                result.artist_id,
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
                                                .on_press(user_nav_message(
                                                    NavMessage::ArtistAlbumsView(result.artist_id),
                                                )),
                                            )
                                        },
                                    ))
                                    .height(Length::Fill);
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
                                                            result.metadata.album_artist_id,
                                                            result.metadata.album_id,
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
                                ))
                                .height(Length::Fill);
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
                                                                result.metadata.album_artist_id,
                                                                result.metadata.album_id,
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
                                    ))
                                    .height(Length::Fill);
                                (
                                    album_results,
                                    artist_results,
                                    track_results,
                                    featured_artist_results,
                                )
                            }
                            None => (
                                Scrollable::new(Space::with_width(Length::Fill))
                                    .height(Length::Fill),
                                Scrollable::new(Space::with_width(Length::Fill))
                                    .height(Length::Fill),
                                Scrollable::new(Space::with_width(Length::Fill))
                                    .height(Length::Fill),
                                Scrollable::new(Space::with_width(Length::Fill))
                                    .height(Length::Fill),
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
                                                            model::MovieRelPath::from_metadata(
                                                                result,
                                                            ),
                                                        )
                                                        .unwrap_or(
                                                            app_images.get_dvd_image().clone(),
                                                        ),
                                                    model::MovieSize::Micro,
                                                    true,
                                                ))
                                                .push(
                                                    bright_paragraph(result.title.clone())
                                                        .width(Length::Fill),
                                                ),
                                        )
                                        .on_press(
                                            user_nav_message(NavMessage::MovieView(
                                                result.clone(),
                                                None,
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
                            .push(domain_button(
                                "Music",
                                query.clone(),
                                model::SearchDomain::Music,
                                domain.clone(),
                            ))
                            .push(domain_button(
                                "Movies",
                                query.clone(),
                                model::SearchDomain::Movies,
                                domain.clone(),
                            )),
                    )
                    .push(
                        Row::new()
                            .push(
                                TextInput::new("Search...", query)
                                    .on_input(|s| Message::Action(message::Action::UpdateText(s)))
                                    .on_submit(Message::Action(message::Action::PerformSearch(
                                        query.clone(),
                                        domain.clone(),
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

fn domain_button<'a>(
    display_text: &'static str,
    query: String,
    domain: model::SearchDomain,
    current_domain: model::SearchDomain,
) -> Button<'a, Message> {
    let text_element = if domain == current_domain {
        h2(display_text)
    } else {
        dark(h2(display_text))
    };
    dark_button(text_element).on_press(user_nav_message(NavMessage::SearchPage(
        query, domain, true,
    )))
}
