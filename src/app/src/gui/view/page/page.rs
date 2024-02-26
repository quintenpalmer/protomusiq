use iced::widget::Container;

use crate::model;

use crate::gui::message::{self, Message};
use crate::state::{self, ActionState, Page, PlayQueueInfo, PlayerInfo};

use crate::datastore::staticassets::embedded;

use super::super::elements::*;

use super::pages;

pub fn render_page<'a>(
    current_page: &'a Page,
    page_current_history: &'a message::NavMessage,
    library: &'a model::LibraryState,
    movie_library: &'a model::VideoLibraryState,
    app_images: &embedded::AppImages,
    action_state: &'a ActionState,
    play_queue_info: &PlayQueueInfo,
    player_info: &'a PlayerInfo,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    let message_sourced_breadcrumbs = compute_breadcrumb(library, page_current_history);

    let (ret_breadcrumbs, ret_page) = match current_page {
        Page::Home(ref state) => pages::home::home_page(app_images, state),
        Page::Config(state::ConfigState {}) => pages::config::config_page(),
        Page::PlayQueue(state::PlayQueueState {}) => (
            Vec::new(),
            Container::new(bright_paragraph("The Play Queue")),
        ),
        Page::PlaylistView(ref state) => {
            pages::playlist::playlist_view(library, action_state, player_info, state)
        }
        Page::PlaylistList(ref state) => pages::playlists::playlist_list_view(library, state),
        Page::Search(ref state) => {
            pages::search::search_page(library, movie_library, app_images, state)
        }
        Page::TrackList(ref state) => pages::tracks::track_list(library, state),
        Page::AlbumList(ref state) => pages::albums::album_list(library, play_queue_info, state),
        Page::ArtistList(ref state) => pages::artist::artist_list(library, play_queue_info, state),
        Page::ArtistAlbumsView(ref state) => {
            pages::artistalbums::artist_album_list(library, play_queue_info, state)
        }
        Page::ArtistAlbumView(ref state) => {
            pages::artistalbum::artist_album_view_state(library, action_state, player_info, state)
        }
        Page::ArtistTrackView(ref state) => {
            pages::artisttracks::artist_track_view_state(library, player_info, state)
        }
        Page::ArtistFeaturedTrackView(ref state) => {
            pages::artistfeatured::artist_featured_track_view_state(library, player_info, state)
        }
        Page::MovieHome => pages::moviehome::movie_home(),
        Page::MovieList(ref state) => pages::movielist::movie_list(
            movie_library,
            state,
            play_queue_info,
            &library.grid_info,
            app_images,
        ),
        Page::MovieAttributes(ref state) => pages::movieattrs::movie_attributes(state),
        Page::MovieQuery(ref state) => {
            pages::moviequery::movie_query(movie_library, state, app_images)
        }
        Page::MovieView(ref state) => pages::movie::movie_page(movie_library, state, app_images),
    };

    match message_sourced_breadcrumbs {
        Some(breadcrumbs) => (breadcrumbs, ret_page),
        None => (ret_breadcrumbs, ret_page),
    }
}

fn compute_breadcrumb(
    library: &model::LibraryState,
    page_current_history: &message::NavMessage,
) -> Option<Vec<(String, Message)>> {
    match page_current_history {
        message::NavMessage::Home => Some(vec![]),
        message::NavMessage::Config => Some(vec![(
            "Settings".to_string(),
            message::NavMessage::Config.into_message(),
        )]),
        message::NavMessage::PlayQueueFocus => Some(vec![(
            "Play Queue".to_string(),
            message::NavMessage::PlayQueueFocus.into_message(),
        )]),
        message::NavMessage::SearchPage(query, domain, perform_search) => {
            let mut ret = vec![(
                "Search".to_string(),
                message::NavMessage::SearchPage("".to_string(), model::SearchDomain::Music, false)
                    .into_message(),
            )];
            if *perform_search {
                ret.push((
                    format!("\"{}\"", query),
                    message::NavMessage::SearchPage(query.clone(), domain.clone(), *perform_search)
                        .into_message(),
                ));
            }
            Some(ret)
        }
        message::NavMessage::TrackList(_, _, _) => Some(vec![(
            "Tracks".to_string(),
            message::NavMessage::TrackList(
                0,
                model::TrackSortKey::ByName,
                model::TrackSortKey::ByName.default_order(),
            )
            .into_message(),
        )]),
        message::NavMessage::AlbumList(_, _, _) => Some(vec![(
            "Albums".to_string(),
            message::NavMessage::AlbumList(
                0,
                model::AlbumSortKey::ByParent,
                model::AlbumSortKey::ByParent.default_order(),
            )
            .into_message(),
        )]),
        message::NavMessage::Movie(movie_message) => Some(movie_breadcrumbs(movie_message)),
        message::NavMessage::Playlist(playlist_message) => {
            Some(playlist_breadcrumbs(library, playlist_message))
        }
        // TODO remove this catch all and force new page messages to be handled here
        _ => None,
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
                playlist_name,
                message::PlaylistNavMessage::PlaylistView(*playlist_id).into_message(),
            ));
        }
    };
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
                    a.display_text(),
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
        message::MovieNavMessage::MovieView(movie, _) => ret.push((
            movie.title.clone(),
            message::MovieNavMessage::MovieView(movie.clone(), None).into_message(),
        )),
    }

    ret
}
