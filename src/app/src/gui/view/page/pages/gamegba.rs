use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::gui::message;

use super::super::super::elements::*;

pub fn gba_list<'a>() -> Container<'a, message::Message> {
    let body_column = Column::new().spacing(10).padding(10).push(h1("GBA Games:"));

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
