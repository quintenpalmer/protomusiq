use iced::widget::{Column, Container, Row, Scrollable, Space};
use iced::{Alignment, Element, Length};

use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::model;
use crate::shared;
use crate::state;

use super::common;
use super::components;
use super::consts;
use super::elements::*;
use super::page;
use super::style;

pub fn view_loading<'a>() -> Element<'a, Message> {
    Container::new(Container::new(h1("Loading, thank you for your patience...")).padding(100))
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn view_app(app: &state::AppState) -> Element<Message> {
    println!(
        "GUI:\tviewing: {}",
        app.current_page.super_simple_debug_string()
    );
    let library = &app.library;
    let movie_library = &app.video_library;
    let config = &app.config.rest;
    let messages = &app.messages;
    let app_images = &app.app_images;
    let current_page = &app.current_page;
    let action_state = &app.action_state;
    let player_info = &app.player_info;
    let play_queue_info = &app.play_queue_info;

    let (additional_breadcrumbs, rendered_page) = page::render_page(
        current_page,
        &library,
        &movie_library,
        &app_images,
        &action_state,
        &play_queue_info,
        &player_info,
    );

    let header = render_header(additional_breadcrumbs, messages);

    let (play_queue_view, play_queue_expanded) = render_play_queue(&library, &play_queue_info);

    let playthrough = render_playthrough(&player_info.current_playback);

    let player_controls = render_player_controls(player_info, &library);

    render_entire_page(
        config,
        header,
        rendered_page,
        play_queue_view,
        play_queue_expanded,
        playthrough,
        player_controls,
    )
}

pub fn render_entire_page<'a>(
    config: &model::app::AppConfigState,
    header: Container<'a, Message>,
    rendered_page: Container<'a, Message>,
    play_queue_view: Container<'a, Message>,
    play_queue_expanded: bool,
    playthrough: Option<Container<'a, Message>>,
    player_controls: Option<Container<'a, Message>>,
) -> Element<'a, Message> {
    let mut ret = Column::new()
        .push(
            Container::new(header.padding(5).height(Length::Fixed(50.0))).style(
                iced::theme::Container::Custom(Box::new(style::ContainerPopForward)),
            ),
        )
        .push({
            let row = match play_queue_expanded {
                true => Row::new()
                    .padding(10)
                    .push(rendered_page.width(Length::FillPortion(config.split_ratio_left)))
                    .push(play_queue_view.width(Length::FillPortion(config.split_ratio_right))),
                false => Row::new()
                    .padding(10)
                    .push(rendered_page.width(Length::Fill))
                    .push(play_queue_view),
            };

            row.height(Length::Fill)
        });
    match playthrough {
        Some(through) => {
            ret = ret.push(through.height(Length::Fixed(15.0)));
        }
        None => (),
    };
    match player_controls {
        Some(controls) => {
            ret = ret.push(Container::new(controls.height(Length::Fixed(70.0))).style(
                iced::theme::Container::Custom(Box::new(style::ContainerPopForward)),
            ));
        }
        None => (),
    };

    ret.width(Length::Fill).into()
}

