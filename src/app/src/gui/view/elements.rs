use iced::widget::image;
use iced::widget::{Button, Column, Container, Image, Row, Text};
use iced::{Alignment, Element, Length};

use crate::model;

use crate::gui::message::Message;

use super::style;

pub fn line_row<'a>() -> Row<'a, Message> {
    Row::new().align_items(Alignment::Center)
}

pub fn dark(t: Text) -> Text {
    t.style(iced::Color::from_rgb8(0x70, 0x70, 0x70))
}

pub fn h1<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(29)
        .style(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn h2<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(20)
        .style(iced::Color::from_rgb8(0xb8, 0xb8, 0xb8))
}

pub fn h3<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(16)
        .style(iced::Color::from_rgb8(0xad, 0xad, 0xad))
}

pub fn bright_paragraph<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(13)
        .style(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn paragraph<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(13)
        .style(iced::Color::from_rgb8(0xad, 0xad, 0xad))
}

pub fn dark_paragraph<S: Into<String>>(s: S) -> Text<'static> {
    Text::new(s.into())
        .size(13)
        .style(iced::Color::from_rgb8(0x70, 0x70, 0x70))
}

pub fn green_notification<S: Into<String>>(s: S) -> Container<'static, Message> {
    Container::new(Text::new(s.into()).size(13))
        .style(iced::theme::Container::Custom(Box::new(
            style::NotificationGreenBackground {},
        )))
        .padding([2, 4])
}

pub fn grey_notification<S: Into<String>>(s: S) -> Container<'static, Message> {
    Container::new(Text::new(s.into()).size(13))
        .style(iced::theme::Container::Custom(Box::new(
            style::NotificationGreyBackground {},
        )))
        .padding([2, 4])
}

pub fn bright(t: Text) -> Text {
    t.style(iced::Color::from_rgb8(0xd8, 0xd8, 0xd8))
}

pub fn dark_button<'a, E>(content: E) -> Button<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    Button::new(content).style(iced::theme::Button::Custom(Box::new(style::DarkButton {})))
}

pub fn dark_text_like_button<'a, E>(content: E) -> Button<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    Button::new(content)
        .style(iced::theme::Button::Custom(Box::new(
            style::DarkTextLikeButton {},
        )))
        .padding(0)
}

pub fn bottom_label<'a, E: Into<Element<'a, Message>>>(
    top: Element<'a, Message>,
    label: E,
) -> Column<'a, Message> {
    Column::new()
        .padding(2)
        .spacing(2)
        .align_items(Alignment::Center)
        .push(top)
        .push(label)
}

pub fn album_image(album_cover_bytes: Vec<u8>, size: model::AlbumSize) -> Image<image::Handle> {
    Image::new(iced::widget::image::Handle::from_memory(album_cover_bytes))
        .width(Length::Fixed(size.width() as f32))
        .height(Length::Fixed(size.height() as f32))
}

pub fn movie_image(
    movie_cover_bytes: Vec<u8>,
    size: model::MovieSize,
    fixed_width: bool,
) -> Image<image::Handle> {
    Image::new(iced::widget::image::Handle::from_memory(movie_cover_bytes))
        .width(if fixed_width {
            Length::Fixed(size.height() as f32 * 2.4 / 3.0)
        } else {
            Length::Shrink
        })
        .height(Length::Fixed(size.height() as f32))
}
