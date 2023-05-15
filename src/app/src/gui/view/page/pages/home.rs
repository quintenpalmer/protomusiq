use iced::{self, button, Column, Container, Row, Scrollable};

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
            search_button,
            settings_button,
            scroll,
        } => {
            let album_list = dark_button(
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
                                    ));
            let artist_list = dark_button(
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
                                    ));
            let track_list = dark_button(
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
                                    ));
            let playlist = dark_button(
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
                                    ));
            let search = dark_button(
                                        search_button,
                                        Container::new(bottom_label(
                                            album_image(
                                                app_images.get_search_image().clone(),
                                                model::AlbumSize::Small,
                                            )
                                            .into(),
                                            bright_paragraph(common::abr_str(
                                                "Search".to_string(),
                                                consts::ICON_STR_LENGTH,
                                            )),
                                        )),
                                    )
                                    .on_press(user_nav_message(
                                        NavMessage::SearchPage(
                                            "".to_string(),
                                            false,
                                        )
                                    ));
            let settings = dark_button(
                                        settings_button,
                                        Container::new(bottom_label(
                                            album_image(
                                                app_images.get_settings_image().clone(),
                                                model::AlbumSize::Small,
                                            )
                                            .into(),
                                            bright_paragraph(common::abr_str(
                                                "Settings".to_string(),
                                                consts::ICON_STR_LENGTH,
                                            )),
                                        )),
                                    )
                                    .on_press(user_nav_message(
                                        NavMessage::Config,
                                    ));

            let page = Container::new(
                Scrollable::new(
                    scroll,
                ).push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(album_list)
                                .push(artist_list)
                                .push(search),
                        )
                        .push(
                            Row::new()
                                .push(track_list)
                                .push(playlist)
                                .push(settings),
                        ),
                    ),
                );

            (Vec::new(), page)
        },
    }
}
