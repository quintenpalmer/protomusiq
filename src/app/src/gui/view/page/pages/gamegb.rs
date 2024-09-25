use iced::widget::{Column, Container, Image, Scrollable};
use iced::Length;

use crate::gui::message;

use super::super::super::elements::*;

pub fn gb_list<'a>(
    game_library: &'a musiqcore::model::gl::GameLibraryState,
) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("GB Games:"));

    match game_library.get_gb_rom_paths() {
        Some(gb_rom_paths) => {
            for gb_rom_path in gb_rom_paths.iter() {
                body_column = body_column.push({
                    let mut row = line_row();
                    row = row.push(
                        dark_button(h2(">")).on_press(message::Message::ExternalSpawn(
                            message::ExternalSpawn::LaunchEmulator(
                                musiqcore::model::gl::consoles::GameConsole::GameBoy,
                                gb_rom_path.path.clone(),
                            ),
                        )),
                    );

                    row = row.push(
                        Image::new(iced::widget::image::Handle::from_memory(
                            gb_rom_path.image.clone(),
                        ))
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(500.0)),
                    );

                    row = row.push(h2(gb_rom_path.name.clone()));
                    row
                });
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
