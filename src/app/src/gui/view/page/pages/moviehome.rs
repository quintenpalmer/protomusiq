use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, Message};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn movie_home<'a>(app_images: &embedded::AppImages) -> Container<'a, Message> {
    let body_column = Column::new()
        .push(
            Row::new()
                .push(
                    dark_button(Container::new(bottom_label(
                        album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small)
                            .into(),
                        bright_paragraph(common::abr_str(
                            "Movie List".to_string(),
                            consts::ICON_STR_LENGTH,
                        )),
                    )))
                    .on_press(
                        message::MovieNavMessage::MovieList(
                            0,
                            model::MovieSortKey::preferred_home(),
                            model::MovieSortKey::preferred_home().default_order(),
                        )
                        .into_message(),
                    ),
                )
                .push(
                    dark_button(Container::new(bottom_label(
                        album_image(
                            app_images.get_search_image().clone(),
                            model::AlbumSize::Small,
                        )
                        .into(),
                        bright_paragraph(common::abr_str(
                            "Movie Query".to_string(),
                            consts::ICON_STR_LENGTH,
                        )),
                    )))
                    .on_press(message::MovieNavMessage::MovieQuery(None).into_message()),
                )
                .push(
                    dark_button(Container::new(bottom_label(
                        album_image(app_images.get_tag_image().clone(), model::AlbumSize::Small)
                            .into(),
                        bright_paragraph(common::abr_str(
                            "Movie Attrs".to_string(),
                            consts::ICON_STR_LENGTH,
                        )),
                    )))
                    .on_press(message::MovieNavMessage::MovieAttributes(None).into_message()),
                ),
        )
        .push(
            Row::new().push(
                dark_button(Container::new(bottom_label(
                    album_image(app_images.get_tag_image().clone(), model::AlbumSize::Small).into(),
                    bright_paragraph(common::abr_str(
                        "Movie Series".to_string(),
                        consts::ICON_STR_LENGTH,
                    )),
                )))
                .on_press(message::MovieNavMessage::SeriesList.into_message()),
            ),
        );

    let body = Container::new(
        Column::new()
            .push(h1("Movies"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