pub fn render_header<'a>(
    additional_breadcrumbs: Vec<(String, Message)>,
    messages: &Vec<state::MessageInfo>,
) -> Container<'a, Message> {
    let mut breadcrumbs: Vec<Element<Message>> = vec![dark_button(bright_paragraph("Home"))
        .on_press(user_nav_message(NavMessage::Home))
        .into()];

    breadcrumbs.extend(
        additional_breadcrumbs
            .into_iter()
            .map(|(crumb_text, on_press)| {
                dark_button(bright_paragraph(crumb_text))
                    .on_press(on_press)
                    .into()
            })
            .collect::<Vec<Element<Message>>>(),
    );

    let back_forward_buttons =
        Row::new().push(dark_button(bright_paragraph("<")).on_press(Message::HistoryNav));

    let header = Row::new().push(
        line_row()
            .push(
                line_row()
                    .push(back_forward_buttons)
                    .push({
                        let mut row = line_row().padding(5);
                        for crumb in breadcrumbs.into_iter() {
                            row = row.push(paragraph(" > "));
                            row = row.push(crumb);
                        }
                        row
                    })
                    .width(Length::Fill),
            )
            .push(
                line_row()
                    .push({
                        let row = Row::new().align_items(Alignment::Center);
                        match messages.as_slice() {
                            [.., last] => match last.notification_type {
                                message::NotificationAction::AddedToPlayQueue(ref title) => row
                                    .push(
                                        green_notification(format!(
                                            "+queue: {}",
                                            common::abr_str(title.clone(), 20)
                                        ))
                                        .width(
                                            Length::Fixed(consts::NOTIFICATION_TEXT_PIXEL_WIDTH),
                                        ),
                                    )
                                    .push(
                                        dark_button("-")
                                            .on_press(message::Message::Action(
                                                message::Action::Notify(
                                                    message::NotificationMessage::PopOnScreen,
                                                ),
                                            ))
                                            .width(Length::Fixed(consts::SINGLE_CHAR_WIDTH)),
                                    )
                                    .push(
                                        dark_button("x")
                                            .on_press(message::Message::Action(
                                                message::Action::Notify(
                                                    message::NotificationMessage::ClearOnScreen,
                                                ),
                                            ))
                                            .width(Length::Fixed(consts::SINGLE_CHAR_WIDTH)),
                                    ),
                            },
                            _ => row
                                .push(
                                    grey_notification("<no history> ").width(Length::Fixed(
                                        consts::NOTIFICATION_TEXT_PIXEL_WIDTH,
                                    )),
                                )
                                .push(Space::with_width(Length::Fixed(consts::SINGLE_CHAR_WIDTH)))
                                .push(Space::with_width(Length::Fixed(consts::SINGLE_CHAR_WIDTH))),
                        }
                    })
                    .push(
                        dark_button(bright_paragraph("Search")).on_press(user_nav_message(
                            message::NavMessage::SearchPage(
                                "".to_string(),
                                message::SearchDomain::Music,
                                false,
                            ),
                        )),
                    )
                    .push(
                        dark_button(bright_paragraph("Settings"))
                            .on_press(user_nav_message(message::NavMessage::Config)),
                    )
                    .push(
                        dark_button(bright_paragraph("X"))
                            .on_press(Message::Action(message::Action::Close)),
                    ),
            ),
    );
    Container::new(header)
}

