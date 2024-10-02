use super::super::commands::FlexibleCmd;

pub struct CSVHistoryGenerator {}

impl FlexibleCmd for CSVHistoryGenerator {
    fn flex_operate(&self, args: Vec<String>) {
        if args.len() != 0 {
            panic!("csv-ifier of tracker history doesn't need any commands");
        }

        let config_state = musiqcore::model::app::AppConfigState::get_default();

        let library_path = config_state.library_path;

        let library = musiqlibrary::RawLibrary::new(library_path).unwrap();

        let json_track_reporter = musiqcore::datastore::jsonbacked::tracker::ReadOnlyTracker::new(
            &config_state.app_data_path.to_path_buf(),
            config_state.hostname.clone(),
            &config_state.allowed_tracker_files,
        );

        let mut all_listens = Vec::new();

        for track in library.get_all_tracks().into_iter() {
            match json_track_reporter.get_track_history(&track.to_unique_id()) {
                Some(listens) => {
                    for listen in listens.iter() {
                        all_listens.push((track.clone(), listen.clone()));
                    }
                }
                None => (),
            }
        }

        for (track, listen) in all_listens.into_iter() {
            println!(
                "{},{},{},{}",
                quote_escape(track.title),
                quote_escape(track.album),
                quote_escape(track.album_artist),
                listen.format("%Y/%m/%d %H:%M:%S")
            )
        }
    }
}

fn quote_escape(s: String) -> String {
    // We juse quote every value, but if there are quotes inside of the value,
    // then we should escape the quotes with another quotes character.
    // See RFC 4180:
    // https://www.ietf.org/rfc/rfc4180.txt
    let inner = str::replace(s.as_str(), "\"", "\"\"");
    format!("\"{}\"", inner)
}
