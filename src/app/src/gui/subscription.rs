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
    let mod_enum = Mods::from_iced(modifiers);
    match (key.as_ref(), mod_enum) {
        (keyboard::Key::Character("p"), Mods::None) => Some(message::Message::Action(
            message::Action::TogglePlayQueueVisible,
        )),
        (keyboard::Key::Character("h"), Mods::Shift) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::First),
        )),
        (keyboard::Key::Character("h"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Backwards),
        )),
        (keyboard::Key::Character("l"), Mods::Shift) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Last),
        )),
        (keyboard::Key::Character("l"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Forwards),
        )),
        (keyboard::Key::Named(key::Named::Backspace), Mods::None) => {
            Some(message::Message::HistoryNav)
        }
        (keyboard::Key::Named(key::Named::BrowserBack), Mods::None) => {
            Some(message::Message::HistoryNav)
        }
        (keyboard::Key::Named(key::Named::ArrowLeft), Mods::Alt) => {
            Some(message::Message::HistoryNav)
        }

        (keyboard::Key::Named(key::Named::ArrowUp), Mods::None) => {
            Some(message::Message::NavRelative(
                message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::First),
            ))
        }
        (keyboard::Key::Named(key::Named::ArrowLeft), Mods::None) => {
            Some(message::Message::HistoryNav)
        }
        (keyboard::Key::Named(key::Named::ArrowRight), Mods::None) => {
            Some(message::Message::NavRelative(
                message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Forwards),
            ))
        }
        (keyboard::Key::Named(key::Named::ArrowDown), Mods::None) => {
            Some(message::Message::NavRelative(
                message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Last),
            ))
        }
        _ => None,
    }
}

enum Mods {
    None,
    Shift,
    Control,
    Alt,
    Logo,
    ShiftControl,
    ShiftAlt,
    ShiftLogo,
    ControlAlt,
    ControlLogo,
    AltLogo,
    ShiftControlAlt,
    ShiftControlLogo,
    ShiftAltLogo,
    ControlAltLogo,
    ShiftControlAltLogo,
}

impl Mods {
    pub fn from_iced(mods: keyboard::Modifiers) -> Self {
        match (mods.shift(), mods.control(), mods.alt(), mods.logo()) {
            (false, false, false, false) => Mods::None,
            (true, false, false, false) => Mods::Shift,
            (false, true, false, false) => Mods::Control,
            (false, false, true, false) => Mods::Alt,
            (false, false, false, true) => Mods::Logo,
            (true, true, false, false) => Mods::ShiftControl,
            (true, false, true, false) => Mods::ShiftAlt,
            (true, false, false, true) => Mods::ShiftLogo,
            (false, true, true, false) => Mods::ControlAlt,
            (false, true, false, true) => Mods::ControlLogo,
            (false, false, true, true) => Mods::AltLogo,
            (true, true, true, false) => Mods::ShiftControlAlt,
            (true, true, false, true) => Mods::ShiftControlLogo,
            (true, false, true, true) => Mods::ShiftAltLogo,
            (false, true, true, true) => Mods::ControlAltLogo,
            (true, true, true, true) => Mods::ShiftControlAltLogo,
        }
    }
}
