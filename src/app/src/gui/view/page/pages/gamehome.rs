use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;

use crate::gui::message;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn game_home<'a>(app_images: &embedded::AppImages) -> Container<'a, message::Message> {
    let body_column = Column::new()
        .spacing(10)
        .padding(10)
        .push(h1("Games"))
        .push(
            dark_button(Container::new(bottom_label(
                album_image(app_images.get_gba_image().clone(), model::AlbumSize::Small).into(),
                bright_paragraph(common::abr_str("GBA".to_string(), consts::ICON_STR_LENGTH)),
            )))
            .on_press(message::GameNavMessage::GBAList.into_message()),
        );

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
