use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, Message};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn show_home<'a>(app_images: &embedded::AppImages) -> Container<'a, Message> {
    let show_list_link = dark_button(Container::new(bottom_label(
        album_image(
            app_images.get_tracks_image().clone(),
            model::AlbumSize::Small,
        )
        .into(),
        bright_paragraph(common::abr_str(
            "All Show Series".to_string(),
            consts::ICON_STR_LENGTH,
        )),
    )))
    .on_press(message::ShowNavMessage::ShowList.into_message());

    let body_column = Column::new().push(line_row().push(show_list_link));

    let body = Container::new(
        Column::new()
            .push(h1("Shows"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