pub fn render_play_queue<'a>(
    library: &'a model::LibraryState,
    play_queue_info: &'a state::PlayQueueInfo,
) -> (Container<'a, Message>, bool) {
    match play_queue_info.play_queue_visible {
        true => {
            let mut play_queue_view = Column::new().spacing(5).padding(10).push(
                iced::widget::Row::new()
                    .align_items(iced::Alignment::Start)
                    .push(h1("Current Playback").width(Length::Fill))
                    .push(
                        dark_button(bright_paragraph("Focus"))
                            .on_press(user_nav_message(NavMessage::PlayQueueFocus)),
                    )
                    .push(
                        dark_button(bright_paragraph(">"))
                            .on_press(Message::Action(message::Action::TogglePlayQueueVisible)),
                    ),
            );
            let mut play_queue_column = Column::new();
            let mut stripe_marker = false;

            for (index, play_queue_entry) in play_queue_info.play_history.iter().enumerate() {
                stripe_marker = !stripe_marker;
                play_queue_column = play_queue_column.push(
                    Container::new(match play_queue_entry {
                        state::PlayQueueEntry::Track(play_queue_track) => line_row()
                            .spacing(5)
                            .push(album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Micro,
                                    play_queue_track.track.metadata.album_artist_id.clone(),
                                    play_queue_track.track.metadata.album_id.clone(),
                                ),
                                model::AlbumSize::Micro,
                            ))
                            .push(
                                dark_button(bright_paragraph(
                                    play_queue_track.track.metadata.title.clone(),
                                ))
                                .on_press(components::track_link(&play_queue_track.track.metadata))
                                .width(Length::Fill),
                            )
                            .push(dark_button(bright_paragraph("-")).on_press(
                                message::Message::PlaybackRequest(
                                    shared::PlaybackRequest::RemoveTrackFromPlayQueue(
                                        shared::HistoryOrQueue::History,
                                        index,
                                    ),
                                ),
                            ))
                            .push(dark_paragraph(common::format_duration(
                                play_queue_track.track.metadata.duration.as_secs(),
                            ))),
                        state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => line_row()
                            .spacing(5)
                            .push(bright_paragraph("Paused").width(Length::Fill))
                            .push(dark_button(bright_paragraph("-")).on_press(
                                message::Message::PlaybackRequest(
                                    shared::PlaybackRequest::RemoveTrackFromPlayQueue(
                                        shared::HistoryOrQueue::History,
                                        index,
                                    ),
                                ),
                            )),
                    })
                    .padding(2)
                    .style(iced::theme::Container::Custom(
                        style::get_stripe_style(stripe_marker),
                    )),
                );
            }
            match play_queue_info.current_playback {
                Some(ref outer_current_playback) => match outer_current_playback {
                    state::PlayQueueEntry::Track(ref current_playback) => {
                        stripe_marker = !stripe_marker;
                        play_queue_column = play_queue_column.push(
                            Container::new(
                                line_row()
                                    .spacing(5)
                                    .push(album_image(
                                        library.get_album_cover(
                                            model::AlbumSize::Micro,
                                            current_playback.track.metadata.album_artist_id.clone(),
                                            current_playback.track.metadata.album_id.clone(),
                                        ),
                                        model::AlbumSize::Micro,
                                    ))
                                    .push(
                                        bright_paragraph(
                                            current_playback.track.metadata.title.clone(),
                                        )
                                        .width(Length::Fill),
                                    )
                                    .push(bright_paragraph(common::format_duration(
                                        current_playback.track.metadata.duration.as_secs(),
                                    ))),
                            )
                            .padding(2)
                            .style(iced::theme::Container::Custom(Box::new(
                                style::ContainerStripeHighlight {},
                            ))),
                        );
                    }
                    state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => {
                        stripe_marker = !stripe_marker;
                        play_queue_column = play_queue_column.push(
                            Container::new(
                                line_row()
                                    .spacing(5)
                                    .push(bright_paragraph("Paused").width(Length::Fill)),
                            )
                            .padding(2)
                            .style(iced::theme::Container::Custom(Box::new(
                                style::ContainerStripeHighlight {},
                            ))),
                        );
                    }
                },
                None => (),
            };
            for (index, play_queue_entry) in play_queue_info.play_queue.iter().enumerate() {
                stripe_marker = !stripe_marker;
                play_queue_column = play_queue_column.push(
                    Container::new(match play_queue_entry {
                        state::PlayQueueEntry::Track(play_queue_track) => line_row()
                            .spacing(5)
                            .push(album_image(
                                library.get_album_cover(
                                    model::AlbumSize::Micro,
                                    play_queue_track.track.metadata.album_artist_id.clone(),
                                    play_queue_track.track.metadata.album_id.clone(),
                                ),
                                model::AlbumSize::Micro,
                            ))
                            .push(
                                dark_button(bright_paragraph(
                                    play_queue_track.track.metadata.title.clone(),
                                ))
                                .on_press(components::track_link(&play_queue_track.track.metadata))
                                .width(Length::Fill),
                            )
                            .push(dark_button(bright_paragraph("-")).on_press(
                                message::Message::PlaybackRequest(
                                    shared::PlaybackRequest::RemoveTrackFromPlayQueue(
                                        shared::HistoryOrQueue::Queue,
                                        index,
                                    ),
                                ),
                            ))
                            .push(dark_paragraph(common::format_duration(
                                play_queue_track.track.metadata.duration.as_secs(),
                            ))),
                        state::PlayQueueEntry::Action(state::PlayQueueAction::Pause) => line_row()
                            .spacing(5)
                            .push(bright_paragraph("Paused").width(Length::Fill))
                            .push(dark_button(bright_paragraph("-")).on_press(
                                message::Message::PlaybackRequest(
                                    shared::PlaybackRequest::RemoveTrackFromPlayQueue(
                                        shared::HistoryOrQueue::History,
                                        index,
                                    ),
                                ),
                            )),
                    })
                    .padding(2)
                    .style(iced::theme::Container::Custom(
                        style::get_stripe_style(stripe_marker),
                    )),
                );
            }
            play_queue_view = play_queue_view.push(Scrollable::new(play_queue_column));
            (
                Container::new(Container::new(play_queue_view).height(Length::Fill).style(
                    iced::theme::Container::Custom(Box::new(style::ContainerPopMidForward {})),
                ))
                .height(Length::Fill),
                true,
            )
        }
        false => (
            Container::new(
                Column::new().spacing(5).padding(5).push(
                    dark_button(bright_paragraph("<"))
                        .on_press(Message::Action(message::Action::TogglePlayQueueVisible)),
                ),
            ),
            false,
        ),
    }
}

