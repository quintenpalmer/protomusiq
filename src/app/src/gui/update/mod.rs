mod action;
mod common;
mod external;
mod loaded;
mod loading;
mod nav;
mod navrel;
mod playback;
mod spawner;
mod volume;

use iced::Command;

use super::message::Message;
use super::state;

pub fn update(app: &mut state::App, message: Message) -> Command<Message> {
    match app {
        state::App::Loading => loading::update_from_loading_state(app, message),
        state::App::Loaded(ref mut loaded) => loaded::update_state(loaded, message),
    }
}
