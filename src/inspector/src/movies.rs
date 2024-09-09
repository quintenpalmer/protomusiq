use std::path::PathBuf;

use musiqlibrary::video;

use super::commands::AppCmd;

pub struct MovieTreeViewer {}

impl AppCmd for MovieTreeViewer {
    fn operate(&self, path: PathBuf) {
        let mut movies = video::find_movies_in_dir(path.clone());
        movies.sort_by(|a, b| a.title.cmp(&b.title));
        eprintln!("Movie Library:");
        let movie_count = movies.len() - 1;
        for (current_movie_index, movie) in movies.iter().enumerate() {
            println!(
                "{} Movie: '{}'{}",
                if current_movie_index == movie_count {
                    "└───"
                } else {
                    "├───"
                },
                movie.title,
                match movie.extra {
                    Some(ref extra) => match extra.series {
                        Some(ref series) => {
                            format!("\t{} in {}", series.index, series.name)
                        }
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                }
            );
            println!(
                "{}    └─── {}",
                if current_movie_index < movie_count {
                    "│"
                } else {
                    " "
                },
                movie.path.clone().into_os_string().to_string_lossy()
            );
        }
    }
}
