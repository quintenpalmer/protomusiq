use iced::widget::{Column, Container, Row};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{user_nav_message, ExternalSpawn, Message, NavMessage};
use crate::state;

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

    let movie_image_element = match movie_library.art.get_movie_cover(
        model::MovieSize::Large,
        model::MovieTitle::from_metadata(&state.movie),
    ) {
        Some(movie_image_bytes) => movie_image(movie_image_bytes, model::MovieSize::Large),
        None => album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Large),
    };

    let contents = Column::new()
        .spacing(10)
        .push(
            Row::new().push(movie_image_element).push(
                Column::new()
                    .padding(10)
                    .spacing(10)
                    .push(h2(state.movie.title.clone()))
                    .push(dark_button(h3("Play")).on_press(Message::ExternalSpawn(
                        ExternalSpawn::Mpv(state.movie.path.clone().to_path_buf()),
                    ))),
            ),
        )
        .push(bright_paragraph(
            state
                .movie
                .path
                .clone()
                .into_os_string()
                .to_string_lossy()
                .to_string(),
        ));

    let body = Container::new(Column::new().spacing(10).push(h1("Movies")).push(contents));

    (breadcrumbs, body)
}
