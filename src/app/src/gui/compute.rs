use crate::model;

use crate::gui::message::{self, Message};
use crate::gui::view::common;
use crate::gui::view::consts;

pub fn compute_breadcrumb(
    library: &model::LibraryState,
    page_current_history: &message::NavMessage,
) -> Vec<(String, Message)> {
    match page_current_history {
        message::NavMessage::Home => vec![],
        message::NavMessage::Config => vec![(
            "Settings".to_string(),
            message::NavMessage::Config.into_message(),
        )],
        message::NavMessage::PlayQueueFocus => vec![(
            "Play Queue".to_string(),
            message::NavMessage::PlayQueueFocus.into_message(),
        )],
        message::NavMessage::SearchPage(query, domain, perform_search) => {
            let mut ret = vec![(
                "Search".to_string(),
                message::NavMessage::SearchPage("".to_string(), model::SearchDomain::Music, false)
                    .into_message(),
            )];
            if *perform_search {
                ret.push((
                    common::abr_str(format!("\"{}\"", query), consts::NAV_STR_LENGTH),
                    message::NavMessage::SearchPage(query.clone(), domain.clone(), *perform_search)
                        .into_message(),
                ));
            }
            ret
        }
        message::NavMessage::TrackList(_, _, _) => vec![(
            "Tracks".to_string(),
            message::NavMessage::TrackList(
                0,
                model::TrackSortKey::ByName,
                model::TrackSortKey::ByName.default_order(),
            )
            .into_message(),
        )],
        message::NavMessage::AlbumList(_, _, _) => vec![(
            "Albums".to_string(),
            message::NavMessage::AlbumList(
                0,
                model::AlbumSortKey::ByParent,
                model::AlbumSortKey::ByParent.default_order(),
            )
            .into_message(),
        )],
        message::NavMessage::Artist(artist_message) => artist_breadcrumbs(library, artist_message),
        message::NavMessage::Movie(movie_message) => movie_breadcrumbs(movie_message),
        message::NavMessage::Playlist(playlist_message) => {
            playlist_breadcrumbs(library, playlist_message)
        }
    }
}

fn playlist_breadcrumbs(
    library: &model::LibraryState,
    message: &message::PlaylistNavMessage,
) -> Vec<(String, Message)> {
    let mut ret = vec![(
        "Playlists".to_string(),
        message::PlaylistNavMessage::PlaylistList("".to_string()).into_message(),
    )];
    match message {
        message::PlaylistNavMessage::PlaylistList(_new_name_part) => (),
        message::PlaylistNavMessage::PlaylistView(playlist_id) => {
            let playlist_name = library
                .user_playlists
                .get_playlist(*playlist_id)
                .unwrap()
                .name
                .clone();
            ret.push((
                common::abr_str(playlist_name, consts::NAV_STR_LENGTH),
                message::PlaylistNavMessage::PlaylistView(*playlist_id).into_message(),
            ));
        }
    };
    ret
}

fn artist_breadcrumbs(
    library: &model::LibraryState,
    message: &message::ArtistNavMessage,
) -> Vec<(String, Message)> {
    let mut ret = vec![(
        "Artists".to_string(),
        message::ArtistNavMessage::ArtistList(
            0,
            model::ArtistSortKey::ByName,
            model::ArtistSortKey::ByName.default_order(),
        )
        .into_message(),
    )];

    match message {
        message::ArtistNavMessage::ArtistList(_, _, _) => (),
        message::ArtistNavMessage::ArtistView(artist_id, _type) => {
            let artist_name = library.get_artist_info(*artist_id).artist_name;

            ret.push((
                common::abr_str(artist_name.clone(), consts::NAV_STR_LENGTH),
                message::ArtistNavMessage::ArtistView(
                    *artist_id,
                    message::ArtistViewType::ArtistAlbumsView,
                )
                .into_message(),
            ));
        }
        message::ArtistNavMessage::ArtistAlbumView(
            artist_id,
            album_id,
            _size,
            _selected_track,
            _sort_order_placement,
        ) => {
            let artist_album_info = library.get_artist_album_info(*artist_id, *album_id);
            let artist_name = artist_album_info.artist.artist_name.clone();
            let album_name = artist_album_info.album.album_name.clone();

            ret.push((
                common::abr_str(artist_name, consts::NAV_STR_LENGTH),
                message::ArtistNavMessage::ArtistView(
                    *artist_id,
                    message::ArtistViewType::ArtistAlbumsView,
                )
                .into_message(),
            ));
            ret.push((
                common::abr_str(album_name, consts::NAV_STR_LENGTH),
                message::ArtistNavMessage::ArtistAlbumView(
                    *artist_id,
                    *album_id,
                    model::AlbumSize::Regular,
                    None,
                    None,
                )
                .into_message(),
            ));
        }
    }

    ret
}

fn movie_breadcrumbs(message: &message::MovieNavMessage) -> Vec<(String, Message)> {
    let mut ret = vec![(
        "Movie".to_string(),
        message::MovieNavMessage::MovieHome.into_message(),
    )];

    match message {
        message::MovieNavMessage::MovieHome => (),
        message::MovieNavMessage::MovieList(_, _, _) => ret.push((
            "Movies".to_string(),
            message::MovieNavMessage::MovieList(
                0,
                model::MovieSortKey::ByTitle,
                model::MovieSortKey::ByTitle.default_order(),
            )
            .into_message(),
        )),
        message::MovieNavMessage::MovieAttributes(maybe_attr) => {
            ret.push((
                "Attributes".to_string(),
                message::MovieNavMessage::MovieAttributes(None).into_message(),
            ));
            match maybe_attr {
                Some(a) => ret.push((
                    common::abr_str(a.display_text(), consts::NAV_STR_LENGTH),
                    message::MovieNavMessage::MovieAttributes(Some(a.clone())).into_message(),
                )),
                None => (),
            }
        }
        message::MovieNavMessage::MovieQuery(maybe_query) => {
            ret.push((
                "Query".to_string(),
                message::MovieNavMessage::MovieQuery(None).into_message(),
            ));
            match maybe_query {
                Some(q) => ret.push((
                    "...".to_string(),
                    message::MovieNavMessage::MovieQuery(Some(q.clone())).into_message(),
                )),
                None => (),
            }
        }
        message::MovieNavMessage::MovieView(movie, _) => {
            ret.push((
                "Movies".to_string(),
                message::MovieNavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::MovieSortKey::ByTitle.default_order(),
                )
                .into_message(),
            ));
            ret.push((
                common::abr_str(movie.title.clone(), consts::NAV_STR_LENGTH),
                message::MovieNavMessage::MovieView(movie.clone(), None).into_message(),
            ))
        }
    }

    ret
}
