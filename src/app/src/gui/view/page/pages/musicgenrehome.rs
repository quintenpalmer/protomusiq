use iced::widget::Container;

use crate::gui::message::Message;

use super::super::super::elements::*;

pub fn genre_home<'a>() -> Container<'a, Message> {
    let page = h1("Genres");

    Container::new(page)
}