pub fn render_playthrough(
    maybe_current_playback: &Option<state::CurrentPlayback>,
) -> Option<Container<'static, Message>> {
    match maybe_current_playback {
        Some(ref outer_current_playback) => match outer_current_playback {
            state::CurrentPlayback::Track(ref current_playback) => {
                let playback_marker_pre_fill_portion = (1 + current_playback.current_second) as u16;
                let playback_marker_post_fill_portion =
                    (1 + current_playback.track.metadata.duration.as_secs()
                        - current_playback.current_second) as u16;
                Some(
                    Container::new(
                        Row::new()
                            .push(
                                Container::new(Space::new(Length::Fill, Length::Shrink))
                                    .style(iced::theme::Container::Custom(Box::new(
                                        style::ContainerPlaybackPlayedThrough,
                                    )))
                                    .height(Length::Fill)
                                    .width(Length::FillPortion(playback_marker_pre_fill_portion)),
                            )
                            .push(
                                Container::new(Space::new(Length::Fill, Length::Fill))
                                    .style(iced::theme::Container::Custom(Box::new(
                                        style::ContainerPlaybackToPlayThrough,
                                    )))
                                    .height(Length::Fill)
                                    .width(Length::FillPortion(playback_marker_post_fill_portion)),
                            ),
                    )
                    .width(Length::Fill),
                )
            }
            state::CurrentPlayback::PauseBreak => None,
        },
        None => None,
    }
}

pub fn render_player_controls<'a>(
    player_info: &'a state::PlayerInfo,
    library: &'a model::LibraryState,
) -> Option<Container<'a, Message>> {
    match player_info.current_playback {
        Some(ref outer_current_playback) => Some(controls_with_maybe_track_info(
            player_info,
            library,
            outer_current_playback,
        )),
        None => None,
    }
}

