use iced::widget::{Column, Container, Image, Scrollable};
use iced::Length;

use crate::gui::message;

use super::super::super::elements::*;

pub fn gbc_list<'a>(
    game_library: &'a musiqcore::model::gl::GameLibraryState,
) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("GBC Games:"));

    match game_library.get_gbc_rom_paths() {
        Some(gbc_rom_paths) => {
            for gbc_rom_path in gbc_rom_paths.iter() {
                body_column = body_column.push({
                    let mut row = line_row();
                    row = row.push(
                        dark_button(h2(">")).on_press(message::Message::ExternalSpawn(
                            message::ExternalSpawn::MGBA(gbc_rom_path.path.clone()),
                        )),
                    );

                    row = row.push(
                        Image::new(iced::widget::image::Handle::from_memory(
                            gbc_rom_path.image.clone(),
                        ))
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(500.0)),
                    );

                    row = row.push(h2(gbc_rom_path.name.clone()));
                    row
                });
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
