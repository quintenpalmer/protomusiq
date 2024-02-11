use iced::widget::{Column, Container, Row, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{user_nav_message, ExternalSpawn, Message, NavMessage};
use crate::state;

use super::super::super::common;
use super::super::super::elements::*;

pub fn movie_page<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieViewState,
    app_images: &embedded::AppImages,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    let breadcrumbs = vec![(
        "Movies".to_string(),
        user_nav_message(NavMessage::MovieList(
            0,
            model::MovieSortKey::ByTitle,
            model::SortOrder::Regular,
        )),
    )];

    let cover_size = state.movie_size.clone();

    let movie_image_element = match movie_library.art.get_movie_cover(
        cover_size.clone(),
        model::MovieTitle::from_metadata(&state.movie),
    ) {
        Some(movie_image_bytes) => {
            let (current, toggle_to) = match cover_size {
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

            let movie_image = movie_image(movie_image_bytes, current);

            let movie_button = dark_button(movie_image).on_press(user_nav_message(
                NavMessage::MovieView(state.movie.clone(), toggle_to),
            ));

            Container::new(movie_button)
        }
        None => Container::new(movie_image(
            app_images.get_dvd_image().clone(),
            model::MovieSize::SemiLarge,
        )),
    };

    let mut movie_info = Column::new()
        .padding(10)
        .spacing(10)
        .push(h1(state.movie.title.clone()));

    let mut play_release_and_length =
        Row::new()
            .spacing(10)
            .push(
                dark_button(h2("Play")).on_press(Message::ExternalSpawn(ExternalSpawn::Mpv(
                    state.movie.path.clone().to_path_buf(),
                ))),
            );

    let mut length = Column::new();
    length = length.push(h2("Length"));
    length = length.push(h3(common::format_duration(state.movie.duration.as_secs())));
    play_release_and_length = play_release_and_length.push(length);

    match state.movie.extra {
        Some(ref extra_movie_metadata) => {
            let mut release = Column::new();
            release = release.push(h2("Release"));
            release = release.push(h3(extra_movie_metadata
                .release
                .format("%Y/%m/%d")
                .to_string()));
            play_release_and_length = play_release_and_length.push(release);
        }
        None => (),
    };

    movie_info = movie_info.push(play_release_and_length);

    match state.movie.extra {
        Some(ref extra) => {
            let mut genre_list = Row::new().spacing(10);
            for genre in extra.genres.iter() {
                genre_list = genre_list.push(h3(genre.clone()));
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
                directors = directors.push(h3(director.clone()));
            }

            let mut writers = Column::new().push(h2("Writer(s)"));
            for writer in extra.writers.iter() {
                writers = writers.push(h3(writer.clone()));
            }

            let director_writer = Row::new().spacing(10).push(directors).push(writers);
            movie_info = movie_info.push(director_writer);
        }
        None => (),
    }

    match state.movie.extra {
        Some(ref extra) => {
            let mut production_entities = Column::new().push(h2("Production"));
            for prod in extra.production.iter() {
                production_entities = production_entities.push(h3(prod.clone()));
            }
            movie_info = movie_info.push(production_entities);
        }
        None => (),
    }

    let top_header = Row::new()
        .padding(10)
        .push(movie_image_element)
        .push(movie_info);

    let mut bottom_footer = Column::new().padding(10);

    match state.movie.extra {
        Some(ref extra) => {
            bottom_footer = bottom_footer.push(h1("Cast:"));
            let mut cast = Column::new();
            for actor in extra.cast.iter() {
                cast = cast.push(h2(actor.clone()));
            }
            let cast_scrollable = Scrollable::new(cast);
            bottom_footer = bottom_footer.push(cast_scrollable);
        }
        None => (),
    }

    bottom_footer = bottom_footer
        .push(Space::new(Length::Fill, Length::Fill))
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

    let contents = Column::new()
        .spacing(10)
        .push(top_header)
        .push(bottom_footer);

    let body = Container::new(contents);

    (breadcrumbs, body)
}
