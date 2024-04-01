use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn home_page<'a>(
    app_images: &embedded::AppImages,
    state: &'a state::HomeState,
) -> Container<'a, Message> {
    match state {
        state::HomeState {} => {
            let album_list = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_albums_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Albums".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::MusicNavMessage::AlbumList(
                    0,
                    model::AlbumSortKey::ByParent,
                    model::AlbumSortKey::ByParent.default_order(),
                )
                .into_message(),
            );
            let artist_list = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_artists_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Artists".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::ArtistNavMessage::ArtistList(
                    0,
                    model::ArtistSortKey::ByName,
                    model::ArtistSortKey::ByName.default_order(),
                )
                .into_message(),
            );
            let track_list = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_tracks_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Tracks".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::MusicNavMessage::TrackList(
                    0,
                    model::TrackSortKey::ByName,
                    model::TrackSortKey::ByName.default_order(),
                )
                .into_message(),
            );
            let playlist = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_playlists_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Playlists".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(message::PlaylistNavMessage::PlaylistList("".to_string()).into_message());
            let search = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_search_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Search".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(user_nav_message(NavMessage::SearchPage(
                "".to_string(),
                model::SearchDomain::Music,
                false,
            )));
            let settings = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_settings_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Settings".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(user_nav_message(NavMessage::Config));

            let dvd = dark_button(Container::new(bottom_label(
                album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small).into(),
                bright_paragraph(common::abr_str(
                    "Movies".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::MovieNavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::MovieSortKey::ByTitle.default_order(),
                )
                .into_message(),
            );

            let page = Container::new(
                Scrollable::new(
                    Column::new()
                        .width(Length::Fill)
                        .push(Row::new().push(album_list).push(artist_list).push(search))
                        .push(Row::new().push(track_list).push(dvd).push(playlist))
                        .push(Row::new().push(settings)),
                )
                .height(Length::Fill),
            );

            page
        }
    }
}
