use crate::shared;

pub fn handle_playback_request(
    play_queue: &mut shared::PlayQueueInfo,
    sink_client: shared::Client<shared::SinkMessage>,
    mpris_client: shared::Client<shared::MprisMessage>,
    tracker_client: shared::Client<shared::TrackerMessage>,
    internal: shared::PlaybackRequest,
) {
    println!("GUI:\thandling internal: {:?}", internal);
    match internal {
        shared::PlaybackRequest::LoadCurrentSong => match play_queue.current_playback {
            Some(ref outer_current_playback) => match outer_current_playback {
                shared::CurrentPlayback::Track(ref current_playback) => {
                    sink_client
                        .send(shared::SinkMessage::LoadSong(
                            current_playback.track.metadata.path.clone(),
                            play_queue.current_volume,
                        ))
                        .unwrap();
                    let _ = mpris_client.send(shared::MprisMessage::SetMetadata(
                        current_playback.track.metadata.album_artist.clone(),
                        current_playback.track.metadata.title.clone(),
                    ));
                    let _ = tracker_client.send(shared::TrackerMessage::SongStarted(
                        current_playback.track.clone(),
                    ));
                }
                shared::CurrentPlayback::PauseBreak => {
                    play_queue.playing = false;
                    handle_playback_request(
                        play_queue,
                        sink_client,
                        mpris_client,
                        tracker_client,
                        shared::PlaybackRequest::Pause,
                    );
                }
            },
            None => println!("Nothing to load for current song"),
        },
        shared::PlaybackRequest::PlaySongs(tracks) => handle_playback_request(
            play_queue,
            sink_client,
            mpris_client,
            tracker_client,
            shared::PlaybackRequest::InsertSongs(tracks, true),
        ),
        shared::PlaybackRequest::AppendSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(shared::PlayQueueEntry::Track(shared::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            play_queue.play_queue.append(&mut new_songs_to_queue);
            if load_next {
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    tracker_client,
                    shared::PlaybackRequest::Next,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::InsertSongs(tracks, load_next) => {
            let mut new_songs_to_queue = Vec::new();
            for iter_track in tracks.into_iter() {
                new_songs_to_queue.push(shared::PlayQueueEntry::Track(shared::PlayQueueTrack {
                    track: iter_track,
                }));
            }
            new_songs_to_queue.append(&mut play_queue.play_queue);
            play_queue.play_queue = new_songs_to_queue;

            if load_next {
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    tracker_client,
                    shared::PlaybackRequest::Next,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::Prev => {
            if play_queue.play_history.len() > 0 {
                match play_queue.current_playback {
                    Some(ref current_playback) => {
                        let mut new_play_queue =
                            vec![current_playback.clone().to_play_queue_entry()];
                        new_play_queue.append(&mut play_queue.play_queue);
                        play_queue.play_queue = new_play_queue;
                    }
                    None => (),
                };
                let track = play_queue.play_history.pop().unwrap();
                play_queue.current_playback = Some(shared::CurrentPlayback::from_shared(track, 0));
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    tracker_client,
                    shared::PlaybackRequest::LoadCurrentSong,
                );
            } else {
                // Nothing else to run if not loading next
            }
        }
        shared::PlaybackRequest::Next => {
            if play_queue.play_queue.len() > 0 {
                match play_queue.current_playback {
                    Some(ref current_playback) => play_queue
                        .play_history
                        .push(current_playback.clone().to_play_queue_entry()),
                    None => (),
                };

                let track = play_queue.play_queue.remove(0);
                //play_queue.current_playback = Some(state::CurrentPlayback::from_entry_zeroed(&track));
                play_queue.current_playback = Some(shared::CurrentPlayback::from_shared(track, 0));
                handle_playback_request(
                    play_queue,
                    sink_client,
                    mpris_client,
                    tracker_client,
                    shared::PlaybackRequest::LoadCurrentSong,
                );
            } else {
                match play_queue.current_playback {
                    Some(ref current_playback) => play_queue
                        .play_history
                        .push(current_playback.clone().to_play_queue_entry()),
                    None => (),
                };
                play_queue.current_playback = None;
                let _ = mpris_client.send(shared::MprisMessage::SetStopped);
            }
        }
        shared::PlaybackRequest::Play => {
            play_queue.playing = true;
            let _ = sink_client.send(shared::SinkMessage::PlayButton);
            let _ = mpris_client.send(shared::MprisMessage::SetPlaying);
        }
        shared::PlaybackRequest::Pause => {
            play_queue.playing = false;
            let _ = sink_client.send(shared::SinkMessage::PauseButton);
            let _ = mpris_client.send(shared::MprisMessage::SetPaused);
        }
        shared::PlaybackRequest::PlayPauseToggle => {
            if play_queue.playing {
                play_queue.playing = false;
                let _ = sink_client.send(shared::SinkMessage::PauseButton);
                let _ = mpris_client.send(shared::MprisMessage::SetPaused);
            } else {
                play_queue.playing = true;
                let _ = sink_client.send(shared::SinkMessage::PlayButton);
                let _ = mpris_client.send(shared::MprisMessage::SetPlaying);
            }
        }
        shared::PlaybackRequest::InsertPause => {
            let mut new_songs_to_queue = vec![shared::PlayQueueEntry::Action(
                shared::PlayQueueAction::Pause,
            )];
            new_songs_to_queue.append(&mut play_queue.play_queue);
            play_queue.play_queue = new_songs_to_queue;
        }
        shared::PlaybackRequest::SetVolume(new_volume) => {
            let _ = sink_client.send(shared::SinkMessage::SetVolume(new_volume));
        }

        shared::PlaybackRequest::RemoveTrackFromPlayQueue(history_or_queue, index) => {
            match history_or_queue {
                shared::HistoryOrQueue::History => {
                    play_queue.play_history.remove(index);
                }
                shared::HistoryOrQueue::Queue => {
                    play_queue.play_queue.remove(index);
                }
            };
        }

        shared::PlaybackRequest::Close => {
            let _ = sink_client.send(shared::SinkMessage::Close);
            let _ = mpris_client.send(shared::MprisMessage::Close);
        }
    }
}
