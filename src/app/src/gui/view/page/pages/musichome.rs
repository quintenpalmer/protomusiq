use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;

use crate::gui::message;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn music_home<'a>(app_images: &embedded::AppImages) -> Container<'a, message::Message> {
    let album_link = dark_button(Container::new(bottom_label(
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
            model::AlbumSortKey::preferred_home(),
            model::AlbumSortKey::preferred_home().default_order(),
        )
        .into_message(),
    );

    let artist_link = dark_button(Container::new(bottom_label(
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
            model::ArtistSortKey::preferred_home(),
            model::ArtistSortKey::preferred_home().default_order(),
        )
        .into_message(),
    );

    let track_link = dark_button(Container::new(bottom_label(
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

    let playlist_link = dark_button(Container::new(bottom_label(
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

    let body_column = Column::new()
        .push(h1("Music"))
        .push(line_row().push(album_link).push(artist_link))
        .push(line_row().push(track_link).push(playlist_link));

    let body = Container::new(Scrollable::new(body_column).height(Length::Fill));

    body
}
