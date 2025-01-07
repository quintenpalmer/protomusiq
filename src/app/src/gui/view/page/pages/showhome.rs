use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::gui::message::Message;

use super::super::super::elements::*;

pub fn show_home<'a>() -> Container<'a, Message> {
    let body_column = Column::new();

    let body = Container::new(
        Column::new()
            .push(h1("Shows"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
