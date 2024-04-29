use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::datastore::staticassets::embedded;

use crate::gui::message;

use super::super::super::elements::*;

pub fn game_home<'a>(_app_images: &embedded::AppImages) -> Container<'a, message::Message> {
    let body_column = Column::new().spacing(10).padding(10).push(h1("Games"));

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
