use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};
use crate::gui::view::components;
use crate::state;

use super::super::super::elements::*;

pub fn artist_album_featured_in_playlist_state<'a>(
    library: &'a model::LibraryState,
    state: &'a state::ArtistAlbumFeaturedInPlaylistState,
) -> Container<'a, Message> {
    match state {
        state::ArtistAlbumFeaturedInPlaylistState {
            artist_id,
            album_id,

            playlist_ids,
        } => {
            let album_view_button_row = line_row()
                .push(
                    dark_button(dark(h2("Track List"))).on_press(
                        message::ArtistAlbumView::ArtistAlbumTrackView(
                            model::AlbumSize::Regular,
                            None,
                            None,
                        )
                        .into_message(*artist_id, *album_id),
                    ),
                )
                .push(dark_button(h2("In Playlists")).on_press(
                    message::ArtistAlbumView::InPlaylist.into_message(*artist_id, *album_id),
                ));

            let mut playlists = Vec::new();

            for playlist_id in playlist_ids.iter() {
                let playlist = library.user_playlists.get_playlist(*playlist_id).unwrap();
                playlists.push(playlist);
            }

            let mut stripe_marker = false;

            let mut playlist_list = Column::new().padding(15);

            for playlist in playlists.iter() {
                stripe_marker = !stripe_marker;

                let row = line_row()
                    .padding(8)
                    .align_items(iced::Alignment::Center)
                    .push(
                        dark_button(components::compute_playlist_thumbnail(
                            library,
                            &playlist.tracks,
                            components::PlaylistIconSize::Small,
                        ))
                        .on_press(
                            message::PlaylistNavMessage::PlaylistView(playlist.id).into_message(),
                        ),
                    )
                    .push(
                        dark_button(Column::new().push(h2(playlist.name.clone())).push(
                            bright_paragraph(format!("{} tracks", playlist.tracks.len())),
                        ))
                        .on_press(
                            message::PlaylistNavMessage::PlaylistView(playlist.id).into_message(),
                        ),
                    )
                    .width(Length::Fill);

                playlist_list = playlist_list.push(row);
            }

            let scrollable = Scrollable::new(playlist_list).height(Length::Fill);

            let body = Container::new(Column::new().push(album_view_button_row).push(scrollable));

            body
        }
    }
}
