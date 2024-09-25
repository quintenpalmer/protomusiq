use iced::widget::{Column, Container, Image, Scrollable};
use iced::Length;

use crate::gui::message;

use super::super::super::elements::*;

pub fn nds_list<'a>(
    game_library: &'a musiqcore::model::gl::GameLibraryState,
) -> Container<'a, message::Message> {
    let mut body_column = Column::new()
        .spacing(10)
        .padding(10)
        .push(h1("Nintendo DS Games:"));

    match game_library.get_nds_rom_paths() {
        Some(nds_rom_paths) => {
            for nds_rom_path in nds_rom_paths.iter() {
                body_column = body_column.push({
                    let mut ret_row = line_row();
                    ret_row = ret_row.push(dark_button(h2(">")).on_press(
                        message::Message::ExternalSpawn(message::ExternalSpawn::LaunchEmulator(
                            musiqcore::model::gl::consoles::GameConsole::NintendoDS,
                            nds_rom_path.path.clone(),
                        )),
                    ));

                    ret_row = ret_row.push(
                        Image::new(iced::widget::image::Handle::from_memory(
                            nds_rom_path.image.clone(),
                        ))
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(500.0)),
                    );

                    ret_row = ret_row.push(h2(nds_rom_path.name.clone()));
                    ret_row
                });
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
