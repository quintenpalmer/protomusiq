use iced::{button, scrollable, text_input, Command};

use crate::model;
use crate::shared;

use super::init;
use super::message::{self, Message, NavMessage};
use super::state::{self, App, Loaded, Page};

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

pub fn update_state(app: &mut Loaded, message: Message) -> Command<Message> {
    println!(
        "GUI:\tupdating with {:?} ({})",
        message,
        app.rest
            .page_back_history
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
                app.rest.library.user_playlists.add_playlist(playlist_name);
                Command::from(message::MessageFuture {
                    inner: message::user_nav_message(message::NavMessage::PlaylistList(
                        "".to_string(),
                    )),
                })
            }
            message::Action::MakePlaylistDefault(playlist_id) => {
                app.rest
                    .library
                    .user_playlists
                    .make_playlist_default(playlist_id);
                Command::none()
            }
            message::Action::AddTracksToPlaylist(playlist_id, track_ids) => {
                for track_id in track_ids.into_iter() {
                    match app
                        .rest
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
                match app.rest.library.user_playlists.delete_playlist(playlist_id) {
                    Ok(_) => (),
                    Err(err_string) => println!("error deleting playlist: {}", err_string),
                };
                Command::none()
            }
            message::Action::RemoveTrackFromPlaylist(playlist_id, track_id) => {
                match app
                    .rest
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
                match app
                    .rest
                    .library
                    .user_playlists
                    .move_track_in_playlist(playlist_id, direction, track_id)
                {
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
                        app.rest
                            .play_queue_info
                            .gui
                            .track_info
                            .play_history
                            .remove(index);
                        app.rest.play_queue_info.rest.play_history.remove(index);
                    }
                    message::HistoryOrQueue::Queue => {
                        app.rest
                            .play_queue_info
                            .gui
                            .track_info
                            .play_queue
                            .remove(index);
                        app.rest.play_queue_info.rest.play_queue.remove(index);
                    }
                };
                Command::none()
            }
            message::Action::TogglePlayQueueVisible => {
                app.rest.play_queue_info.rest.play_queue_visible =
                    !app.rest.play_queue_info.rest.play_queue_visible;
                Command::none()
            }
            message::Action::UpdateText(new_text) => {
                match &mut app.rest.current_page {
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
            message::Action::PerformSearch(query) => match app.rest.current_page {
                state::Page::Search(ref _search_state) => Command::from(message::MessageFuture {
                    inner: message::user_nav_message(message::NavMessage::SearchPage(query, true)),
                }),
                _ => Command::none(),
            }
            message::Action::Close => {
                app.rest.should_close = true;
                Command::batch(vec![
                    Command::perform(
                        mpris_sender(
                            app.rest.player_info.rest.mpris_message_sender.clone(),
                            shared::MprisMessage::Close,
                        )
                        .send_message(),
                        Message::ErrorResponse,
                    ),
                    Command::perform(
                        sink_sender(
                            app.rest.player_info.rest.sink_message_sender.clone(),
                            shared::SinkMessage::Close,
                        )
                        .send_message(),
                        Message::ErrorResponse,
                    ),
                ])
            },
        },
        Message::Nav(nav_message) => {
            app.rest
                .page_back_history
                .push(app.rest.page_current_history.clone());
            app.rest.page_current_history = nav_message.clone();
            handle_nav(app, nav_message);
            Command::none()
        }
        Message::HistoryNav => {
            match app.rest.page_back_history.pop() {
                Some(history_message) => {
                    let old_current = app.rest.page_current_history.clone();
                    app.rest.page_current_history = history_message.clone();
                    app.rest.page_forward_history.insert(0, old_current);
                    handle_nav(app, history_message);
                }
                None => (),
            };
            Command::none()
        }
        Message::PlaybackRequest(internal) => {
            println!("GUI:\thandling internal: {:?}", internal);
            match internal {
                message::PlaybackRequest::LoadCurrentSong => {
                    match app.rest.player_info.rest.current_playback {
                        Some(ref outer_current_playback) => match outer_current_playback {
                            state::CurrentPlayback::Track(ref current_playback) => {
                                    app.rest.player_info.rest.playing = true;
                                    Command::batch(vec![
                                        Command::perform(
                                            mpris_sender(
                                                app.rest.player_info.rest.mpris_message_sender.clone(),
                                                shared::MprisMessage::SetMetadata(
                                                    current_playback.track.metadata.album_artist.clone(),
                                                    current_playback.track.metadata.title.clone(),
                                                ),
                                            )
                                            .send_message(),
                                            Message::ErrorResponse,
                                        ),
                                        Command::perform(
                                            {
                                                sink_sender(
                                                    app.rest.player_info.rest.sink_message_sender.clone(),
                                                    shared::SinkMessage::LoadSong(
                                                        current_playback.track.metadata.path.clone(),
                                                        app.rest.player_info.rest.current_volume,
                                                    ),
                                                )
                                                .send_message()
                                            },
                                            Message::ErrorResponse,
                                        ),
                                        Command::perform(
                                            tracker_sender(
                                                app.rest.player_info.rest.tracker_message_sender.clone(),
                                                shared::TrackerMessage::SongStarted(
                                                    current_playback.track.clone(),
                                                ),
                                            )
                                            .send_message(),
                                            Message::ErrorResponse,
                                        ),
                                    ])
                                },
                                state::CurrentPlayback::PauseBreak => {
                                    //app.rest.player_info.rest.playing = false;
                                    Command::from(message::MessageFuture {
                                        inner: Message::PlaybackRequest(message::PlaybackRequest::Pause),
                                    })
                                },
                            }
                        None => Command::none(),
                    }
                }
                message::PlaybackRequest::PlaySongs(tracks) => {
                    Command::from(message::MessageFuture {
                        inner: Message::PlaybackRequest(message::PlaybackRequest::InsertSongs(
                            tracks, true,
                        )),
                    })
                }
                message::PlaybackRequest::AppendSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(state::PlayQueueTrack { track: iter_track }));
                    }
                    for _track in new_songs_to_queue.iter() {
                        app.rest.play_queue_info.gui.track_info.play_queue.push(
                            state::PlayQueueGuiEntry {
                                remove_me_button: button::State::default(),
                                track_link_button: button::State::default(),
                            },
                        );
                    }
                    app.rest
                        .play_queue_info
                        .rest
                        .play_queue
                        .append(&mut new_songs_to_queue);
                    if load_next {
                        Command::from(message::MessageFuture {
                            inner: Message::PlaybackRequest(message::PlaybackRequest::Next),
                        })
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::InsertSongs(tracks, load_next) => {
                    let mut new_songs_to_queue = Vec::new();
                    for iter_track in tracks.into_iter() {
                        new_songs_to_queue.push(state::PlayQueueEntry::Track(state::PlayQueueTrack { track: iter_track }));
                    }
                    new_songs_to_queue.append(&mut app.rest.play_queue_info.rest.play_queue);
                    app.rest.play_queue_info.rest.play_queue = new_songs_to_queue;

                    let mut new_song_buttons = Vec::new();
                    for _new_song in app.rest.play_queue_info.rest.play_queue.iter() {
                        new_song_buttons.push(state::PlayQueueGuiEntry {
                            remove_me_button: button::State::default(),
                            track_link_button: button::State::default(),
                        });
                    }
                    app.rest.play_queue_info.gui.track_info.play_queue = new_song_buttons;

                    if load_next {
                        Command::from(message::MessageFuture {
                            inner: Message::PlaybackRequest(message::PlaybackRequest::Next),
                        })
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Prev => {
                    if app.rest.play_queue_info.rest.play_history.len() > 0 {
                        match app.rest.player_info.rest.current_playback {
                            Some(ref current_playback) => {
                                let mut new_play_queue = vec![state::PlayQueueEntry::from_playback(current_playback)];
                                new_play_queue
                                    .append(&mut app.rest.play_queue_info.rest.play_queue);
                                app.rest.play_queue_info.rest.play_queue = new_play_queue;

                                app.rest.play_queue_info.gui.track_info.play_queue.push(
                                    state::PlayQueueGuiEntry {
                                        remove_me_button: button::State::default(),
                                        track_link_button: button::State::default(),
                                    },
                                );
                            }
                            None => (),
                        };
                        app.rest
                            .play_queue_info
                            .gui
                            .track_info
                            .play_history
                            .pop()
                            .unwrap();
                        let track = app.rest.play_queue_info.rest.play_history.pop().unwrap();
                        app.rest.player_info.rest.current_playback = Some(match track {
                            state::PlayQueueEntry::Track(ref t) => state::CurrentPlayback::Track(state::CurrentTrackPlayback {
                                track: t.track.clone(),
                                current_second: 0,
                            }),
                            state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => state::CurrentPlayback::PauseBreak,
                        });
                        app.rest.play_queue_info.rest.current_playback = Some(track.clone());
                        Command::from(message::MessageFuture {
                            inner: Message::PlaybackRequest(
                                message::PlaybackRequest::LoadCurrentSong,
                            ),
                        })
                    } else {
                        Command::none()
                    }
                }
                message::PlaybackRequest::Next => {
                    if app.rest.play_queue_info.rest.play_queue.len() > 0 {
                        match app.rest.player_info.rest.current_playback {
                            Some(ref current_playback) => {
                                app.rest.play_queue_info.gui.track_info.play_history.push(
                                    state::PlayQueueGuiEntry {
                                        remove_me_button: button::State::default(),
                                        track_link_button: button::State::default(),
                                    },
                                );
                                app.rest.play_queue_info.rest.play_history.push(
                                    state::PlayQueueEntry::from_playback(current_playback),
                                )
                            }
                            None => (),
                        };

                        app.rest.play_queue_info.gui.track_info.play_queue.remove(0);
                        let track = app.rest.play_queue_info.rest.play_queue.remove(0);
                        app.rest.player_info.rest.current_playback = Some(state::CurrentPlayback::from_entry_zeroed(&track));
                        app.rest.play_queue_info.rest.current_playback = Some(track.clone());
                        Command::from(message::MessageFuture {
                            inner: Message::PlaybackRequest(
                                message::PlaybackRequest::LoadCurrentSong,
                            ),
                        })
                    } else {
                        match app.rest.player_info.rest.current_playback {
                            Some(ref current_playback) => {
                                app.rest.play_queue_info.gui.track_info.play_history.push(
                                    state::PlayQueueGuiEntry {
                                        remove_me_button: button::State::default(),
                                        track_link_button: button::State::default(),
                                    },
                                );
                                app.rest.play_queue_info.rest.play_history.push(
                                    state::PlayQueueEntry::from_playback(current_playback),
                                )
                            }
                            None => (),
                        };
                        app.rest.player_info.rest.current_playback = None;
                        app.rest.play_queue_info.rest.current_playback = None;
                        Command::perform(
                            mpris_sender(
                                app.rest.player_info.rest.mpris_message_sender.clone(),
                                shared::MprisMessage::SetStopped,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        )
                    }
                }
                message::PlaybackRequest::Play => {
                    app.rest.player_info.rest.playing = true;
                    Command::batch(vec![
                        Command::perform(
                            mpris_sender(
                                app.rest.player_info.rest.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPlaying,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            sink_sender(
                                app.rest.player_info.rest.sink_message_sender.clone(),
                                shared::SinkMessage::PlayButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::Pause => {
                    app.rest.player_info.rest.playing = false;
                    Command::batch(vec![
                        Command::perform(
                            mpris_sender(
                                app.rest.player_info.rest.mpris_message_sender.clone(),
                                shared::MprisMessage::SetPaused,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                        Command::perform(
                            sink_sender(
                                app.rest.player_info.rest.sink_message_sender.clone(),
                                shared::SinkMessage::PauseButton,
                            )
                            .send_message(),
                            Message::ErrorResponse,
                        ),
                    ])
                }
                message::PlaybackRequest::InsertPause => {
                    let mut new_songs_to_queue = vec![state::PlayQueueEntry::Action(state::PlayQueueAction::Pause)];
                    new_songs_to_queue.append(&mut app.rest.play_queue_info.rest.play_queue);
                    app.rest.play_queue_info.rest.play_queue = new_songs_to_queue;

                    app.rest.play_queue_info.gui.track_info.play_queue.push(
                        state::PlayQueueGuiEntry{
                            remove_me_button: button::State::default(),
                            track_link_button: button::State::default(),
                        },
                    );

                    Command::none()
                }
            }
        }
        Message::SinkCallback(callback) => match callback {
            shared::SinkCallbackMessage::SongEnded => Command::from(message::MessageFuture {
                inner: Message::PlaybackRequest(message::PlaybackRequest::Next),
            }),
            shared::SinkCallbackMessage::SecondElapsed => {
                match app.rest.player_info.rest.current_playback {
                    Some(ref mut outer_current_playback) => match outer_current_playback {
                        state::CurrentPlayback::Track(ref mut current_playback) => current_playback.current_second += 1,
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
        Message::MprisCallback(callback) => Command::from(match callback {
            shared::MprisCallbackMessage::PlayPause => {
                if app.rest.player_info.rest.playing {
                    message::MessageFuture {
                        inner: Message::PlaybackRequest(message::PlaybackRequest::Pause),
                    }
                } else {
                    message::MessageFuture {
                        inner: Message::PlaybackRequest(message::PlaybackRequest::Play),
                    }
                }
            }
            shared::MprisCallbackMessage::Play => message::MessageFuture {
                inner: Message::PlaybackRequest(message::PlaybackRequest::Play),
            },
            shared::MprisCallbackMessage::Pause => message::MessageFuture {
                inner: Message::PlaybackRequest(message::PlaybackRequest::Pause),
            },
            shared::MprisCallbackMessage::Prev => message::MessageFuture {
                inner: Message::PlaybackRequest(message::PlaybackRequest::Prev),
            },
            shared::MprisCallbackMessage::Next => message::MessageFuture {
                inner: Message::PlaybackRequest(message::PlaybackRequest::Next),
            },
        }),
    }
}

fn handle_volume_request(
    app: &mut Loaded,
    volume_request: message::VolumeRequest,
) -> Command<Message> {
    match volume_request {
        message::VolumeRequest::Up(delta) => app.rest.player_info.rest.current_volume += delta,
        message::VolumeRequest::Down(delta) => app.rest.player_info.rest.current_volume -= delta,
        message::VolumeRequest::Set(new_volume) => {
            app.rest.player_info.rest.current_volume = new_volume
        }
    };
    Command::perform(
        {
            sink_sender(
                app.rest.player_info.rest.sink_message_sender.clone(),
                shared::SinkMessage::SetVolume(app.rest.player_info.rest.current_volume),
            )
            .send_message()
        },
        Message::ErrorResponse,
    )
}

fn handle_nav(app: &mut Loaded, nav_message: message::NavMessage) {
    match nav_message {
        NavMessage::Home => {
            app.rest.current_page = Page::Home(state::HomeState {
                artist_list_button: button::State::default(),
                album_list_button: button::State::default(),
                track_list_button: button::State::default(),
                playlist_list_button: button::State::default(),
                search_button: button::State::default(),
                scroll: scrollable::State::default(),
            })
        }
        NavMessage::Config => {
            app.rest.current_page = Page::Config(state::ConfigState {
                refresh_library_button: button::State::default(),
            })
        }
        NavMessage::PlayQueueFocus => {
            app.rest.current_page = Page::PlayQueue(state::PlayQueueState {});
        }
        NavMessage::PlaylistView(playlist_id) => {
            let buttons = match app.rest.library.user_playlists.get(playlist_id) {
                Some(playlist) => playlist
                    .tracks
                    .iter()
                    .map(|_| state::PlaylistTrackLineItemButtons {
                        play_button: button::State::default(),
                        link_button: button::State::default(),
                        remove_from_playlist_button: button::State::default(),
                        move_down_in_playlist_button: button::State::default(),
                        move_up_in_playlist_button: button::State::default(),
                        insert_button: button::State::default(),
                        append_button: button::State::default(),
                    })
                    .collect(),
                None => Vec::new(),
            };
            app.rest.current_page = Page::PlaylistView(state::PlaylistViewState {
                playlist_play_queue_buttons: state::PlayQueueInteractionButtons {
                    play_button: button::State::default(),
                    insert_button: button::State::default(),
                    append_button: button::State::default(),
                },
                track_play_buttons: buttons,
                playlist_list_breadcrumb: button::State::default(),
                this_playlist_breadcrumb: button::State::default(),
                track_scroll: scrollable::State::default(),
                playlist_id: playlist_id,
            });
        }
        NavMessage::PlaylistList(new_playlist_name) => {
            app.rest.current_page = Page::PlaylistList(state::PlaylistListState {
                playlist_scroll: scrollable::State::default(),
                playlist_list_breadcrumb: button::State::default(),
                playlist_make_default_buttons: app
                    .rest
                    .library
                    .user_playlists
                    .to_vec()
                    .iter()
                    .map(|_| state::PlaylistListButtons {
                        link_to_playlist_button: button::State::default(),
                        delete_playlist_button: button::State::default(),
                        make_default_button: button::State::default(),
                    })
                    .collect(),
                new_playlist_name: new_playlist_name,
                new_playlist_text_input: text_input::State::default(),
                new_playlist_button: button::State::default(),
            });
        }
        NavMessage::SearchPage(query, perform_search) => {
            let computed_results = match perform_search {
                true => {
                    let search_results = app.rest.library.search(query.clone());
                    let mapped_search_results = model::SearchResults {
                        artists: search_results
                            .artists
                            .into_iter()
                            .map(|artist| model::Pair {
                                first: artist.first,
                                second: button::State::default(),
                            })
                            .collect(),
                        albums: search_results
                            .albums
                            .into_iter()
                            .map(|artist_album| model::Pair {
                                first: artist_album.first,
                                second: button::State::default(),
                            })
                            .collect(),
                        tracks: search_results
                            .tracks
                            .into_iter()
                            .map(|track| model::Pair {
                                first: track.first,
                                second: button::State::default(),
                            })
                            .collect(),
                        track_artists: search_results
                            .track_artists
                            .into_iter()
                            .map(|track_artist| model::Pair {
                                first: track_artist.first,
                                second: button::State::default(),
                            })
                            .collect(),
                    };
                    mapped_search_results
                }
                false => model::SearchResults {
                    artists: Vec::new(),
                    albums: Vec::new(),
                    tracks: Vec::new(),
                    track_artists: Vec::new(),
                },
            };

            app.rest.current_page = Page::Search(state::SearchPageState {
                query: query,
                artist_scroll: scrollable::State::default(),
                album_scroll: scrollable::State::default(),
                track_scroll: scrollable::State::default(),
                track_artist_scroll: scrollable::State::default(),
                input_state: text_input::State::focused(),
                results: computed_results,
            });
        }
        NavMessage::TrackList(page, sort, sort_order) => {
            app.rest.current_page = Page::TrackList(state::TrackListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
                album_list_breadcrumb: button::State::default(),
                sort_order_regular_button: button::State::default(),
                sort_order_reverse_button: button::State::default(),
                sort_by_name_button: button::State::default(),
                sort_by_play_count_button: button::State::default(),
                sort_by_duration_button: button::State::default(),
                sort_by_played_duration_button: button::State::default(),
                sort_random_button: button::State::default(),
                nav_first_button: button::State::default(),
                nav_back_button: button::State::default(),
                nav_forward_button: button::State::default(),
                nav_last_button: button::State::default(),
                album_buttons: vec![
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                ],
                track_scroll: scrollable::State::default(),
            });
        }
        NavMessage::AlbumList(page, sort, sort_order) => {
            app.rest.current_page = Page::AlbumList(state::AlbumListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
                album_list_breadcrumb: button::State::default(),
                sort_order_regular_button: button::State::default(),
                sort_order_reverse_button: button::State::default(),
                sort_by_name_button: button::State::default(),
                sort_by_date_button: button::State::default(),
                sort_by_duration_button: button::State::default(),
                sort_by_last_mod_button: button::State::default(),
                sort_by_total_play_count_button: button::State::default(),
                sort_by_total_played_duration_button: button::State::default(),
                sort_random_button: button::State::default(),
                sort_by_artist_button: button::State::default(),
                nav_first_button: button::State::default(),
                nav_back_button: button::State::default(),
                nav_forward_button: button::State::default(),
                nav_last_button: button::State::default(),
                album_buttons: vec![
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                ],
                album_scroll: scrollable::State::default(),
            });
        }
        NavMessage::ArtistList(page, sort, sort_order) => {
            app.rest.current_page = Page::ArtistList(state::ArtistListState {
                page: page,
                sort_key: sort,
                sort_order: sort_order,
                artist_list_breadcrumb: button::State::default(),
                sort_order_regular_button: button::State::default(),
                sort_order_reverse_button: button::State::default(),

                sort_by_name_button: button::State::default(),
                sort_random_button: button::State::default(),
                sort_by_play_count_button: button::State::default(),
                sort_by_album_count_button: button::State::default(),
                sort_by_track_count_button: button::State::default(),
                sort_by_track_duration_button: button::State::default(),
                sort_by_duration_played_button: button::State::default(),

                nav_first_button: button::State::default(),
                nav_back_button: button::State::default(),
                nav_forward_button: button::State::default(),
                nav_last_button: button::State::default(),
                artist_buttons: vec![
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                    button::State::default(),
                ],
                artist_scroll: scrollable::State::default(),
            });
        }
        NavMessage::ArtistView(artist_id) => {
            app.rest.current_page = Page::ArtistView(state::ArtistViewState {
                artist_list_breadcrumb: button::State::default(),
                artist_view_breadcrumb: button::State::default(),
                artist_id: artist_id.clone(),

                album_view_button: button::State::default(),
                track_view_button: button::State::default(),

                album_buttons: app
                    .rest
                    .library
                    .get_artist_map()
                    .get(&artist_id)
                    .unwrap()
                    .albums
                    .keys()
                    .map(|k| (k.clone(), button::State::default()))
                    .collect(),
                album_scroll: scrollable::State::default(),
            });
        }
        NavMessage::ArtistTrackView(artist_id, sort_key, sort_order) => {
            let mut track_buttons = Vec::new();
            for album in app
                .rest
                .library
                .get_artist_map()
                .get(&artist_id)
                .unwrap()
                .albums
                .values()
            {
                for disc in album.discs.values() {
                    for _track in disc.tracks.values() {
                        track_buttons.push(
                            button::State::default(),
                        );
                    }
                }
            }
            app.rest.current_page = Page::ArtistTrackView(state::ArtistTrackViewState {
                artist_list_breadcrumb: button::State::default(),
                artist_view_breadcrumb: button::State::default(),

                sort_by_name_button: button::State::default(),
                sort_by_album_button: button::State::default(),
                sort_by_play_count_button: button::State::default(),
                sort_by_duration_button: button::State::default(),
                sort_by_played_duration_button: button::State::default(),
                sort_random_button: button::State::default(),

                album_view_button: button::State::default(),
                track_view_button: button::State::default(),

                sort_order_regular_button: button::State::default(),
                sort_order_reverse_button: button::State::default(),

                track_buttons: track_buttons,
                track_scroll: scrollable::State::default(),

                artist_id: artist_id.clone(),

                sort_key: sort_key,
                sort_order: sort_order,
            });
        }
        NavMessage::ArtistAlbumView(artist_id, album_id, album_size, maybe_selected_track) => {
            app.rest.current_page = Page::ArtistAlbumView(state::ArtistAlbumViewState {
                artist_list_breadcrumb: button::State::default(),
                artist_view_breadcrumb: button::State::default(),
                artist_album_view_breadcrumb: button::State::default(),
                artist_id: artist_id.clone(),
                album_id: album_id.clone(),
                album_size: album_size,
                maybe_selected_track: maybe_selected_track,
                toggle_image_size_button: button::State::default(),
                entire_track_list_buttons: state::TrackLineItemButtons {
                    play_button: button::State::default(),
                    play_all_from_here_button: button::State::default(),
                    insert_button: button::State::default(),
                    append_button: button::State::default(),
                    add_to_default_playlist_button: button::State::default(),
                },
                all_disc_buttons: app
                    .rest
                    .library
                    .get_artist_album_tracks(artist_id.clone(), album_id.clone())
                    .discs
                    .keys()
                    .map(|_disc| state::TrackLineItemButtons {
                        play_button: button::State::default(),
                        play_all_from_here_button: button::State::default(),
                        add_to_default_playlist_button: button::State::default(),
                        insert_button: button::State::default(),
                        append_button: button::State::default(),
                    })
                    .collect(),
                track_play_buttons: app
                    .rest
                    .library
                    .get_artist_album_tracks(artist_id.clone(), album_id.clone())
                    .discs
                    .values()
                    .map(|tracks| {
                        tracks
                            .tracks
                            .values()
                            .map(|_track| state::TrackLineItemButtons {
                                play_button: button::State::default(),
                                play_all_from_here_button: button::State::default(),
                                add_to_default_playlist_button: button::State::default(),
                                insert_button: button::State::default(),
                                append_button: button::State::default(),
                            })
                            .collect()
                    })
                    .collect(),
                scroll: scrollable::State::default(),
            });
        }
    };
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
