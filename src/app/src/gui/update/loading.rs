use iced::Command;

use super::init;
use super::message::{self, Message};
use super::state::App;

pub fn update_from_loading_state(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::Action(message::Action::LoadEverything) => {
            *app = init::initialize_everything();
            Command::none()
        }
        _ => {
            println!("cannot process {:?} before we are loaded", message);
            Command::none()
        }
    }
}
