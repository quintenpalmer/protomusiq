use iced::widget::{Column, Container, Image, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message;

use super::super::super::elements::*;

pub fn gba_list<'a>(game_library: &'a model::GameLibraryState) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("GBA Games:"));

    match game_library.get_gba_rom_paths() {
        Some(gba_rom_paths) => {
            for gba_rom_path in gba_rom_paths.iter() {
                body_column = body_column.push({
                    let mut row = line_row();
                    row = row.push(
                        dark_button(h2(">")).on_press(message::Message::ExternalSpawn(
                            message::ExternalSpawn::MGBA(gba_rom_path.path.clone()),
                        )),
                    );

                    match gba_rom_path.image {
                        Some(ref game_image_bytes) => {
                            row = row.push(
                                Image::new(iced::widget::image::Handle::from_memory(
                                    game_image_bytes.clone(),
                                ))
                                .width(Length::Fixed(500.0))
                                .height(Length::Fixed(500.0)),
                            );
                        }
                        None => (),
                    };

                    row = row.push(h2(gba_rom_path.name.clone()));
                    row
                });
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
