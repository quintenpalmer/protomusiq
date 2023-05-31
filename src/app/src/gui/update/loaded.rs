use iced::Command;

use super::super::message::Message;
use super::super::state::AppState;

use super::action;
use super::mpris;
use super::nav;
use super::playback;
use super::setup;
use super::sink;

pub fn update_state(app: &mut AppState, message: Message) -> Command<Message> {
    println!(
        "GUI:\tupdating with {:?} ({})",
        message,
        app.page_back_history
            .iter()
            .fold("".to_string(), |total, current| {
                format!("{:?}, {}", current, total)
            })
    );

    match message {
        Message::Action(action) => action::handle_action(app, action),
        Message::PlaybackRequest(internal) => playback::handle_playback_request(app, internal),
        Message::Setup(setup) => setup::handle_setup(app, setup),
        Message::SinkCallback(callback) => sink::handle_sink_callback(app, callback),
        Message::MprisCallback(callback) => mpris::handle_mpris_callback(app, callback),
        Message::Nav(nav_message) => {
            app.page_back_history.push(app.page_current_history.clone());
            app.page_current_history = nav_message.clone();
            nav::handle_nav(app, nav_message)
        }
        Message::HistoryNav => match app.page_back_history.pop() {
            Some(history_message) => {
                let old_current = app.page_current_history.clone();
                app.page_current_history = history_message.clone();
                app.page_forward_history.insert(0, old_current);
                nav::handle_nav(app, history_message)
            }
            None => Command::none(),
        },
        Message::ErrorResponse(resp) => {
            match resp {
                Ok(()) => (),
                Err(e) => println!("error from calling out to subservice: {:?}", e),
            };
            Command::none()
        }
    }
}
