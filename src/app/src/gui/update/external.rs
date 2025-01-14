use iced::Command;

use crate::gui::message;

use super::super::state::AppState;

use super::loaded;

pub fn handle_external_request(
    app: &mut AppState,
    external_request: message::ExternalRequest,
) -> Command<message::Message> {
    match external_request {
        message::ExternalRequest::PlayShow(show_episode) => {
            match app.show_library.get_shows_if_exists_mut() {
                Some(show_library) => {
                    let tracker = show_library.get_tracker_mut();
                    tracker.mark_episode_viewed_now(show_episode.get_key());
                }
                None => println!("we can't actually store the recording of this show"),
            }
            let _follow_up_empty_action = loaded::update_state(
                app,
                message::ExternalSpawn::Mpv(show_episode.full_path).into_message(),
            );
        }
    }
    Command::none()
}
