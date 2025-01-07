use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::Message;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn show_list<'a>(
    show_library_state: &'a musiqcore::model::shows::ShowLibraryState,
    app_images: &embedded::AppImages,
) -> Container<'a, Message> {
    let mut body_column = Column::new();

    match show_library_state.get_shows_if_exists() {
        Some(show_library) => {
            for show in show_library.get_structured_shows().get_shows().values() {
                body_column = body_column.push(dark_button(Container::new(bottom_label(
                    album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small).into(),
                    bright_paragraph(common::abr_str(show.name.clone(), consts::ICON_STR_LENGTH)),
                ))))
            }
        }
        None => (),
    }

    let body = Container::new(
        Column::new()
            .push(h1("Show List"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
