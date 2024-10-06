use iced::widget::Button;

use crate::model;

use crate::gui::message;

use super::super::elements::*;

pub fn sort_button<'a>(
    display_text: String,
    sort_key: model::ArtistSortKey,
    order: model::SortOrder,
    current_sort: &'a model::ArtistSortKey,
) -> Button<'a, message::Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element)
        .on_press(message::ArtistNavMessage::ArtistList(0, sort_key, order).into_message())
}
