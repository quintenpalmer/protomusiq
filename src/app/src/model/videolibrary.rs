use std::path;

use musiqlibrary::video;

use super::sorts;

pub struct VideoLibraryState {
    pub movies: Vec<video::MovieMetadata>,

    pub movie_sorts: sorts::MovieSorts,
}

impl VideoLibraryState {
    pub fn new<P: AsRef<path::Path>>(movie_path: P) -> Self {
        let movies = video::find_movies_in_dir(movie_path);
        let movie_sorts = sorts::MovieSorts::new(&movies);

        VideoLibraryState {
            movies,
            movie_sorts,
        }
    }
}
