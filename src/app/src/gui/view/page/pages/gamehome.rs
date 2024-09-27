use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;

use crate::gui::message;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn game_home<'a>(app_images: &embedded::AppImages) -> Container<'a, message::Message> {
    let gb_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_gba_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("GB".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::GBList.into_message());

    let gbc_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_gba_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("GBC".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::GBCList.into_message());

    let gba_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_gba_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("GBA".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::GBAList.into_message());

    let nds_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_nds_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str(
            "Nintendo DS".to_string(),
            consts::ICON_STR_LENGTH,
        )),
    )))
    .on_press(message::GameNavMessage::NDSList.into_message());

    let snes_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_snes_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("SNES".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::SNESList.into_message());

    let n64_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_n64_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("N64".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::N64List.into_message());

    let ngc_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_ngc_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str(
            "GameCube".to_string(),
            consts::ICON_STR_LENGTH,
        )),
    )))
    .on_press(message::GameNavMessage::GameCubeList.into_message());

    let wii_link = dark_button(Container::new(bottom_label(
        album_image(app_images.get_wii_image().clone(), model::AlbumSize::Small).into(),
        bright_paragraph(common::abr_str("Wii".to_string(), consts::ICON_STR_LENGTH)),
    )))
    .on_press(message::GameNavMessage::WiiList.into_message());

    let body_column = Column::new()
        .push(h1("Games"))
        .push(line_row().push(gb_link).push(gbc_link).push(gba_link))
        .push(line_row().push(snes_link).push(nds_link))
        .push(line_row().push(n64_link).push(ngc_link).push(wii_link));

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
