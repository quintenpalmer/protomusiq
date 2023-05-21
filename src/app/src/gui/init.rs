use std::cell::RefCell;

use iced;

use crate::datastore::{
    self,
    jsonbacked::{self, playlists as userplaylists},
    sqlitebacked,
};
use crate::services::{mpris, sink, tracker};

use crate::util::{config, logging};
use crate::model;

use crate::datastore::loader;
use crate::datastore::staticassets::embedded;

use super::message;
use super::state;

pub fn initialize_everything() -> state::App {
    let mut logger = logging::Logger::new(logging::LogType::Timing, "initial load");

    logger.print_elapsed("starting loading (should be 0)");

    let config_state = config::get_default_config();

    let load_mode = config_state.get_safe_load_mode();
    let loader = loader::Loader::from_load_mode(config_state.clone(), load_mode);

    logger.print_elapsed("loading config");

    let loaded_library =
        jsonbacked::tracklibrary::load_library_from_cache_and_scan(&config_state, &loader);

    logger.print_elapsed("loading library (with cache)");

    let loaded_images = jsonbacked::albumart::process_cache_and_get_album_art(
        &loaded_library,
        config_state.app_data_path.to_path_buf(),
    );
    logger.print_elapsed("processing album art (with cache)");

    let read_only_tracker: Box<dyn datastore::traits::LiveReadOnlyTrackCountReporter> = match loader
    {
        loader::Loader::NoCache | loader::Loader::JSON => {
            let json_track_reporter = jsonbacked::tracker::ReadOnlyTracker::new(
                &config_state.app_data_path.to_path_buf(),
                config_state.hostname.clone(),
            );
            logger.print_elapsed("read only tracker");

            Box::new(json_track_reporter)
        }
        loader::Loader::Sqlite(ref orig_conn) | loader::Loader::Latest(ref orig_conn) => {
            let mut conn = orig_conn.spawn_connection();

            match conn.needs_livehistory_seeded() {
                true => {
                    let livehistory_records = jsonbacked::tracker::list_all_tracker_records(
                        &config_state.app_data_path.clone(),
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
        loader::Loader::NoCache | loader::Loader::JSON => {
            let historical_play_count_reporter =
                jsonbacked::prehistory::Reporter::new(&config_state.app_data_path.to_path_buf());
            logger.print_elapsed("got JSON historical play count reporter");

            Box::new(historical_play_count_reporter)
        }
        loader::Loader::Sqlite(ref orig_conn) | loader::Loader::Latest(ref orig_conn) => {
            let mut conn = orig_conn.spawn_connection();

            match conn.needs_prehistory_seeded() {
                true => {
                    let prehistory_records = jsonbacked::prehistory::compute_historical_map(
                        &config_state.app_data_path.clone(),
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

    let augmented_library =
        model::augmented_from_raw(loaded_library, read_only_tracker, historical_reporter);
    logger.print_elapsed("augmenting raw library");

    let artist_sorts = model::ArtistSorts::new(&augmented_library);
    logger.print_elapsed("sorting artists");

    let album_sorts = model::AlbumSorts::new(&augmented_library);
    logger.print_elapsed("sorting albums");

    let track_sorts = model::TrackSorts::new(&augmented_library);
    logger.print_elapsed("sorting tracks");

    let playlists = userplaylists::PlaylistData::new(&config_state.app_data_path.to_path_buf());
    logger.print_elapsed("loading playlists");

    let (sink_client, sink_callback) = sink::create_backend_with_client_and_callback();
    logger.print_elapsed("creating sink");

    let (mpris_client, mpris_callback) = mpris::create_backend_with_client_and_callback();
    logger.print_elapsed("registering mpris");

    let tracker_client =
        tracker::create_backend_with_client(config_state.clone(), loader.spawn_copy());

    logger.print_elapsed("starting tracker");

    state::App::Loaded(state::Loaded {
        gui: state::AppGuiState {},
        rest: state::AppState {
            page_back_history: Vec::new(),
            page_current_history: message::NavMessage::Home,
            page_forward_history: Vec::new(),
            current_page: state::Page::Home(state::HomeState {}),
            should_close: false,
            error_messages: Vec::new(),
            app_images: embedded::AppImages::new(&config_state.app_data_path),
            action_state: state::ActionState{
                group_buttons_shuffle: false,
            },
            play_queue_info: state::PlayQueueInfo {
                gui: state::PlayQueueGuiState {},
                rest: state::PlayQueueInfoState {
                    play_queue_visible: true,
                    play_history: Vec::new(),
                    play_queue: Vec::new(),
                    current_playback: None,
                },
            },
            config: state::Config {
                gui: state::AppConfigGuiState {},
                rest: config_state,
            },
            player_info: state::PlayerInfo {
                gui: state::PlayerInfoGuiState {
                },
                rest: state::PlayerInfoState {
                    playing: false,
                    current_volume: 1.0,
                    current_playback: None,
                    sink_message_sender: sink_client,
                    sink_callback_recv: RefCell::new(Some(sink_callback)),
                    mpris_message_sender: mpris_client,
                    mpris_callback_recv: RefCell::new(Some(mpris_callback)),
                    tracker_message_sender: tracker_client,
                },
            },
            library: model::LibraryState {
                raw_library: augmented_library,

                user_playlists: playlists,

                artist_sorts: artist_sorts,
                album_sorts: album_sorts,
                track_sorts: track_sorts,

                grid_info: grid_info,

                album_art: loaded_images,
            },
        },
    })
}

pub fn init_app() -> (state::App, iced::Command<message::Message>) {
    (
        state::App::Loading,
        message::message_command(message::Message::Action(message::Action::LoadEverything)),
    )
}
