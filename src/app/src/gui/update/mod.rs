mod action;
mod common;
mod loaded;
mod loading;
mod mpris;
mod nav;
mod playback;
mod sink;
mod volume;

pub use loaded::update_state;
pub use loading::update_from_loading_state;

use iced::Command;

use super::message::Message;
use super::state;

pub fn update(app: &mut state::App, message: Message) -> Command<Message> {
    match app {
        state::App::Loading => loading::update_from_loading_state(app, message),
        state::App::Loaded(ref mut loaded) => loaded::update_state(loaded, message),
    }
}
