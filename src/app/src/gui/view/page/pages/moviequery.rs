use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, Message};
use crate::state;

use super::super::super::elements::*;

pub fn movie_query<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieQueryState,
    app_images: &embedded::AppImages,
) -> Container<'a, Message> {
    let input_query_element = match state.query {
        Some(ref query) => match query {
            model::MovieQueryParams::Genre(ref genre) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Genre")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Genres,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(genre.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::Genre(genre.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
            model::MovieQueryParams::Production(ref prod) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Production Company")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Production,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(prod.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::Production(prod.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
            model::MovieQueryParams::Producers(ref prod) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Producers")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Producers,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(prod.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::Producers(prod.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
            model::MovieQueryParams::Director(ref director) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Director")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Directors,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(director.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::Director(director.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
            model::MovieQueryParams::Screenplay(ref writer) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Screenplay")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Screenplay,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(writer.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::Screenplay(writer.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
            model::MovieQueryParams::CastMember(ref actor) => {
                Column::new().spacing(10).push(h2("Params:")).push(
                    line_row()
                        .spacing(5)
                        .push(
                            dark_button(h3("Cast Member")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::CastMembers,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(h3(":"))
                        .push(
                            dark_button(h3(actor.clone())).on_press(
                                message::MovieNavMessage::MovieQuery(Some(
                                    model::MovieQueryParams::CastMember(actor.clone()),
                                ))
                                .into_message(),
                            ),
                        ),
                )
            }
        },
        None => Column::new().push(h2("Query Builder coming soon...")),
    };

    let movies = match state.matched_keys {
        Some(ref matched_keys) => {
            let mut movie_list = Column::new().spacing(10);
            for movie_key in matched_keys.iter() {
                let movie = movie_library.get_movie(movie_key);

                let movie_info = h1(movie.title.clone());

                let movie_link =
                    message::MovieNavMessage::MovieView(movie.clone(), None).into_message();

                let movie_image_element = match movie_library.art.get_movie_cover(
                    model::MovieSize::Small,
                    model::MovieRelPath::from_metadata(&movie),
                ) {
                    Some(movie_image_bytes) => {
                        movie_image(movie_image_bytes, model::MovieSize::Small, true)
                    }
                    None => movie_image(
                        app_images.get_dvd_image().clone(),
                        model::MovieSize::Small,
                        true,
                    ),
                };

                let movie_button = Row::new()
                    .push(dark_button(movie_image_element).on_press(movie_link.clone()))
                    .push(
                        Container::new(dark_button(movie_info).on_press(movie_link))
                            .width(Length::Fill),
                    );

                movie_list = movie_list.push(movie_button);
            }

            let movies = Column::new()
                .spacing(10)
                .push(h1("Found Movies:"))
                .push(Scrollable::new(movie_list).height(Length::Fill));

            movies
        }
        None => Column::new().push(h3("Here is where you will be able to build the query")),
    };

    let body = Container::new(
        Column::new()
            .spacing(10)
            .push(h1("Movie Query"))
            .push(input_query_element)
            .push(movies),
    );

    body
}
