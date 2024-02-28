use iced::Command;

use crate::shared;

use super::super::message::{self, Message};
use super::super::state::{self, AppState};

use super::common;
use super::loaded;
use super::volume;

pub fn handle_action(app: &mut AppState, action: message::Action) -> Command<message::Message> {
    match action {
        message::Action::LoadEverything => {
            println!("Everything is already loaded");
            Command::none()
        }
        message::Action::ToggleFullscreen => {
            app.cross_page_display_info.fullscreen_display =
                !app.cross_page_display_info.fullscreen_display;
            Command::none()
        }
        message::Action::SetVolume(volume_request) => {
            volume::handle_volume_request(app, volume_request)
        }
        message::Action::CreateNewPlaylist(playlist_name) => {
            app.library.user_playlists.add_playlist(playlist_name);
            loaded::update_state(
                app,
                message::PlaylistNavMessage::PlaylistList("".to_string()).into_message(),
            )
        }
        message::Action::MakePlaylistDefault(playlist_id) => {
            app.library
                .user_playlists
                .make_playlist_default(playlist_id);
            Command::none()
        }
        message::Action::AddTracksToPlaylist(playlist_id, track_ids) => {
            let playlist_name = app
                .library
                .user_playlists
                .get_playlist(playlist_id)
                .unwrap()
                .name
                .clone();
            for track_id in track_ids.into_iter() {
                match app
                    .library
                    .user_playlists
                    .add_track_to_playlist(playlist_id, track_id.clone())
                {
                    Ok(_) => {
                        let track = app.library.get_track(&track_id);
                        let _follow_up_empty_action = handle_action(
                            app,
                            message::Action::Notify(message::NotificationMessage::OnScreen(
                                message::NotificationAction::AddedToPlaylist(
                                    track.metadata.title.clone(),
                                    playlist_name.clone(),
                                ),
                            )),
                        );
                    }
                    Err(err_string) => {
                        println!("error adding track to playlist: {}", err_string)
                    }
                };
            }
            Command::none()
        }
        message::Action::DeletePlaylist(playlist_id) => {
            match app.library.user_playlists.delete_playlist(playlist_id) {
                Ok(_) => (),
                Err(err_string) => println!("error deleting playlist: {}", err_string),
            };
            Command::none()
        }
        message::Action::RemoveTrackFromPlaylist(playlist_id, track_id) => {
            match app
                .library
                .user_playlists
                .remove_track_from_playlist(playlist_id, track_id)
            {
                Ok(_) => (),
                Err(err_string) => {
                    println!("error removing track from playlist: {}", err_string)
                }
            };
            Command::none()
        }
        message::Action::MoveTrackInPlaylist(playlist_id, direction, track_id) => {
            match app.library.user_playlists.move_track_in_playlist(
                playlist_id,
                direction,
                track_id,
            ) {
                Ok(_) => (),
                Err(err_string) => {
                    println!("error removing track from playlist: {}", err_string)
                }
            };
            Command::none()
        }
        message::Action::ToggleShuffleOnAdd => {
            app.action_state.group_buttons_shuffle = !app.action_state.group_buttons_shuffle;
            Command::none()
        }
        message::Action::TogglePlayQueueVisible => {
            app.player_info.play_queue_info.play_queue_visible =
                !app.player_info.play_queue_info.play_queue_visible;
            Command::none()
        }
        message::Action::UpdateText(new_text) => {
            match &mut app.current_page {
                state::Page::Search(search_page_state) => search_page_state.query = new_text,
                state::Page::PlaylistList(playlist_page_state) => {
                    playlist_page_state.new_playlist_name = new_text
                }
                no_text_input_page => {
                    println!("no text to update for page: {:?}", no_text_input_page)
                }
            };
            Command::none()
        }
        message::Action::PerformSearch(query, domain) => match app.current_page {
            state::Page::Search(ref _search_state) => loaded::update_state(
                app,
                message::user_nav_message(message::NavMessage::SearchPage(query, domain, true)),
            ),
            _ => Command::none(),
        },
        message::Action::Notify(notification_message) => match notification_message {
            message::NotificationMessage::OnScreen(notification_type) => {
                println!("Got this message: {:?}", notification_type);
                app.messages.push(state::MessageInfo { notification_type });
                Command::none()
            }
            message::NotificationMessage::PopOnScreen => {
                let _popped = app.messages.pop();
                Command::none()
            }
            message::NotificationMessage::ClearOnScreen => {
                app.messages = Vec::new();
                Command::none()
            }
        },
        message::Action::Close => Command::batch(vec![
            Command::perform(
                common::backend_sender(
                    app.player_info.backend_message_sender.clone(),
                    shared::GUIToBackendMessage::BackendPlayback(shared::PlaybackRequest::Close),
                )
                .send_message(),
                Message::ErrorResponse,
            ),
            iced::window::close(iced::window::Id::MAIN),
        ]),
    }
}
