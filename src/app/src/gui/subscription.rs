use iced;
use iced::keyboard;
use iced::keyboard::key;

use super::message;
use super::state;

pub fn backend_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => iced::subscription::unfold(
            "backend message callback",
            loaded.player_info.backend_callback_recv.take(),
            move |mut callback| async move {
                let msg = callback.as_mut().unwrap().recv().unwrap();
                (message::Message::BackendCallback(msg), callback)
            },
        ),
        _ => {
            println!("CALLBACK:\tbackend subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}

pub fn keybinding_subscription_fn(
    key: keyboard::Key,
    modifiers: keyboard::Modifiers,
) -> Option<message::Message> {
    match key.as_ref() {
        keyboard::Key::Character("p") => Some(message::Message::Action(
            message::Action::TogglePlayQueueVisible,
        )),
        keyboard::Key::Named(key::Named::Backspace) => Some(message::Message::HistoryNav),
        keyboard::Key::Character("h") => {
            if modifiers.shift() {
                Some(message::Message::NavRelative(
                    message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::First),
                ))
            } else {
                Some(message::Message::NavRelative(
                    message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Backwards),
                ))
            }
        }
        keyboard::Key::Character("l") => {
            if modifiers.shift() {
                Some(message::Message::NavRelative(
                    message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Last),
                ))
            } else {
                Some(message::Message::NavRelative(
                    message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Forwards),
                ))
            }
        }
        _ => None,
    }
}
