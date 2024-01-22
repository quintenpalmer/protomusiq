use iced::widget::{Button, Column, Container, Scrollable};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{user_nav_message, ExternalSpawn, Message, NavMessage};
use crate::state;

use super::super::consts;

use super::super::super::common;
use super::super::super::elements::*;

pub fn movie_list<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieListState,
    app_images: &embedded::AppImages,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::MovieListState {
            page: _page,
            sort_key,
            sort_order,
        } => {
            let breadcrumbs = vec![(
                "Tracks".to_string(),
                user_nav_message(NavMessage::MovieList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )];

            let mut buttons: Vec<Button<Message>> = Vec::new();

            for movie in movie_library.movies.iter() {
                let mut movie_info = Column::new();
                movie_info = movie_info.push(bright_paragraph(common::abr_str(
                    movie
                        .path
                        .clone()
                        .into_os_string()
                        .to_string_lossy()
                        .to_string(),
                    consts::ICON_STR_LENGTH,
                )));
                movie_info = movie_info.push(bright_paragraph(common::abr_str(
                    movie.title.clone(),
                    consts::ICON_STR_LENGTH,
                )));

                buttons.push(
                    dark_button(bottom_label(
                        album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small)
                            .into(),
                        movie_info,
                    ))
                    .on_press(Message::ExternalSpawn(ExternalSpawn::Mpv(
                        movie.path.clone().to_path_buf(),
                    ))),
                );
            }

            let mut button_column = Column::new();
            for button in buttons.into_iter() {
                button_column = button_column.push(button);
            }

            let body = Container::new(
                Column::new()
                    .push(h1("Movies"))
                    .push(Scrollable::new(button_column)),
            );

            (breadcrumbs, body)
        }
    }
}
