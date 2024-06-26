use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, ExternalSpawn, Message};
use crate::state;

use super::super::super::common;
use super::super::super::elements::*;

pub fn movie_page<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieViewState,
    grid_info: &model::GridInfo,
    app_images: &embedded::AppImages,
) -> Container<'a, Message> {
    let maybe_cover_size = state.movie_size.clone();

    let cover_size = match maybe_cover_size {
        Some(c) => c,
        None => model::MovieSize::SemiLarge,
    };

    let maybe_current_sort_order = state.maybe_current_sort_order.clone();

    let title_element = h1(state.movie.title.clone());

    let movie_image_element = match movie_library.art.get_movie_cover(
        cover_size.clone(),
        model::MovieRelPath::from_metadata(&state.movie),
    ) {
        Some(movie_image_bytes) => {
            let (current, toggle_to) = match cover_size.clone() {
                model::MovieSize::Micro => (model::MovieSize::Micro, model::MovieSize::Small),
                model::MovieSize::Small => (model::MovieSize::Small, model::MovieSize::Regular),
                model::MovieSize::Regular => {
                    (model::MovieSize::Regular, model::MovieSize::SemiLarge)
                }
                model::MovieSize::SemiLarge => {
                    (model::MovieSize::SemiLarge, model::MovieSize::Large)
                }
                model::MovieSize::Large => (model::MovieSize::Large, model::MovieSize::Micro),
            };

            let movie_image = movie_image(movie_image_bytes, current, false);

            let movie_button = dark_button(movie_image).on_press(
                message::MovieNavMessage::MovieView(
                    state.movie.clone(),
                    Some(toggle_to),
                    maybe_current_sort_order,
                )
                .into_message(),
            );

            Container::new(movie_button)
        }
        None => Container::new(movie_image(
            app_images.get_dvd_image().clone(),
            model::MovieSize::SemiLarge,
            false,
        )),
    };

    let mut movie_info = Column::new().padding(10).spacing(10);

    let play_button = Container::new(dark_button(h2("Play")).on_press(Message::ExternalSpawn(
        ExternalSpawn::Mpv(state.movie.path.clone().to_path_buf()),
    )));

    let mut length = Column::new();
    length = length.push(h2("Length"));
    length = length.push(h3(common::format_duration(state.movie.duration.as_secs())));

    let play_and_length = Row::new()
        .spacing(10)
        .push(play_button.width(Length::FillPortion(1)))
        .push(length.width(Length::FillPortion(1)));

    let mut play_release_and_length = Row::new()
        .spacing(10)
        .push(play_and_length.width(Length::FillPortion(1)));

    match state.movie.extra {
        Some(ref extra_movie_metadata) => {
            let mut release = Column::new();
            release = release.push(h2("Release"));
            release = release.push(h3(extra_movie_metadata
                .release
                .format("%Y/%m/%d")
                .to_string()));
            play_release_and_length =
                play_release_and_length.push(release.width(Length::FillPortion(1)));
        }
        None => (),
    };

    movie_info = movie_info.push(play_release_and_length);

    match state.movie.extra {
        Some(ref extra) => {
            let mut genre_list = Column::new().spacing(10);
            let mut genre_row = Row::new().spacing(10);
            let mut genre_row_count = 0;
            for (index, genre) in extra.genres.iter().enumerate() {
                genre_row = genre_row.push(
                    dark_button(h3(genre.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(model::MovieQueryParams::Genre(
                            genre.clone(),
                        )))
                        .into_message(),
                    ),
                );
                genre_row_count += 1;

                if index % 3 == 2 {
                    genre_list = genre_list.push(genre_row);
                    genre_row = Row::new().spacing(10);
                    genre_row_count = 0;
                }
            }

            if genre_row_count != 0 {
                genre_list = genre_list.push(genre_row);
            }

            let genres = Column::new()
                .padding([10, 0])
                .push(h2("Genres"))
                .push(genre_list);

            movie_info = movie_info.push(genres);
        }
        None => (),
    }

    match state.movie.extra {
        Some(ref extra) => {
            let mut directors = Column::new().push(h2("Director(s)"));
            for director in extra.directors.iter() {
                directors = directors.push(
                    dark_button(h3(director.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(
                            model::MovieQueryParams::Director(director.clone()),
                        ))
                        .into_message(),
                    ),
                );
            }

            let mut writers = Column::new().push(h2("Screenplay Writer(s)"));
            for writer in extra.writers.iter() {
                writers = writers.push(
                    dark_button(h3(writer.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(
                            model::MovieQueryParams::Screenplay(writer.clone()),
                        ))
                        .into_message(),
                    ),
                );
            }

            let director_writer = Row::new()
                .spacing(10)
                .push(directors.width(Length::FillPortion(1)))
                .push(writers.width(Length::FillPortion(1)));
            movie_info = movie_info.push(director_writer);
        }
        None => (),
    }

    match state.movie.extra {
        Some(ref extra) => {
            let mut producers = Column::new().push(h2("Producers"));
            for prod in extra.producers.iter() {
                producers = producers.push(
                    dark_button(h3(prod.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(
                            model::MovieQueryParams::Producers(prod.clone()),
                        ))
                        .into_message(),
                    ),
                );
            }

            let mut production_entities = Column::new().push(h2("Production"));
            for prod in extra.production.iter() {
                production_entities = production_entities.push(
                    dark_button(h3(prod.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(
                            model::MovieQueryParams::Production(prod.clone()),
                        ))
                        .into_message(),
                    ),
                );
            }

            let production_and_producer = Row::new()
                .spacing(10)
                .push(producers.width(Length::FillPortion(1)))
                .push(production_entities.width(Length::FillPortion(1)));
            movie_info = movie_info.push(production_and_producer);
        }
        None => (),
    }

    let top_header = Row::new()
        .padding(10)
        .push(movie_image_element)
        .push(Scrollable::new(movie_info).height(Length::Fixed(cover_size.height() as f32)));

    let mut cast_main_container = Row::new().spacing(10).padding([0, 30]);

    match state.movie.extra {
        Some(ref extra) => {
            cast_main_container = cast_main_container.push(h1("Cast:"));
            let mut cast = Column::new();
            for actor in extra.cast.iter() {
                cast = cast.push(
                    dark_button(h2(actor.clone())).on_press(
                        message::MovieNavMessage::MovieQuery(Some(
                            model::MovieQueryParams::CastMember(actor.clone()),
                        ))
                        .into_message(),
                    ),
                );
            }
            let cast_scrollable = Scrollable::new(cast.width(Length::Fill)).height(Length::Fill);
            cast_main_container = cast_main_container.push(cast_scrollable);
        }
        None => (),
    }

    let bottom_footer = Column::new()
        .padding(10)
        .push(dark_paragraph("Filepath:"))
        .push(bright_paragraph(
            state
                .movie
                .path
                .clone()
                .into_os_string()
                .to_string_lossy()
                .to_string(),
        ));

    let mut contents = Column::new()
        .spacing(10)
        .push(title_element)
        .push(top_header)
        .push(cast_main_container.height(Length::Fill));

    match state.maybe_current_sort_order {
        Some(ref current_sort_order) => {
            let sort_nav_row = line_row()
                .spacing(8)
                .push(
                    dark_button(bright_paragraph("<")).on_press(Message::NavRelative(
                        message::NavRelMsg::PagifiedMovement(
                            message::PagifiedMovementMsg::Backwards,
                        ),
                    )),
                )
                .push(h3(format!("{}", current_sort_order.index)))
                .push(
                    dark_button(bright_paragraph(">")).on_press(Message::NavRelative(
                        message::NavRelMsg::PagifiedMovement(
                            message::PagifiedMovementMsg::Forwards,
                        ),
                    )),
                )
                .push(
                    dark_button(h3(format!(
                        "{} ({})",
                        current_sort_order.sort_key.display_text(),
                        current_sort_order.sort_order.display_text(),
                    )))
                    .on_press(
                        message::MovieNavMessage::MovieList(
                            current_sort_order.index / grid_info.get_page_size_usize(),
                            current_sort_order.sort_key.clone(),
                            current_sort_order.sort_order.clone(),
                        )
                        .into_message(),
                    ),
                );
            contents = contents.push(sort_nav_row);
        }
        None => (),
    }

    contents = contents.push(bottom_footer);

    let body = Container::new(contents);

    body
}