fn controls_with_maybe_track_info<'a>(
    player_info: &'a state::PlayerInfo,
    library: &'a model::LibraryState,
    outer_current_playback: &'a state::CurrentPlayback,
) -> Container<'a, Message> {
    let (duration_info, album_info) = match outer_current_playback {
        state::CurrentPlayback::Track(ref current_playback) => {
            let duration_info = bright_paragraph(format!(
                "{} / {}",
                common::format_duration(current_playback.current_second),
                common::format_duration(current_playback.track.metadata.duration.as_secs(),)
            ));

            let album_info = Row::new()
                .spacing(10)
                .push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Mini,
                        current_playback.track.metadata.album_artist_id.clone(),
                        current_playback.track.metadata.album_id.clone(),
                    ),
                    model::AlbumSize::Mini,
                ))
                .push(
                    Column::new()
                        .push({
                            let mut row = Row::new().push(
                                dark_text_like_button(bright(h2(current_playback
                                    .track
                                    .metadata
                                    .title
                                    .clone())))
                                .on_press(components::track_link(&current_playback.track.metadata)),
                            );
                            match current_playback.track.metadata.get_maybe_track_artist() {
                                Some(track_artist) => {
                                    row = row.push(
                                        dark_text_like_button(bright(h2(format!(
                                            " ({})",
                                            track_artist
                                        ))))
                                        .on_press(
                                            user_nav_message(NavMessage::ArtistFeaturedTrackView(
                                                musiqlibrary::ID::new(&track_artist),
                                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                                                model::SortOrder::Reversed,
                                            )),
                                        ),
                                    )
                                }
                                None => (),
                            }
                            row
                        })
                        .push(
                            Row::new()
                                .push(
                                    dark_text_like_button(h3(current_playback
                                        .track
                                        .metadata
                                        .album_artist
                                        .clone()))
                                    .on_press(user_nav_message(NavMessage::ArtistAlbumsView(
                                        current_playback.track.metadata.album_artist_id.clone(),
                                    ))),
                                )
                                .push(h3("-"))
                                .push(
                                    dark_text_like_button(h3(current_playback
                                        .track
                                        .metadata
                                        .album
                                        .clone()))
                                    .on_press(user_nav_message(NavMessage::ArtistAlbumView(
                                        current_playback.track.metadata.album_artist_id.clone(),
                                        current_playback.track.metadata.album_id.clone(),
                                        model::AlbumSize::Regular,
                                        None,
                                    ))),
                                ),
                        ),
                );
            (duration_info, album_info)
        }
        state::CurrentPlayback::PauseBreak => {
            let duration_info = bright_paragraph("");
            let album_info = Row::new().spacing(10).push(bright_paragraph("Paused"));

            (duration_info, album_info)
        }
    };

    Container::new(
        line_row()
            .push(
                line_row()
                    .spacing(10)
                    .push(
                        Row::new()
                            .push(
                                dark_button(bright_paragraph("<<")).on_press(
                                    Message::PlaybackRequest(shared::PlaybackRequest::Prev),
                                ),
                            )
                            .push(if player_info.playing {
                                dark_button(bright_paragraph("=")).on_press(
                                    Message::PlaybackRequest(shared::PlaybackRequest::Pause),
                                )
                            } else {
                                dark_button(bright_paragraph(">")).on_press(
                                    Message::PlaybackRequest(shared::PlaybackRequest::Play),
                                )
                            })
                            .push(
                                dark_button(bright_paragraph(">>")).on_press(
                                    Message::PlaybackRequest(shared::PlaybackRequest::Next),
                                ),
                            )
                            .push(dark_button(bright_paragraph("=|")).on_press(
                                Message::PlaybackRequest(shared::PlaybackRequest::InsertPause),
                            )),
                    )
                    .push(duration_info)
                    .width(Length::FillPortion(1)),
            )
            .push(album_info)
            .push(Space::with_width(Length::FillPortion(1)))
            .push(
                Row::new()
                    .push(
                        dark_button(bright_paragraph("--")).on_press(Message::Action(
                            message::Action::SetVolume(message::VolumeRequest::Set(0.0)),
                        )),
                    )
                    .push(dark_button(bright_paragraph("-")).on_press(Message::Action(
                        message::Action::SetVolume(message::VolumeRequest::Down(0.1)),
                    )))
                    .push(bright_paragraph(format!(
                        "{}",
                        (player_info.current_volume * 100.0).round()
                    )))
                    .push(dark_button(bright_paragraph("+")).on_press(Message::Action(
                        message::Action::SetVolume(message::VolumeRequest::Up(0.1)),
                    )))
                    .push(
                        dark_button(bright_paragraph("++")).on_press(Message::Action(
                            message::Action::SetVolume(message::VolumeRequest::Set(1.0)),
                        )),
                    ),
            ),
    )
    .padding(10)
}
