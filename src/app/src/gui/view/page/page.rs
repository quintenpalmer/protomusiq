use iced::widget::Container;

use crate::model;

use crate::gui::message::Message;
use crate::state::{self, ActionState, Page, PlayQueueInfo, PlayerInfo};

use crate::datastore::staticassets::embedded;

use super::super::elements::*;

use super::pages;

pub fn render_page<'a>(
    current_page: &'a Page,
    library: &'a model::LibraryState,
    movie_library: &'a model::VideoLibraryState,
    app_images: &embedded::AppImages,
    action_state: &'a ActionState,
    play_queue_info: &PlayQueueInfo,
    player_info: &'a PlayerInfo,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match current_page {
        Page::Home(ref state) => pages::home::home_page(&app_images, state),
        Page::Config(state::ConfigState {}) => pages::config::config_page(),
        Page::PlayQueue(state::PlayQueueState {}) => (
            Vec::new(),
            Container::new(bright_paragraph("The Play Queue")),
        ),
        Page::PlaylistView(ref state) => {
            pages::playlist::playlist_view(&library, &action_state, &player_info, state)
        }
        Page::PlaylistList(ref state) => pages::playlists::playlist_list_view(&library, state),
        Page::Search(ref state) => {
            pages::search::search_page(&library, &movie_library, &app_images, state)
        }
        Page::TrackList(ref state) => pages::tracks::track_list(&library, state),
        Page::AlbumList(ref state) => pages::albums::album_list(&library, &play_queue_info, state),
        Page::ArtistList(ref state) => {
            pages::artist::artist_list(&library, &play_queue_info, state)
        }
        Page::ArtistAlbumsView(ref state) => {
            pages::artistalbums::artist_album_list(&library, &play_queue_info, state)
        }
        Page::ArtistAlbumView(ref state) => pages::artistalbum::artist_album_view_state(
            &library,
            &action_state,
            &player_info,
            state,
        ),
        Page::ArtistTrackView(ref state) => {
            pages::artisttracks::artist_track_view_state(&library, &player_info, state)
        }
        Page::ArtistFeaturedTrackView(ref state) => {
            pages::artistfeatured::artist_featured_track_view_state(&library, &player_info, state)
        }
        Page::MovieHome => pages::moviehome::movie_home(),
        Page::MovieList(ref state) => pages::movielist::movie_list(
            &movie_library,
            state,
            &play_queue_info,
            &library.grid_info,
            &app_images,
        ),
        Page::MovieAttributes(ref state) => pages::movieattrs::movie_attributes(state),
        Page::MovieQuery(ref state) => {
            pages::moviequery::movie_query(&movie_library, state, &app_images)
        }
        Page::MovieView(ref state) => pages::movie::movie_page(&movie_library, state, &app_images),
    }
}
