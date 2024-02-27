use iced::widget::{Button, Column, Container, Row, Scrollable, Space};
use iced::{Alignment, Length};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, Message, NavRelMsg, PagifiedMovementMsg};
use crate::state;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn movie_list<'a>(
    movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieListState,
    play_queue_visible: bool,
    grid_info: &model::GridInfo,
    app_images: &embedded::AppImages,
) -> Container<'a, Message> {
    match state {
        state::MovieListState {
            page,
            sort_key,
            sort_order,
        } => {
            let page: usize = *page;

            let indices = common::get_page(
                movie_library
                    .movie_sorts
                    .from_sort_key(sort_key, sort_order),
                page,
                grid_info.get_page_size_usize(),
            );

            let mut buttons: Vec<Button<Message>> = Vec::new();

            for movie in indices.iter() {
                let mut movie_info = Column::new();
                movie_info = movie_info.push(bright_paragraph(common::abr_str(
                    movie.title.clone(),
                    consts::MOVIE_SUB_STR_LENGTH,
                )));

                let movie_image_element = match movie_library.art.get_movie_cover(
                    model::MovieSize::Small,
                    model::MovieRelPath::from_metadata(movie),
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

                buttons.push(
                    dark_button(bottom_label(movie_image_element.into(), movie_info)).on_press(
                        message::MovieNavMessage::MovieView(movie.clone(), None).into_message(),
                    ),
                );
            }

            let mut columns: Column<Message> = Column::new();
            if play_queue_visible {
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

            let page_buttons = line_row().push(
                line_row()
                    .push(paragraph("Sort By: "))
                    .push(sort_button(
                        "Title",
                        model::MovieSortKey::ByTitle,
                        model::MovieSortKey::ByTitle.default_order(),
                        sort_key,
                    ))
                    .push(sort_button(
                        "Added",
                        model::MovieSortKey::LastModified,
                        model::MovieSortKey::LastModified.default_order(),
                        sort_key,
                    ))
                    .push(sort_button(
                        "Length",
                        model::MovieSortKey::ByDuration,
                        model::MovieSortKey::ByDuration.default_order(),
                        sort_key,
                    ))
                    .push(sort_button(
                        "Release",
                        model::MovieSortKey::ByRelease,
                        model::MovieSortKey::ByRelease.default_order(),
                        sort_key,
                    ))
                    .push(sort_button(
                        "Random",
                        model::MovieSortKey::Random,
                        model::MovieSortKey::Random.default_order(),
                        sort_key,
                    )),
            );

            let extra_page_buttons = line_row()
                .push(
                    line_row()
                        .push(
                            dark_button(bright_paragraph("<<")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::First),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph("<")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Backwards),
                            )),
                        )
                        .push(bright_paragraph(page.to_string()))
                        .push(
                            dark_button(bright_paragraph(">")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Forwards),
                            )),
                        )
                        .push(
                            dark_button(bright_paragraph(">>")).on_press(Message::NavRelative(
                                NavRelMsg::PagifiedMovement(PagifiedMovementMsg::Last),
                            )),
                        ),
                )
                .push(Space::with_width(Length::Fill))
                .push(
                    line_row()
                        .push(paragraph("Order: "))
                        .push(
                            dark_button({
                                if sort_order == &model::SortOrder::Reversed {
                                    bright_paragraph("^")
                                } else {
                                    dark_paragraph("^")
                                }
                            })
                            .on_press(
                                message::MovieNavMessage::MovieList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Reversed,
                                )
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button({
                                if sort_order == &model::SortOrder::Regular {
                                    bright_paragraph("v")
                                } else {
                                    dark_paragraph("v")
                                }
                            })
                            .on_press(
                                message::MovieNavMessage::MovieList(
                                    0,
                                    sort_key.clone(),
                                    model::SortOrder::Regular,
                                )
                                .into_message(),
                            ),
                        ),
                );

            let body = Container::new(
                Column::new()
                    .spacing(10)
                    .push(
                        Column::new()
                            .spacing(10)
                            .push(
                                Row::new()
                                    .align_items(Alignment::End)
                                    .push(h1("Movies"))
                                    .push(Space::with_width(Length::Fixed(20.0)))
                                    .push(extra_page_buttons),
                            )
                            .push(page_buttons),
                    )
                    .push(Scrollable::new(columns.width(Length::Fill)).height(Length::Fill)),
            );

            body
        }
    }
}

fn sort_button<'a>(
    display_text: &'static str,
    sort_key: model::MovieSortKey,
    order: model::SortOrder,
    current_sort: &'a model::MovieSortKey,
) -> Button<'a, Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element)
        .on_press(message::MovieNavMessage::MovieList(0, sort_key, order).into_message())
}
