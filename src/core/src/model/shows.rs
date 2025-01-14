use std::path;

use musiqlibrary::shows;

use crate::datastore::jsonbacked;

pub struct ShowLibraryState {
    inner: Option<ShowLibrary>,
}

impl ShowLibraryState {
    pub fn new<P: AsRef<path::Path>>(
        maybe_show_root_path: Option<P>,
        tracker: jsonbacked::showtracker::ShowTracker,
    ) -> Self {
        let maybe_inner = match maybe_show_root_path {
            Some(show_root_path) => Some(ShowLibrary::new(show_root_path, tracker)),
            None => None,
        };

        ShowLibraryState { inner: maybe_inner }
    }

    pub fn get_shows_if_exists(&self) -> &Option<ShowLibrary> {
        &self.inner
    }

    pub fn get_shows_if_exists_mut(&mut self) -> &mut Option<ShowLibrary> {
        &mut self.inner
    }
}

pub struct ShowLibrary {
    shows: shows::Shows,
    tracker: jsonbacked::showtracker::ShowTracker,
}

impl ShowLibrary {
    pub fn new<P: AsRef<path::Path>>(
        show_root_path: P,
        tracker: jsonbacked::showtracker::ShowTracker,
    ) -> Self {
        let show_vec = shows::find_shows_in_dir(show_root_path);
        let structured = shows::Shows::from_vec(&show_vec);

        ShowLibrary {
            shows: structured,
            tracker,
        }
    }

    pub fn get_structured_shows(&self) -> &shows::Shows {
        &self.shows
    }

    pub fn get_tracker_mut(&mut self) -> &mut jsonbacked::showtracker::ShowTracker {
        &mut self.tracker
    }
}
