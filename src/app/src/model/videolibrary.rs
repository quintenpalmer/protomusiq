use std::collections::BTreeMap;
use std::path;

use musiqlibrary::video;

use super::sorts;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MovieTitle(String);

impl MovieTitle {
    pub fn from_metadata(movie: &video::MovieMetadata) -> Self {
        MovieTitle(movie.title.clone())
    }
}

pub struct VideoLibrary {
    pub movies: Vec<video::MovieMetadata>,
}

impl VideoLibrary {
    pub fn new<P: AsRef<path::Path>>(movie_path: P) -> Self {
        let movies = video::find_movies_in_dir(movie_path);

        VideoLibrary { movies }
    }
}

pub struct VideoLibraryState {
    pub movies: VideoLibrary,

    pub movie_sorts: sorts::MovieSorts,
}

impl VideoLibraryState {
    pub fn new(movies: VideoLibrary) -> Self {
        let movie_sorts = sorts::MovieSorts::new(&movies.movies);

        VideoLibraryState {
            movies,
            movie_sorts,
        }
    }
}

pub struct MovieArt {
    pub large_movie_covers: BTreeMap<MovieTitle, Vec<u8>>,
    pub regular_movie_covers: BTreeMap<MovieTitle, Vec<u8>>,
    pub micro_movie_covers: BTreeMap<MovieTitle, Vec<u8>>,
}
