use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::consts;

use super::super::super::common;
use super::super::super::elements::*;

pub fn movie_list<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieListState,
    play_queue_info: &state::PlayQueueInfo,
    grid_info: &model::GridInfo,
    app_images: &embedded::AppImages,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::MovieListState {
            page,
            sort_key,
            sort_order,
        } => {
            let breadcrumbs = vec![(
                "Movies".to_string(),
                user_nav_message(NavMessage::MovieList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )];

            let page: usize = page.clone();

            let indices = common::get_page(
                movie_library
                    .movie_sorts
                    .from_sort_key(&sort_key, &sort_order),
                page,
                grid_info.get_page_size_usize(),
            );

            let mut buttons: Vec<Button<Message>> = Vec::new();

            for movie in indices.iter() {
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

                let movie_image_element =
                    album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small);

                buttons.push(
                    dark_button(bottom_label(movie_image_element.into(), movie_info))
                        .on_press(user_nav_message(NavMessage::MovieView(movie.clone()))),
                );
            }

            let mut columns: Column<Message> = Column::new();
            if play_queue_info.play_queue_visible {
                for _i in 0..(grid_info.get_layout_height() * 2) {
                    let mut rows = Row::new();
                    for _j in 0..(grid_info.get_layout_width() / 2) {
                        if buttons.len() > 0 {
                            let button = buttons.remove(0);
                            rows = rows.push(button);
                        }
                    }
                    columns = columns.push(rows);
                }
            } else {
                for _i in 0..grid_info.get_layout_height() {
                    let mut rows = Row::new();
                    for _j in 0..grid_info.get_layout_width() {
                        if buttons.len() > 0 {
                            let button = buttons.remove(0);
                            rows = rows.push(button);
                        }
                    }
                    columns = columns.push(rows);
                }
            }

            let first_page = 0;
            let back_page = {
                if page == 0 {
                    0
                } else {
                    page - 1
                }
            };
            let forward_page = {
                if ((page + 1) * grid_info.get_page_size_usize())
                    >= movie_library.movies.movies.len()
                {
                    page
                } else {
                    page + 1
                }
            };
            let last_page = {
                let maybe_last_page =
                    movie_library.movies.movies.len() / grid_info.get_page_size_usize();
                if maybe_last_page * grid_info.get_page_size_usize()
                    >= movie_library.movies.movies.len()
                {
                    maybe_last_page - 1
                } else {
                    maybe_last_page
                }
            };

            let page_buttons = line_row().push(
                line_row()
                    .push(paragraph("Sort By: "))
                    .push(
                        dark_button(bright_paragraph("Title")).on_press(user_nav_message(
                            NavMessage::MovieList(
                                0,
                                model::MovieSortKey::ByTitle,
                                model::SortOrder::Regular,
                            ),
                        )),
                    )
                    .push(
                        dark_button(bright_paragraph("Random")).on_press(user_nav_message(
                            NavMessage::MovieList(
                                0,
                                model::MovieSortKey::Random,
                                model::SortOrder::Regular,
                            ),
                        )),
                    ),
            );

            let extra_page_buttons = line_row()
                .push(
                    line_row()
                        .push(
                            dark_button(bright_paragraph("<<")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    first_page,
                                    sort_key.clone(),
                                    sort_order.clone(),
                                ),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph("<")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    back_page,
                                    sort_key.clone(),
                                    sort_order.clone(),
                                ),
                            )),
                        )
                        .push(bright_paragraph(page.to_string()))
                        .push(
                            dark_button(bright_paragraph(">")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    forward_page,
                                    sort_key.clone(),
                                    sort_order.clone(),
                                ),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph(">>")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    last_page,
                                    sort_key.clone(),
                                    sort_order.clone(),
                                ),
                            )),
                        ),
                )
                .push(Space::with_width(Length::Fill))
                .push(
                    line_row()
                        .push(paragraph("Order: "))
                        .push(
                            dark_button(bright_paragraph("^")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Reversed,
                                ),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph("v")).on_press(user_nav_message(
                                NavMessage::MovieList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Regular,
                                ),
                            )),
                        ),
                );

            let body = Container::new(
                Column::new()
                    .spacing(10)
                    .push(h1("Movies"))
                    .push(page_buttons)
                    .push(extra_page_buttons)
                    .push(Scrollable::new(columns)),
            );

            (breadcrumbs, body)
        }
    }
}
