use iced::widget::Button;

use crate::model;

use crate::gui::message;

use super::super::elements::*;

pub fn sort_button<'a, A: Eq, F: Fn(A, model::SortOrder) -> message::Message>(
    display_text: String,
    sort_key: A,
    order: model::SortOrder,
    current_sort: &'a A,
    f: F,
) -> Button<'a, message::Message> {
    let text_element = if &sort_key == current_sort {
        bright_paragraph(display_text)
    } else {
        dark_paragraph(display_text)
    };
    dark_button(text_element).on_press(f(sort_key, order))
}
