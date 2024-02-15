use iced;

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

pub fn keybinding_subscription_fn(_app: &state::App) -> iced::Subscription<message::Message> {
    iced::subscription::events_with(|event, status| {
        match status {
            iced::event::Status::Captured => return None,
            iced::event::Status::Ignored => (),
        };

        match event {
            iced::event::Event::Keyboard(keypress_event) => match keypress_event {
                iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::P,
                    ..
                } => Some(message::Message::Action(
                    message::Action::TogglePlayQueueVisible,
                )),
                iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Backspace,
                    ..
                } => Some(message::Message::HistoryNav),
                iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::H,
                    modifiers,
                } => {
                    if modifiers.shift() {
                        Some(message::Message::NavRelative(
                            message::NavRelMsg::PagifiedMovement(
                                message::PagifiedMovementMsg::First,
                            ),
                        ))
                    } else {
                        Some(message::Message::NavRelative(
                            message::NavRelMsg::PagifiedMovement(
                                message::PagifiedMovementMsg::Backwards,
                            ),
                        ))
                    }
                }
                iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::L,
                    modifiers,
                } => {
                    if modifiers.shift() {
                        Some(message::Message::NavRelative(
                            message::NavRelMsg::PagifiedMovement(
                                message::PagifiedMovementMsg::Last,
                            ),
                        ))
                    } else {
                        Some(message::Message::NavRelative(
                            message::NavRelMsg::PagifiedMovement(
                                message::PagifiedMovementMsg::Forwards,
                            ),
                        ))
                    }
                }
                _ => None,
            },
            _ => None,
        }
    })
}
