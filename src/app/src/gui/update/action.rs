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
        message::Action::SetVolume(volume_request) => {
            volume::handle_volume_request(app, volume_request)
        }
        message::Action::CreateNewPlaylist(playlist_name) => {
            app.library.user_playlists.add_playlist(playlist_name);
            loaded::update_state(
                app,
                message::user_nav_message(message::NavMessage::PlaylistList("".to_string())),
            )
        }
        message::Action::MakePlaylistDefault(playlist_id) => {
            app.library
                .user_playlists
                .make_playlist_default(playlist_id);
            Command::none()
        }
        message::Action::AddTracksToPlaylist(playlist_id, track_ids) => {
            for track_id in track_ids.into_iter() {
                match app
                    .library
                    .user_playlists
                    .add_track_to_playlist(playlist_id, track_id)
                {
                    Ok(_) => (),
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
        message::Action::RemoveTrackFromPlayQueue(history_or_queue, index) => {
            match history_or_queue {
                message::HistoryOrQueue::History => {
                    app.play_queue_info.play_history.remove(index);
                }
                message::HistoryOrQueue::Queue => {
                    app.play_queue_info.play_queue.remove(index);
                }
            };
            Command::none()
        }
        message::Action::ToggleShuffleOnAdd => {
            app.action_state.group_buttons_shuffle = !app.action_state.group_buttons_shuffle;
            Command::none()
        }
        message::Action::TogglePlayQueueVisible => {
            app.play_queue_info.play_queue_visible = !app.play_queue_info.play_queue_visible;
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
        message::Action::PerformSearch(query) => match app.current_page {
            state::Page::Search(ref _search_state) => loaded::update_state(
                app,
                message::user_nav_message(message::NavMessage::SearchPage(query, true)),
            ),
            _ => Command::none(),
        },
        message::Action::Close => Command::batch(vec![
            Command::perform(
                common::mpris_sender(
                    app.player_info.mpris_message_sender.clone(),
                    shared::MprisMessage::Close,
                )
                .send_message(),
                Message::ErrorResponse,
            ),
            Command::perform(
                common::sink_sender(
                    app.player_info.sink_message_sender.clone(),
                    shared::SinkMessage::Close,
                )
                .send_message(),
                Message::ErrorResponse,
            ),
            iced::window::close(),
        ]),
    }
}
