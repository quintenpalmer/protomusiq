use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};
use crate::gui::view::components;
use crate::state;

use super::super::super::elements::*;

pub fn artist_in_playlist_view_state<'a>(
    library: &'a model::LibraryState,
    state: &'a state::ArtistFeaturedInPlaylistState,
) -> Container<'a, Message> {
    match state {
        state::ArtistFeaturedInPlaylistState {
            artist_id,

            playlist_ids,
        } => {
            let artist = library.get_artist_info(*artist_id);

            let artist_view_button_row =
                line_row()
                    .push(dark_button(dark(h2("Albums"))).on_press(
                        message::ArtistViewType::ArtistAlbumsView.into_message(*artist_id),
                    ))
                    .push(
                        dark_button(dark(h2("Tracks"))).on_press(
                            message::ArtistViewType::ArtistTrackView(
                                model::ArtistTrackSortKey::ByTotalPlayCount,
                                model::ArtistTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(dark(h2("Featured"))).on_press(
                            message::ArtistViewType::ArtistFeaturedTrackView(
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(h2("In Playlists"))
                            .on_press(message::ArtistViewType::InPlaylist.into_message(*artist_id)),
                    );

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

            let body = Container::new(
                Column::new()
                    .push(h1(artist.artist_name.clone()))
                    .push(artist_view_button_row)
                    .push(scrollable),
            );

            body
        }
    }
}
