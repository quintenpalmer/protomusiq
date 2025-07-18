use std::cell::RefCell;

use super::update;

use musiqcore::datastore;

use crate::backend;
use crate::datastore::{
    jsonbacked::{self, playlists as userplaylists},
    sqlitebacked,
};

use crate::model;
use crate::util::logging;

use crate::datastore::loader;
use crate::datastore::staticassets::embedded;

use super::message;
use super::state;

pub fn initialize_everything() -> state::App {
    let mut logger = logging::Logger::new(logging::LogType::Timing, "initial load");

    logger.print_elapsed("starting loading (should be 0)");

    let config_state = musiqcore::model::app::AppConfigState::get_default();

    let load_mode = config_state.get_safe_load_mode();
    let loader = loader::Loader::from_load_mode(config_state.clone(), load_mode);

    logger.print_elapsed("loading config");

    let loaded_library =
        jsonbacked::tracklibrary::load_library_from_cache_and_scan(&config_state, &loader);
    logger.print_elapsed("loading library (with cache)");

    let video_library =
        jsonbacked::movielibrary::load_library_from_cache_and_scan(&config_state, &loader);
    logger.print_elapsed("loading video library");

    let image_mode = musiqcore::model::gl::ImageMode::ExactMatch;

    let game_library = musiqcore::model::gl::GameLibrary::new(&image_mode, &config_state.games);

    let loaded_images = jsonbacked::albumart::process_cache_and_get_album_art(
        &loaded_library,
        config_state.app_data_path.to_path_buf(),
    );
    logger.print_elapsed("processing album art (with cache)");

    let loaded_movie_images = jsonbacked::movieart::process_cache_and_get_movie_art(
        &video_library,
        config_state.app_data_path.to_path_buf(),
    );
    logger.print_elapsed("processing movie art (with cache)");

    let show_tracker = datastore::jsonbacked::showtracker::ShowTracker::new(
        &config_state.app_data_path.to_path_buf(),
    );

    let video_library_state = model::VideoLibraryState::new(video_library, loaded_movie_images);

    let game_library_state = musiqcore::model::gl::GameLibraryState::new(game_library);

    let show_library_state = musiqcore::model::shows::ShowLibraryState::new(
        config_state.show_path.clone(),
        show_tracker,
    );

    let read_only_tracker: Box<dyn datastore::traits::LiveReadOnlyTrackCountReporter> = match loader
    {
        loader::Loader::NoCache | loader::Loader::Json => {
            let json_track_reporter =
                musiqcore::datastore::jsonbacked::tracker::ReadOnlyTracker::new(
                    &config_state.app_data_path.to_path_buf(),
                    config_state.hostname.clone(),
                    &config_state.allowed_tracker_files,
                );
            logger.print_elapsed("read only tracker");

            Box::new(json_track_reporter)
        }
        loader::Loader::Sqlite(ref orig_conn) | loader::Loader::Latest(ref orig_conn) => {
            let mut conn = orig_conn.spawn_connection();

            // TODO: honor `config_state.allowed_tracker_files` and
            // `config_state.allowed_prehistory_files` in sqlite implementation
            match conn.needs_livehistory_seeded() {
                true => {
                    let livehistory_records =
                        musiqcore::datastore::jsonbacked::tracker::list_all_tracker_records(
                            &config_state.app_data_path.clone(),
                            &config_state.allowed_tracker_files,
                        );

                    conn.bootstrap_livehistory(&livehistory_records);
                }
                false => println!("already has livehistory seeded"),
            };

            let sqlite_reporter = sqlitebacked::SqliteLiveHistoryReporter::new(conn);
            logger.print_elapsed("got sqlite historical play count reporter");

            Box::new(sqlite_reporter)
        }
    };

    let historical_reporter: Box<dyn datastore::traits::HistoricalTrackCountReporter> = match loader
    {
        loader::Loader::NoCache | loader::Loader::Json => {
            let historical_play_count_reporter = jsonbacked::prehistory::Reporter::new(
                &config_state.app_data_path.to_path_buf(),
                &config_state.allowed_prehistory_files,
            );
            logger.print_elapsed("got JSON historical play count reporter");

            Box::new(historical_play_count_reporter)
        }
        loader::Loader::Sqlite(ref orig_conn) | loader::Loader::Latest(ref orig_conn) => {
            let mut conn = orig_conn.spawn_connection();

            match conn.needs_prehistory_seeded() {
                true => {
                    let prehistory_records = jsonbacked::prehistory::compute_historical_map(
                        &config_state.app_data_path.clone(),
                        &config_state.allowed_prehistory_files,
                    );

                    conn.bootstrap_prehistory(&prehistory_records);
                }
                false => println!("already has prehistory seeded"),
            };

            let sqlite_reporter = sqlitebacked::SqlitePreHistoryReporter::new(conn);
            logger.print_elapsed("got sqlite historical play count reporter");

            Box::new(sqlite_reporter)
        }
    };

    let grid_info = model::GridInfo::new(
        config_state.grid_layout_width,
        config_state.grid_layout_height,
        config_state.grid_layout_track_multiplier,
    );

    let musicbrainz_library = model::musicbrainzlib::Library::new(&loaded_library);

    let augmented_library =
        model::augmented_from_raw(loaded_library, read_only_tracker, historical_reporter);
    logger.print_elapsed("augmenting raw library");

    let extra_library = model::ExtraLibraryKeys::from_library(&augmented_library);

    let artist_sorts = model::ArtistSorts::new(&augmented_library);
    logger.print_elapsed("sorting artists");

    let album_sorts = model::AlbumSorts::new(&augmented_library);
    logger.print_elapsed("sorting albums");

    let track_sorts = model::TrackSorts::new(&augmented_library);
    logger.print_elapsed("sorting tracks");

    let playlists = userplaylists::PlaylistData::new(&config_state.app_data_path.to_path_buf());
    logger.print_elapsed("loading playlists");

    let (backend_client, backend_callback) = backend::create_backend_with_client_and_callback(
        config_state.clone(),
        loader.spawn_copy(),
        config_state.get_safe_sink_mode(),
    );
    logger.print_elapsed("creating backend");

    logger.print_elapsed("starting tracker");

    state::App::Loaded(state::AppState {
        page_state: state::PageState {
            current_page: state::Page::Home(state::HomeState {}),
            page_back_history: Vec::new(),
            page_current_history: message::NavMessage::Home,
            page_forward_history: Vec::new(),
        },
        cross_page_display_info: state::CrossPageDisplayInfo {
            fullscreen_display: false,
        },
        should_close: false,
        messages: Vec::new(),
        app_images: embedded::AppImages::new(&config_state.app_data_path),
        action_state: state::ActionState {
            group_buttons_shuffle: false,
        },
        video_library: video_library_state,
        show_library: show_library_state,
        game_library: game_library_state,
        config: config_state,
        player_info: state::PlayerInfo {
            playing: false,
            current_volume: 1.0,
            play_queue_info: state::PlayQueueInfo {
                play_queue_visible: true,
                play_history: Vec::new(),
                play_queue: Vec::new(),
                current_playback: None,
            },
            backend_message_sender: backend_client,
            backend_callback_recv: RefCell::new(Some(backend_callback)),
        },
        library: model::LibraryState::new(
            augmented_library,
            extra_library,
            musicbrainz_library,
            playlists,
            artist_sorts,
            album_sorts,
            track_sorts,
            grid_info,
            loaded_images,
        ),
    })
}

pub fn init_app() -> (state::App, iced::Command<message::Message>) {
    let mut app = state::App::Loading;
    let ret = update::update(
        &mut app,
        message::Message::Action(message::Action::LoadEverything),
    );
    (app, ret)
}
