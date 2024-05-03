use iced::widget::Container;

use crate::model;

use crate::gui::compute;
use crate::gui::message::{self, Message};
use crate::state::{self, ActionState, Page, PlayerInfo};

use crate::datastore::staticassets::embedded;

use super::super::elements::*;

use super::pages;

pub fn render_page<'a>(
    current_page: &'a Page,
    page_current_history: &'a message::NavMessage,
    library: &'a model::LibraryState,
    movie_library: &'a model::VideoLibraryState,
    game_library: &'a model::GameLibraryState,
    app_images: &embedded::AppImages,
    action_state: &'a ActionState,
    play_queue_visible: bool,
    player_info: &'a PlayerInfo,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    let message_sourced_breadcrumbs = compute::compute_breadcrumb(library, page_current_history);

    let ret_page = match current_page {
        Page::Home(ref state) => pages::home::home_page(app_images, state),
        Page::Config(state::ConfigState {}) => pages::config::config_page(),
        Page::PlayQueue(state::PlayQueueState {}) => {
            Container::new(bright_paragraph("The Play Queue"))
        }
        Page::PlaylistView(ref state) => {
            pages::playlist::playlist_view(library, action_state, player_info, state)
        }
        Page::PlaylistList(ref state) => pages::playlists::playlist_list_view(library, state),
        Page::Search(ref state) => {
            pages::search::search_page(library, movie_library, app_images, state)
        }
        Page::GenreHome => pages::musicgenrehome::genre_home(),
        Page::TrackList(ref state) => pages::tracklist::track_list(library, state),
        Page::AlbumList(ref state) => {
            pages::albumlist::album_list(library, play_queue_visible, state)
        }
        Page::ArtistList(ref state) => {
            pages::artistlist::artist_list(library, play_queue_visible, state)
        }
        Page::ArtistAlbumsView(ref state) => {
            pages::artistalbums::artist_album_list(library, play_queue_visible, state)
        }
        Page::ArtistAlbumView(ref state) => {
            pages::artistalbum::artist_album_view_state(library, action_state, player_info, state)
        }
        Page::ArtistAlbumFeaturedInPlaylist(ref state) => {
            pages::artistalbuminplaylist::artist_album_featured_in_playlist_state(library, state)
        }
        Page::ArtistTrackView(ref state) => {
            pages::artisttracks::artist_track_view_state(library, player_info, state)
        }
        Page::ArtistInfoView(ref state) => {
            pages::artistinfo::artist_info_view_state(library, state)
        }
        Page::ArtistFeaturedTrackView(ref state) => {
            pages::artistfeatured::artist_featured_track_view_state(library, player_info, state)
        }
        Page::ArtistFeaturedInPlaylist(ref state) => {
            pages::artistinplaylists::artist_in_playlist_view_state(library, state)
        }
        Page::MovieHome => pages::moviehome::movie_home(app_images),
        Page::MovieList(ref state) => pages::movielist::movie_list(
            movie_library,
            state,
            play_queue_visible,
            &library.grid_info,
            app_images,
        ),
        Page::MovieAttributes(ref state) => pages::movieattrs::movie_attributes(state),
        Page::MovieQuery(ref state) => {
            pages::moviequery::movie_query(movie_library, state, app_images)
        }
        Page::MovieView(ref state) => {
            pages::movie::movie_page(movie_library, state, &library.grid_info, app_images)
        }
        Page::GameHome => pages::gamehome::game_home(app_images),
        Page::GBAList => pages::gamegba::gba_list(game_library),
        Page::SNESList => pages::gamesnes::snes_list(game_library),
        Page::N64List => pages::gamen64::n64_list(game_library),
        Page::NDSList => pages::gamends::nds_list(game_library),
        Page::GameCubeList => pages::gamengc::ngc_list(game_library),
        Page::WiiList => pages::gamewii::wii_list(game_library),
    };

    (message_sourced_breadcrumbs, ret_page)
}
