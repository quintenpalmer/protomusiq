use std::path;

use musiqlibrary::shows;

pub struct ShowLibraryState {
    inner: Option<ShowLibrary>,
}

impl ShowLibraryState {
    pub fn new<P: AsRef<path::Path>>(maybe_show_root_path: Option<P>) -> Self {
        let maybe_inner = match maybe_show_root_path {
            Some(show_root_path) => Some(ShowLibrary::new(show_root_path)),
            None => None,
        };

        ShowLibraryState { inner: maybe_inner }
    }

    pub fn get_shows_if_exists(&self) -> &Option<ShowLibrary> {
        &self.inner
    }
}

pub struct ShowLibrary {
    shows: shows::Shows,
}

impl ShowLibrary {
    pub fn new<P: AsRef<path::Path>>(show_root_path: P) -> Self {
        let show_vec = shows::find_shows_in_dir(show_root_path);
        let structured = shows::Shows::from_vec(&show_vec);

        ShowLibrary { shows: structured }
    }

    pub fn get_structured_shows(&self) -> &shows::Shows {
        &self.shows
    }
}
