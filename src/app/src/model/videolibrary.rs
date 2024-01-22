use std::path;

use musiqlibrary::video;

pub struct VideoLibraryState {
    pub movies: Vec<video::MovieMetadata>,
}

impl VideoLibraryState {
    pub fn new<P: AsRef<path::Path>>(movie_path: P) -> Self {
        let movies = video::find_movies_in_dir(movie_path);

        VideoLibraryState { movies }
    }
}
