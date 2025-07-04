use std::collections::BTreeMap;
use std::path;

use musiqlibrary::video;

use super::{common, sorts};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MovieRelPath(String);

impl MovieRelPath {
    pub fn from_metadata(movie: &video::MovieMetadata) -> Self {
        MovieRelPath(
            movie
                .relative_path
                .parent()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
        )
    }
}

pub struct VideoLibrary {
    pub movies: BTreeMap<MovieRelPath, video::MovieMetadata>,
    // TODO either use this or drop it
    #[allow(unused)]
    pub movie_id_to_path: BTreeMap<video::MovieID, MovieRelPath>,
}

impl VideoLibrary {
    pub fn new<P: AsRef<path::Path>>(movie_path: P) -> Self {
        let movies = video::find_movies_in_dir(movie_path);

        let mut movie_btree = BTreeMap::new();
        let mut movie_id_to_path = BTreeMap::new();

        for movie in movies.into_iter() {
            let rel_path = MovieRelPath::from_metadata(&movie);
            let id = movie.get_id();
            movie_btree.insert(rel_path.clone(), movie);
            movie_id_to_path.insert(id, rel_path);
        }

        VideoLibrary {
            movies: movie_btree,
            movie_id_to_path,
        }
    }

    pub fn from_metadata_list(metadata: Vec<video::MovieMetadata>) -> Self {
        let mut movie_btree = BTreeMap::new();
        let mut movie_id_to_path = BTreeMap::new();

        for movie in metadata.into_iter() {
            let rel_path = MovieRelPath::from_metadata(&movie);
            let id = movie.get_id();
            movie_btree.insert(rel_path.clone(), movie);
            movie_id_to_path.insert(id, rel_path);
        }

        VideoLibrary {
            movies: movie_btree,
            movie_id_to_path,
        }
    }
}

pub struct VideoLibraryState {
    pub movies: VideoLibrary,

    pub series: BTreeMap<String, Vec<(u32, video::MovieID)>>,

    pub art: MovieArt,

    pub movie_sorts: sorts::MovieSorts,
}

impl VideoLibraryState {
    pub fn new(movies: VideoLibrary, art: MovieArt) -> Self {
        let movie_sorts = sorts::MovieSorts::new(
            &movies
                .movies
                .values()
                .cloned()
                .collect::<Vec<video::MovieMetadata>>(),
        );

        let mut series_info = BTreeMap::new();

        for movie in movies.movies.values() {
            match movie.extra {
                Some(ref extra) => match extra.series {
                    Some(ref parsed_movie_series_info) => {
                        series_info
                            .entry(parsed_movie_series_info.name.clone())
                            .or_insert(Vec::new())
                            .push((parsed_movie_series_info.index, movie.get_id()));
                    }
                    None => (),
                },
                None => (),
            }
        }

        VideoLibraryState {
            movies,
            series: series_info,
            art,
            movie_sorts,
        }
    }

    pub fn get_movie(&self, title: &MovieRelPath) -> video::MovieMetadata {
        self.movies.movies.get(title).unwrap().clone()
    }

    pub fn get_movie_cover(&self, size: common::MovieSize, title: MovieRelPath) -> Option<Vec<u8>> {
        self.art.get_movie_cover(size, title)
    }

    pub fn search_movies(&self, query: String) -> common::MovieSearchResults {
        let mut titles = Vec::new();

        for movie in self.movies.movies.values() {
            if movie.title.to_lowercase().contains(&query.to_lowercase()) {
                titles.push(movie.clone());
            }
        }

        common::MovieSearchResults { titles }
    }

    pub fn get_series(&self) -> &BTreeMap<String, Vec<(u32, video::MovieID)>> {
        &self.series
    }
}

pub struct MovieArt {
    pub large_movie_covers: BTreeMap<MovieRelPath, Vec<u8>>,
    pub semilarge_movie_covers: BTreeMap<MovieRelPath, Vec<u8>>,
    pub regular_movie_covers: BTreeMap<MovieRelPath, Vec<u8>>,
    pub small_movie_covers: BTreeMap<MovieRelPath, Vec<u8>>,
    pub micro_movie_covers: BTreeMap<MovieRelPath, Vec<u8>>,
}

impl MovieArt {
    pub fn get_movie_cover(
        &self,
        album_size: common::MovieSize,
        movie_key: MovieRelPath,
    ) -> Option<Vec<u8>> {
        match album_size {
            common::MovieSize::Large => self.large_movie_covers.get(&movie_key).cloned(),
            common::MovieSize::SemiLarge => self.semilarge_movie_covers.get(&movie_key).cloned(),
            common::MovieSize::Small => self.small_movie_covers.get(&movie_key).cloned(),
            common::MovieSize::Regular => self.regular_movie_covers.get(&movie_key).cloned(),
            common::MovieSize::Micro => self.micro_movie_covers.get(&movie_key).cloned(),
        }
    }
}
