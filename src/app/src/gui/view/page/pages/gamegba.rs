use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message;

use super::super::super::elements::*;

pub fn gba_list<'a>(game_library: &'a model::GameLibraryState) -> Container<'a, message::Message> {
    let mut body_column = Column::new().spacing(10).padding(10).push(h1("GBA Games:"));

    for gba_rom_path in game_library.get_gba_rom_paths() {
        body_column = body_column.push(h2(gba_rom_path
            .clone()
            .into_os_string()
            .to_string_lossy()
            .to_string()));
    }

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
