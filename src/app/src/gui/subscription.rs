use iced::keyboard;
use iced::keyboard::key;
use iced::mouse;

use crate::model;
use crate::shared;

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

pub fn on_mouse_event(mouse_event: mouse::Event) -> Option<message::Message> {
    match mouse_event {
        mouse::Event::ButtonPressed(mouse::Button::Back) => Some(message::Message::HistoryNav(
            message::HistoryDirection::Backwards,
        )),
        mouse::Event::ButtonPressed(mouse::Button::Forward) => Some(message::Message::HistoryNav(
            message::HistoryDirection::Forwards,
        )),
        _ => None,
    }
}

pub fn keybinding_subscription_fn(
    key: keyboard::Key,
    modifiers: keyboard::Modifiers,
) -> Option<message::Message> {
    let mod_enum = Mods::from_iced(modifiers);
    println!("Handle Key: {:?} (with mods: {:?})", key, mod_enum);
    match (key.as_ref(), mod_enum) {
        (keyboard::Key::Character("p"), Mods::None) => Some(message::Message::Action(
            message::Action::TogglePlayQueueVisible,
        )),

        (keyboard::Key::Character("r"), Mods::Shift) => Some(
            message::ArtistNavMessage::ArtistList(
                0,
                model::ArtistSortKey::ByName,
                model::ArtistSortKey::ByName.default_order(),
            )
            .into_message(),
        ),
        (keyboard::Key::Character("a"), Mods::Shift) => Some(
            message::MusicNavMessage::AlbumList(
                0,
                model::AlbumSortKey::preferred_home(),
                model::AlbumSortKey::preferred_home().default_order(),
            )
            .into_message(),
        ),
        (keyboard::Key::Character("t"), Mods::Shift) => Some(
            message::MusicNavMessage::TrackList(
                0,
                model::TrackSortKey::ByName,
                model::TrackSortKey::ByName.default_order(),
            )
            .into_message(),
        ),
        (keyboard::Key::Character("m"), Mods::Shift) => Some(
            message::MovieNavMessage::MovieList(
                0,
                model::MovieSortKey::preferred_home(),
                model::MovieSortKey::preferred_home().default_order(),
            )
            .into_message(),
        ),
        (keyboard::Key::Character("g"), Mods::Shift) => {
            Some(message::GameNavMessage::GameHome.into_message())
        }
        (keyboard::Key::Character("p"), Mods::Shift) => {
            Some(message::PlaylistNavMessage::PlaylistList("".to_string()).into_message())
        }
        (keyboard::Key::Character("s"), Mods::Shift) => Some(
            message::NavMessage::SearchPage("".to_string(), model::SearchDomain::Music, false)
                .into_message(),
        ),

        (keyboard::Key::Character("f"), Mods::None) => {
            Some(message::Message::Action(message::Action::ToggleFullscreen))
        }

        (keyboard::Key::Character("1"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(0).into_message())
        }
        (keyboard::Key::Character("2"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(1).into_message())
        }
        (keyboard::Key::Character("3"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(2).into_message())
        }
        (keyboard::Key::Character("4"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(3).into_message())
        }
        (keyboard::Key::Character("5"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(4).into_message())
        }
        (keyboard::Key::Character("6"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(5).into_message())
        }
        (keyboard::Key::Character("7"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(6).into_message())
        }
        (keyboard::Key::Character("8"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(7).into_message())
        }
        (keyboard::Key::Character("9"), Mods::None) => {
            Some(message::NavRelMsg::BreadcrumbSelection(8).into_message())
        }

        (keyboard::Key::Named(key::Named::Space), Mods::None) => Some(
            message::Message::PlaybackRequest(shared::PlaybackRequest::PlayPauseToggle),
        ),

        (keyboard::Key::Named(key::Named::Backspace), Mods::None) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Backwards),
        ),
        (keyboard::Key::Named(key::Named::BrowserBack), Mods::None) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Backwards),
        ),
        (keyboard::Key::Named(key::Named::GoBack), Mods::None) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Backwards),
        ),
        (keyboard::Key::Named(key::Named::ArrowLeft), Mods::Alt) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Backwards),
        ),

        (keyboard::Key::Named(key::Named::BrowserForward), Mods::None) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Forwards),
        ),
        (keyboard::Key::Named(key::Named::ArrowRight), Mods::Alt) => Some(
            message::Message::HistoryNav(message::HistoryDirection::Forwards),
        ),

        (keyboard::Key::Character("["), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::SwitchSortBy(message::MoveDirectionMsg::Left),
        )),
        (keyboard::Key::Character("]"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::SwitchSortBy(message::MoveDirectionMsg::Right),
        )),

        (keyboard::Key::Character("o"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::ToggleSortOrder,
        )),

        (keyboard::Key::Character("h"), Mods::Shift) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::First),
        )),
        (keyboard::Key::Character("h"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Backwards),
        )),
        (keyboard::Key::Character("l"), Mods::None) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Forwards),
        )),
        (keyboard::Key::Character("l"), Mods::Shift) => Some(message::Message::NavRelative(
            message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Last),
        )),

        (keyboard::Key::Named(key::Named::ArrowUp), Mods::None) => {
            Some(message::Message::NavRelative(
                message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::First),
            ))
        }
        (keyboard::Key::Named(key::Named::ArrowLeft), Mods::None) => {
            Some(message::Message::NavRelative(
                message::NavRelMsg::PagifiedMovement(message::PagifiedMovementMsg::Backwards),
            ))
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

#[derive(Debug)]
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
