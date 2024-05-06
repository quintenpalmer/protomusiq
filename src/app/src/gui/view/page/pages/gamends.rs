use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message;

use super::super::super::elements::*;

pub fn nds_list<'a>(game_library: &'a model::GameLibraryState) -> Container<'a, message::Message> {
    let mut body_column = Column::new()
        .spacing(10)
        .padding(10)
        .push(h1("Nintendo DS Games:"));

    match game_library.get_nds_rom_paths() {
        Some(nds_rom_paths) => {
            for nds_rom_path in nds_rom_paths.iter() {
                body_column = body_column.push(
                    line_row()
                        .push(
                            dark_button(h2(">")).on_press(message::Message::ExternalSpawn(
                                message::ExternalSpawn::Desmume(nds_rom_path.path.clone()),
                            )),
                        )
                        .push(h2(nds_rom_path.name.clone())),
                );
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
