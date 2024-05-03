use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message;

use super::super::super::elements::*;

pub fn wii_list<'a>(game_library: &'a model::GameLibraryState) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("Wii Games:"));

    match game_library.get_wii_rom_paths() {
        Some(wii_rom_paths) => {
            for wii_rom_path in wii_rom_paths.iter() {
                body_column = body_column.push(
                    line_row()
                        .push(
                            dark_button(h2(">")).on_press(message::Message::ExternalSpawn(
                                message::ExternalSpawn::DolphinWii(wii_rom_path.path.clone()),
                            )),
                        )
                        .push(h2(wii_rom_path.name.clone())),
                );
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
