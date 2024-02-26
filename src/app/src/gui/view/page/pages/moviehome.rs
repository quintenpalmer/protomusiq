use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};

use super::super::super::elements::*;

pub fn movie_home<'a>() -> (Vec<(String, Message)>, Container<'a, Message>) {
    let breadcrumbs = vec![(
        "Movie".to_string(),
        message::MovieNavMessage::MovieHome.into_message(),
    )];

    let body_column = Column::new()
        .spacing(10)
        .padding(10)
        .push(
            dark_button(h1("Movie List")).on_press(
                message::MovieNavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::MovieSortKey::ByTitle.default_order(),
                )
                .into_message(),
            ),
        )
        .push(
            dark_button(h1("Movie Query"))
                .on_press(message::MovieNavMessage::MovieQuery(None).into_message()),
        )
        .push(
            dark_button(h1("Movie Attributes"))
                .on_press(message::MovieNavMessage::MovieAttributes(None).into_message()),
        );

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    (breadcrumbs, body)
}
