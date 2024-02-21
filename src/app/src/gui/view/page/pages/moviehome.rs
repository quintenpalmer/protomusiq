use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};

use super::super::super::elements::*;

pub fn movie_home<'a>() -> (Vec<(String, Message)>, Container<'a, Message>) {
    let breadcrumbs = vec![("Movie".to_string(), user_nav_message(NavMessage::MovieHome))];

    let body_column = Column::new()
        .spacing(10)
        .padding(10)
        .push(
            dark_button(h1("Movie List")).on_press(user_nav_message(NavMessage::MovieList(
                0,
                model::MovieSortKey::ByTitle,
                model::MovieSortKey::ByTitle.default_order(),
            ))),
        )
        .push(
            dark_button(h1("Movie Query")).on_press(user_nav_message(NavMessage::MovieQuery(None))),
        )
        .push(
            dark_button(h1("Movie Attributes"))
                .on_press(user_nav_message(NavMessage::MovieAttributes(None))),
        );

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    (breadcrumbs, body)
}
