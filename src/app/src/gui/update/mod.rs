use iced::widget::text_input;
use iced::Command;

use crate::model;
use crate::shared;

use super::init;
use super::message::{self, Message, NavMessage};
use super::state::{self, App, AppState, Page};

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
        Message::Action(action) => match action {
            message::Action::LoadEverything => {
                println!("Everything is already loaded");
                Command::none()
            }
            message::Action::SetVolume(volume_request) => {
                handle_volume_request(app, volume_request)
            }
            message::Action::CreateNewPlaylist(playlist_name) => {
                app.library.user_playlists.add_playlist(playlist_name);
                message::message_command(message::user_nav_message(
                    message::NavMessage::PlaylistList("".to_string()),
                ))
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
                state::Page::Search(ref _search_state) => message::message_command(
                    message::user_nav_message(message::NavMessage::SearchPage(query, true)),
                ),
                _ => Command::none(),
            },
            message::Action::Close => Command::batch(vec![
                Command::perform(
                    mpris_sender(
                        app.player_info.mpris_message_sender.clone(),
                        shared::MprisMessage::Close,
                    )
                    .send_message(),
                    Message::ErrorResponse,
                ),
                Command::perform(
                    sink_sender(
                        app.player_info.sink_message_sender.clone(),
                        shared::SinkMessage::Close,
                    )
                    .send_message(),
                    Message::ErrorResponse,
                ),
                iced::window::close(),
            ]),
        },
        Message::Nav(nav_message) => {
            app.page_back_history.push(app.page_current_history.clone());
            app.page_current_history = nav_message.clone();
            handle_nav(app, nav_message)
        }
        Message::HistoryNav => match app.page_back_history.pop() {
            Some(history_message) => {
                let old_current = app.page_current_history.clone();
                app.page_current_history = history_message.clone();
                app.page_forward_history.insert(0, old_current);
                handle_nav(app, history_message)
            }
            None => Command::none(),
        },
        Message::PlaybackRequest(internal) => {
            println!("GUI:\thandling internal: {:?}", internal);
            match internal {
                message::PlaybackRequest::LoadCurrentSong => {
                    match app.player_info.current_playback {
                        Some(ref outer_current_playback) => match outer_current_playback {
                            state::CurrentPlayback::Track(ref current_playback) => {
                                app.player_info.playing = true;
                                Command::batch(vec![
                                    Command::perform(
                                        mpris_sender(
                                            app.player_info.mpris_message_sender.clone(),
                                            shared::MprisMessage::SetMetadata(
                                                current_playback
                                                    .track
                                                    .metadata
                                                    .album_artist
                                                    .clone(),
                                                current_playback.track.metadata.title.clone(),
                                            ),
                                        )
                                        .send_message(),
                                        Message::ErrorResponse,
                                    ),
                                    Command::perform(
                                        {
                                            sink_sender(
                                                app.player_info.sink_message_sender.clone(),
                                                shared::SinkMessage::LoadSong(
                                                    current_playback.track.metadata.path.clone(),
                                                    app.player_info.current_volume,
                                                ),
                                            )
                                            .send_message()
                                        },
                                        Message::ErrorResponse,
                                    ),
                                    Command::perform(
                                        tracker_sender(
                                            app.player_info.tracker_message_sender.clone(),
                                            shared::TrackerMessage::SongStarted(
                                                current_playback.track.clone(),
                                            ),
                                        )
                                        .send_message(),
                                        Message::ErrorResponse,
                                    ),
                                ])
                            }
                            state::CurrentPlayback::PauseBreak => {
                                //app.player_info.rest.playing = false;
                                message::message_command(Message::PlaybackRequest(
                                    message::PlaybackRequest::Pause,
                                ))
                            }
                        },
                        None => Command::none(),
                    }
                }
                message::PlaybackRequest::PlaySongs(tracks) => message::message_command(
                    Message::PlaybackRequest(message::PlaybackRequest::InsertSongs(tracks, true)),
                ),
                message::PlaybackRequest::AppendSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(
                            state::PlayQueueTrack { track: iter_track },
                        ));
                    }
                    app.play_queue_info
                        .play_queue
                        .append(&mut new_songs_to_queue);
                    if load_next {
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::Next,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::InsertSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(
                            state::PlayQueueTrack { track: iter_track },
                        ));
                    }
                    new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
                    app.play_queue_info.play_queue = new_songs_to_queue;

                    if load_next {
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::Next,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Prev => {
                    if app.play_queue_info.play_history.len() > 0 {
                        match app.player_info.current_playback {
                            Some(ref current_playback) => {
                                let mut new_play_queue =
                                    vec![state::PlayQueueEntry::from_playback(current_playback)];
                                new_play_queue.append(&mut app.play_queue_info.play_queue);
                                app.play_queue_info.play_queue = new_play_queue;
                            }
                            None => (),
                        };
                        let track = app.play_queue_info.play_history.pop().unwrap();
                        app.player_info.current_playback = Some(match track {
                            state::PlayQueueEntry::Track(ref t) => {
                                state::CurrentPlayback::Track(state::CurrentTrackPlayback {
                                    track: t.track.clone(),
                                    current_second: 0,
                                })
                            }
                            state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => {
                                state::CurrentPlayback::PauseBreak
                            }
                        });
                        app.play_queue_info.current_playback = Some(track.clone());
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::LoadCurrentSong,
                        ))
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Next => {
                    if app.play_queue_info.play_queue.len() > 0 {
                        match app.player_info.current_playback {
                            Some(ref current_playback) => app
                                .play_queue_info
                                .play_history
                                .push(state::PlayQueueEntry::from_playback(current_playback)),
                            None => (),
                        };

                        let track = app.play_queue_info.play_queue.remove(0);
                        app.player_info.current_playback =
                            Some(state::CurrentPlayback::from_entry_zeroed(&track));
                        app.play_queue_info.current_playback = Some(track.clone());
                        message::message_command(Message::PlaybackRequest(
                            message::PlaybackRequest::LoadCurrentSong,
                        ))
                    } else {
                        match app.player_info.current_playback {
                            Some(ref current_playback) => app
                                .play_queue_info
                                .play_history
                                .push(state::PlayQueueEntry::from_playback(current_playback)),
                            None => (),
                        };
                        app.player_info.current_playback = None;
                        app.play_queue_info.current_playback = None;
                        Command::perform(
                            mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetStopped,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        )
                    }
                }
                message::PlaybackRequest::Play => {
                    app.player_info.playing = true;
                    Command::batch(vec![
                        Command::perform(
                            mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPlaying,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            sink_sender(
                                app.player_info.sink_message_sender.clone(),
                                shared::SinkMessage::PlayButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::Pause => {
                    app.player_info.playing = false;
                    Command::batch(vec![
                        Command::perform(
                            mpris_sender(
                                app.player_info.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPaused,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            sink_sender(
                                app.player_info.sink_message_sender.clone(),
                                shared::SinkMessage::PauseButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::InsertPause => {
                    let mut new_songs_to_queue =
                        vec![state::PlayQueueEntry::Action(state::PlayQueueAction::Pause)];
                    new_songs_to_queue.append(&mut app.play_queue_info.play_queue);
                    app.play_queue_info.play_queue = new_songs_to_queue;

                    Command::none()
                }
            }
        }
        Message::SinkCallback(callback) => match callback {
            shared::SinkCallbackMessage::SongEnded => {
                message::message_command(Message::PlaybackRequest(message::PlaybackRequest::Next))
            }
            shared::SinkCallbackMessage::SecondElapsed => {
                match app.player_info.current_playback {
                    Some(ref mut outer_current_playback) => match outer_current_playback {
                        state::CurrentPlayback::Track(ref mut current_playback) => {
                            current_playback.current_second += 1
                        }
                        _ => println!("Hmmm, songs are playing back while on a pause break?"),
                    },
                    None => (),
                };
                Command::none()
            }
            shared::SinkCallbackMessage::Paused => Command::none(),
            shared::SinkCallbackMessage::Playing => Command::none(),
        },
        Message::ErrorResponse(resp) => {
            match resp {
                Ok(()) => (),
                Err(e) => println!("error from calling out to subservice: {:?}", e),
            };
            Command::none()
        }
        Message::MprisCallback(callback) => message::message_command(match callback {
            shared::MprisCallbackMessage::PlayPause => {
                if app.player_info.playing {
                    Message::PlaybackRequest(message::PlaybackRequest::Pause)
                } else {
                    Message::PlaybackRequest(message::PlaybackRequest::Play)
                }
            }
            shared::MprisCallbackMessage::Play => {
                Message::PlaybackRequest(message::PlaybackRequest::Play)
            }
            shared::MprisCallbackMessage::Pause => {
                Message::PlaybackRequest(message::PlaybackRequest::Pause)
            }
            shared::MprisCallbackMessage::Prev => {
                Message::PlaybackRequest(message::PlaybackRequest::Prev)
            }
            shared::MprisCallbackMessage::Next => {
                Message::PlaybackRequest(message::PlaybackRequest::Next)
            }
        }),
    }
}

fn handle_volume_request(
    app: &mut AppState,
    volume_request: message::VolumeRequest,
) -> Command<Message> {
    match volume_request {
        message::VolumeRequest::Up(delta) => app.player_info.current_volume += delta,
        message::VolumeRequest::Down(delta) => app.player_info.current_volume -= delta,
        message::VolumeRequest::Set(new_volume) => app.player_info.current_volume = new_volume,
    };
    Command::perform(
        {
            sink_sender(
                app.player_info.sink_message_sender.clone(),
                shared::SinkMessage::SetVolume(app.player_info.current_volume),
            )
            .send_message()
        },
        Message::ErrorResponse,
    )
}

fn handle_nav(app: &mut AppState, nav_message: message::NavMessage) -> Command<message::Message> {
    match nav_message {
        NavMessage::Home => {
            app.current_page = Page::Home(state::HomeState {});
            Command::none()
        }
        NavMessage::Config => {
            app.current_page = Page::Config(state::ConfigState {});
            Command::none()
        }
        NavMessage::PlayQueueFocus => {
            app.current_page = Page::PlayQueue(state::PlayQueueState {});
            Command::none()
        }
        NavMessage::PlaylistView(playlist_id) => {
            app.current_page = Page::PlaylistView(state::PlaylistViewState {
                playlist_id: playlist_id,
            });
            Command::none()
        }
        NavMessage::PlaylistList(new_playlist_name) => {
            app.current_page = Page::PlaylistList(state::PlaylistListState {
                new_playlist_name: new_playlist_name,
            });
            Command::none()
        }
        NavMessage::SearchPage(query, perform_search) => {
            let computed_results = match perform_search {
                true => {
                    let search_results = app.library.search(query.clone());
                    let mapped_search_results = model::SearchResults {
                        artists: search_results
                            .artists
                            .into_iter()
                            .map(|artist| model::Pair {
                                first: artist.first,
                                second: (),
                            })
                            .collect(),
                        albums: search_results
                            .albums
                            .into_iter()
                            .map(|artist_album| model::Pair {
                                first: artist_album.first,
                                second: (),
                            })
                            .collect(),
                        tracks: search_results
                            .tracks
                            .into_iter()
                            .map(|track| model::Pair {
                                first: track.first,
                                second: (),
                            })
                            .collect(),
                        track_artists: search_results
                            .track_artists
                            .into_iter()
                            .map(|track_artist| model::Pair {
                                first: track_artist.first,
                                second: (),
                            })
                            .collect(),
                    };
                    Some(mapped_search_results)
                }
                false => None,
            };

            app.current_page = Page::Search(state::SearchPageState {
                query: query,
                results: computed_results,
            });
            text_input::focus(state::TEXT_INPUT_ID.clone())
        }
        NavMessage::TrackList(page, sort, sort_order) => {
            app.current_page = Page::TrackList(state::TrackListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::AlbumList(page, sort, sort_order) => {
            app.current_page = Page::AlbumList(state::AlbumListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistList(page, sort, sort_order) => {
            app.current_page = Page::ArtistList(state::ArtistListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistView(artist_id) => {
            app.current_page = Page::ArtistView(state::ArtistViewState {
                artist_id: artist_id.clone(),
                albums: app
                    .library
                    .get_artist_map()
                    .get(&artist_id)
                    .unwrap()
                    .albums
                    .keys()
                    .map(|k| k.clone())
                    .collect(),
            });
            Command::none()
        }
        NavMessage::ArtistTrackView(artist_id, sort_key, sort_order) => {
            app.current_page = Page::ArtistTrackView(state::ArtistTrackViewState {
                artist_id: artist_id.clone(),

                sort_key: sort_key,
                sort_order: sort_order,
            });
            Command::none()
        }
        NavMessage::ArtistAlbumView(artist_id, album_id, album_size, maybe_selected_track) => {
            app.current_page = Page::ArtistAlbumView(state::ArtistAlbumViewState {
                artist_id: artist_id.clone(),
                album_id: album_id.clone(),
                album_size: album_size,
                maybe_selected_track: maybe_selected_track,
            });
            Command::none()
        }
    }
}

fn tracker_sender<T: std::fmt::Debug>(
    tx: shared::Client<T>,
    message: T,
) -> MessageCommandSender<T> {
    MessageCommandSender::new("Tracker".to_string(), tx, message)
}

fn mpris_sender<T: std::fmt::Debug>(tx: shared::Client<T>, message: T) -> MessageCommandSender<T> {
    MessageCommandSender::new("Mpris".to_string(), tx, message)
}

fn sink_sender<T: std::fmt::Debug>(tx: shared::Client<T>, message: T) -> MessageCommandSender<T> {
    MessageCommandSender::new("Sink".to_string(), tx, message)
}

struct MessageCommandSender<T> {
    name: String,
    tx: shared::Client<T>,
    message: T,
}

impl<T: std::fmt::Debug> MessageCommandSender<T> {
    fn new(name: String, tx: shared::Client<T>, message: T) -> Self {
        MessageCommandSender {
            name: name,
            tx: tx,
            message: message,
        }
    }

    async fn send_message(self) -> Result<(), String> {
        println!("GUI:\t{}: payload is {:?}", self.name, self.message);
        match self.tx.send(self.message) {
            Ok(a) => {
                println!("GUI:\t{}: resp was {:?}", self.name, a);
                Ok(())
            }
            Err(e) => {
                println!("GUI:\t{}: err resp was {:?}", self.name, e);
                Err(format!("{:?}", e))
            }
        }
    }
}
