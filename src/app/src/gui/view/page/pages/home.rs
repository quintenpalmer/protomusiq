use iced::{self, button, Column, Container, Row};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::consts;

use super::super::super::common;
use super::super::super::elements::*;

pub fn home_page<'a>(
    app_images: &embedded::AppImages,
    state: &'a mut state::HomeState,
) -> (
    Vec<(&'a mut button::State, String, Message)>,
    Container<'a, Message>,
) {
    match state {
        state::HomeState {
            artist_list_button,
            album_list_button,
            track_list_button,
            playlist_list_button,
        } => (
            Vec::new(),
            Container::new(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                dark_button(
                                    album_list_button,
                                    Container::new(bottom_label(
                                        album_image(
                                            app_images.get_albums_image().clone(),
                                            model::AlbumSize::Small,
                                        )
                                        .into(),
                                        bright_paragraph(common::abr_str(
                                            "Albums".to_string(),
                                            consts::ICON_STR_LENGTH,
                                        )),
                                    )),
                                )
                                .on_press(user_nav_message(
                                    NavMessage::AlbumList(
                                        0,
                                        model::AlbumSortKey::ByParent,
                                        model::SortOrder::Regular,
                                    ),
                                )),
                            )
                            .push(
                                dark_button(
                                    artist_list_button,
                                    Container::new(bottom_label(
                                        album_image(
                                            app_images.get_artists_image().clone(),
                                            model::AlbumSize::Small,
                                        )
                                        .into(),
                                        bright_paragraph(common::abr_str(
                                            "Artists".to_string(),
                                            consts::ICON_STR_LENGTH,
                                        )),
                                    )),
                                )
                                .on_press(user_nav_message(
                                    NavMessage::ArtistList(
                                        0,
                                        model::ArtistSortKey::ByName,
                                        model::SortOrder::Regular,
                                    ),
                                )),
                            ),
                    )
                    .push(
                        Row::new()
                            .push(
                                dark_button(
                                    track_list_button,
                                    Container::new(bottom_label(
                                        album_image(
                                            app_images.get_tracks_image().clone(),
                                            model::AlbumSize::Small,
                                        )
                                        .into(),
                                        bright_paragraph(common::abr_str(
                                            "Tracks".to_string(),
                                            consts::ICON_STR_LENGTH,
                                        )),
                                    )),
                                )
                                .on_press(user_nav_message(
                                    NavMessage::TrackList(
                                        0,
                                        model::TrackSortKey::ByName,
                                        model::SortOrder::Regular,
                                    ),
                                )),
                            )
                            .push(
                                dark_button(
                                    playlist_list_button,
                                    Container::new(bottom_label(
                                        album_image(
                                            app_images.get_playlists_image().clone(),
                                            model::AlbumSize::Small,
                                        )
                                        .into(),
                                        bright_paragraph(common::abr_str(
                                            "Playlists".to_string(),
                                            consts::ICON_STR_LENGTH,
                                        )),
                                    )),
                                )
                                .on_press(user_nav_message(
                                    NavMessage::PlaylistList("".to_string()),
                                )),
                            ),
                    ),
            ),
        ),
    }
}
