use std::path;

use musiqlibrary::video;

pub use video::Error;

pub fn run_app() -> Result<(), Error> {
    let movie_dir_filename = "/home/quinten/storage/media/movies/";

    println!("searching ~/storage/media/movies");

    let movie_files = video::find_movies(path::Path::new(movie_dir_filename).to_path_buf());

    println!("found {} files", movie_files.len());

    let all_movie_metadata: Vec<_> = movie_files
        .iter()
        .map(video::find_movie_metadata)
        .filter_map(|x| {
            match x {
                Ok(_) => (),
                Err(ref e) => println!("error: {:?}", e),
            };
            x.ok()
        })
        .collect();

    println!(
        "found and filtered {} movies with metadata",
        all_movie_metadata.len()
    );

    for movie in all_movie_metadata.into_iter() {
        println!("Movie: {}", movie.title);
        println!("     : {}", movie.path.into_os_string().to_string_lossy());
    }

    println!("exiting");

    Ok(())
}
