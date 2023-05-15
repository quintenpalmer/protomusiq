use iced::{self, Align, Column, Element, Length};

use crate::model;

use crate::gui::message::Message;

use super::style;

pub fn line_row<'a>() -> iced::Row<'a, Message> {
    iced::Row::new().align_items(Align::Center)
}

pub fn h1<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(35)
        .color(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn h2<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(25)
        .color(iced::Color::from_rgb8(0xb8, 0xb8, 0xb8))
}

pub fn h3<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(20)
        .color(iced::Color::from_rgb8(0xad, 0xad, 0xad))
}

pub fn bright_paragraph<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(16)
        .color(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn paragraph<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(16)
        .color(iced::Color::from_rgb8(0xad, 0xad, 0xad))
}

pub fn dark_paragraph<S: Into<String>>(s: S) -> iced::Text {
    iced::Text::new(s)
        .size(16)
        .color(iced::Color::from_rgb8(0x70, 0x70, 0x70))
}

pub fn bright(t: iced::Text) -> iced::Text {
    t.color(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn dark_button<'a, E>(
    state: &'a mut iced::button::State,
    content: E,
) -> iced::Button<'a, Message>
where
    E: Into<iced::Element<'a, Message>>,
{
    iced::Button::new(state, content).style(style::DarkButton {})
}

pub fn dark_text_like_button<'a, E>(
    state: &'a mut iced::button::State,
    content: E,
) -> iced::Button<'a, Message>
where
    E: Into<iced::Element<'a, Message>>,
{
    iced::Button::new(state, content).style(style::DarkTextLikeButton {})
        .padding(0)
}

pub fn bottom_label<'a, E: Into<iced::Element<'a, Message>>>(
    top: Element<'a, Message>,
    label: E,
) -> Column<'a, Message> {
    Column::new()
        .padding(8)
        .spacing(8)
        .align_items(Align::Center)
        .push(top)
        .push(label)
}

pub fn album_image(album_cover_bytes: Vec<u8>, size: model::AlbumSize) -> iced::Image {
    iced::Image::new(iced::widget::image::Handle::from_memory(album_cover_bytes))
        .width(Length::Units(size.width()))
        .height(Length::Units(size.height()))
}
