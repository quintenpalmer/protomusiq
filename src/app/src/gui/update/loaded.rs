use iced::Command;

use crate::shared;

use super::super::message::Message;
use super::super::state::AppState;

use super::action;
use super::nav;
use super::playback;
use super::spawner;

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
        Message::BackendCallback(callback) => match callback {
            shared::BackendToGUIMessage::PlayQueueState(new_play_queue) => {
                playback::handle_set_play_queue(app, new_play_queue)
            }
        },
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
        Message::ExternalSpawn(spawn_cmd) => spawner::exec_cmd(spawn_cmd),
    }
}
