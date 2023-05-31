use iced::Command;

use super::super::message;
use super::super::state::AppState;

pub fn handle_setup(
    _app: &mut AppState,
    _setup: message::SubscriptionSetup,
) -> Command<message::Message> {
    Command::none()
}
