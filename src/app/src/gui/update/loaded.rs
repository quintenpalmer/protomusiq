use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::AppState;

use super::action;
use super::nav;
use super::navrel;
use super::playback;
use super::spawner;

pub fn update_state(app: &mut AppState, message: Message) -> Command<Message> {
    println!(
        "GUI:\tupdating with {:?} ({})",
        message,
        app.page_state
            .page_back_history
            .iter()
            .fold("".to_string(), |total, current| {
                format!("{:?}, {}", current, total)
            })
    );

    match message {
        Message::Action(action) => action::handle_action(app, action),
        Message::PlaybackRequest(internal) => playback::handle_playback_request(app, internal),
        Message::BackendCallback(callback) => match callback {
            shared::BackendToGUIMessage::PlayQueueState(new_play_queue) => {
                playback::handle_set_play_queue(app, new_play_queue)
            }
        },
        Message::Nav(nav_message) => {
            app.page_state
                .page_back_history
                .push(app.page_state.page_current_history.clone());
            app.page_state.page_current_history = nav_message.clone();
            app.page_state.page_forward_history = Vec::new();
            nav::handle_nav(app, nav_message)
        }
        Message::NavRelative(nav_message) => navrel::handle_nav_relative(app, nav_message),
        Message::HistoryNav(direction) => match direction {
            message::HistoryDirection::Backwards => match app.page_state.page_back_history.pop() {
                Some(history_message) => {
                    let old_current = app.page_state.page_current_history.clone();
                    app.page_state.page_current_history = history_message.clone();
                    app.page_state.page_forward_history.insert(0, old_current);
                    nav::handle_nav(app, history_message)
                }
                None => Command::none(),
            },
            message::HistoryDirection::Forwards => {
                if app.page_state.page_forward_history.is_empty() {
                    Command::none()
                } else {
                    let history_message = app.page_state.page_forward_history.remove(0);
                    let old_current = app.page_state.page_current_history.clone();
                    app.page_state.page_current_history = history_message.clone();
                    app.page_state.page_back_history.push(old_current);
                    nav::handle_nav(app, history_message)
                }
            }
        },
        Message::ErrorResponse(resp) => {
            match resp {
                Ok(()) => (),
                Err(e) => println!("error from calling out to subservice: {:?}", e),
            };
            Command::none()
        }
        Message::ExternalSpawn(spawn_cmd) => spawner::exec_cmd(&app.game_library, spawn_cmd),
    }
}
