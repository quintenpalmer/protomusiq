use iced::widget::{Column, Container, Image, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message;

use super::super::super::elements::*;

pub fn n64_list<'a>(game_library: &'a model::GameLibraryState) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("N64 Games:"));

    match game_library.get_n64_rom_paths() {
        Some(n64_rom_paths) => {
            for n64_rom_path in n64_rom_paths.iter() {
                body_column = body_column.push({
                    let mut ret_row = line_row();
                    ret_row = ret_row.push(dark_button(h2(">")).on_press(
                        message::Message::ExternalSpawn(message::ExternalSpawn::Mupen64(
                            n64_rom_path.path.clone(),
                        )),
                    ));

                    match n64_rom_path.image {
                        Some(ref game_image_bytes) => {
                            ret_row = ret_row.push(
                                Image::new(iced::widget::image::Handle::from_memory(
                                    game_image_bytes.clone(),
                                ))
                                .width(Length::Fixed(500.0))
                                .height(Length::Fixed(500.0)),
                            );
                        }
                        None => (),
                    };

                    ret_row = ret_row.push(h2(n64_rom_path.name.clone()));
                    ret_row
                });
            }
        }
        None => (),
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
