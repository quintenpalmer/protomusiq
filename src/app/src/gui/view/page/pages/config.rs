use iced::widget::Container;

use crate::gui::message::{user_nav_message, Message, NavMessage};

use super::super::super::elements::*;

pub fn config_page<'a>() -> (Vec<(String, Message)>, Container<'a, Message>) {
    (
        Vec::new(),
        Container::new(
            dark_button(bright_paragraph("Reload Library"))
                .on_press(user_nav_message(NavMessage::Config)),
        ),
    )
}
