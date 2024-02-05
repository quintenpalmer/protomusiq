use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;

use image::io::Reader as ImageReader;
use image::GenericImageView;

use crate::model;

use super::super::localfs;

pub fn process_cache_and_get_movie_art(
    library: &model::VideoLibrary,
    app_data_path: path::PathBuf,
) -> model::MovieArt {
    let mut large_movie_covers = BTreeMap::new();
    let mut semilarge_movie_covers = BTreeMap::new();
    let mut regular_movie_covers = BTreeMap::new();
    let mut small_movie_covers = BTreeMap::new();
    let mut micro_movie_covers = BTreeMap::new();

    let mut found_cache_entries = 0;

    let movie_cache_path =
        localfs::build_tree_for_dirs(&app_data_path, vec!["cache", "images", "movies"]);

    //let cached_movie_art_checker = FilesystemCachedMovieArt::new(library, app_data_path);

    for movie in library.movies.iter() {
        let key = model::MovieTitle::from_metadata(&movie);

        let full_movie_path = movie.path.clone();
        let full_movie_cover_path = movie.path.parent().unwrap().join("cover.jpg");

        let local_dir = movie.relative_path.parent().unwrap();
        let cache_movie_dir = movie_cache_path.join(local_dir);
        localfs::confirm_dir(&cache_movie_dir).unwrap();

        let cached_large_movie_art_path = cache_movie_dir.join("large.png");
        let cached_semilarge_movie_art_path = cache_movie_dir.join("semilarge.png");
        let cached_regular_movie_art_path = cache_movie_dir.join("regular.png");
        let cached_small_movie_art_path = cache_movie_dir.join("small.png");
        let cached_micro_movie_art_path = cache_movie_dir.join("micro.png");
        let cached_orig_movie_art_path = cache_movie_dir.join("orig.jpg");

        if localfs::check_exists(&cached_large_movie_art_path)
            && localfs::check_exists(&cached_semilarge_movie_art_path)
            && localfs::check_exists(&cached_regular_movie_art_path)
            && localfs::check_exists(&cached_small_movie_art_path)
            && localfs::check_exists(&cached_micro_movie_art_path)
            && localfs::check_exists(&cached_orig_movie_art_path)
        {
            found_cache_entries += 1;
        } else {
            println!(
                "some missing data for {:?}, from {:?}",
                local_dir, full_movie_path
            );

            if !localfs::check_exists(&full_movie_cover_path) {
                println!(
                    "skipping a movie with no album art: {:?}",
                    full_movie_cover_path
                );
            } else {
                if !localfs::check_exists(&cached_orig_movie_art_path) {
                    println!(
                        "copying original movie art to cache dir for {:?}",
                        local_dir
                    );
                    let movie_cover_bytes = fs::read(full_movie_cover_path.clone()).unwrap();
                    fs::write(
                        cached_orig_movie_art_path.clone(),
                        movie_cover_bytes.clone(),
                    )
                    .unwrap();
                }

                let orig_movie_art = match ImageReader::open(full_movie_cover_path.clone())
                    .unwrap()
                    .decode()
                {
                    Ok(v) => v,
                    Err(e) => {
                        println!("err: {:?}", e);
                        ImageReader::with_format(
                            io::BufReader::new(
                                fs::File::open(full_movie_cover_path.clone()).unwrap(),
                            ),
                            image::ImageFormat::Jpeg,
                        )
                        .decode()
                        .unwrap()
                    }
                };

                let (orig_width, orig_height) = orig_movie_art.dimensions();

                if !localfs::check_exists(&cached_large_movie_art_path) {
                    println!(
                        "translating large size movie art to cache dir for {:?}",
                        local_dir
                    );
                    let large_movie_art = image::imageops::resize(
                        &orig_movie_art,
                        ((model::DVD_LARGE_ICON_HEIGHT as u32) * orig_width) / orig_height,
                        model::DVD_LARGE_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    large_movie_art
                        .save(cached_large_movie_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_semilarge_movie_art_path) {
                    println!(
                        "translating semi-large size movie art to cache dir for {:?}",
                        local_dir
                    );
                    let semilarge_movie_art = image::imageops::resize(
                        &orig_movie_art,
                        ((model::DVD_SEMILARGE_ICON_HEIGHT as u32) * orig_width) / orig_height,
                        model::DVD_SEMILARGE_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    semilarge_movie_art
                        .save(cached_semilarge_movie_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_regular_movie_art_path) {
                    println!(
                        "translating regular size movie art to cache dir for {:?}",
                        local_dir
                    );
                    let regular_movie_art = image::imageops::resize(
                        &orig_movie_art,
                        ((model::DVD_REGULAR_ICON_HEIGHT as u32) * orig_width) / orig_height,
                        model::DVD_REGULAR_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    regular_movie_art
                        .save(cached_regular_movie_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_small_movie_art_path) {
                    println!(
                        "translating small size movie art to cache dir for {:?}",
                        local_dir
                    );
                    let small_movie_art = image::imageops::resize(
                        &orig_movie_art,
                        ((model::DVD_SMALL_ICON_HEIGHT as u32) * orig_width) / orig_height,
                        model::DVD_SMALL_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    small_movie_art
                        .save(cached_small_movie_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_micro_movie_art_path) {
                    println!(
                        "translating micro size movie art to cache dir for {:?}",
                        local_dir
                    );
                    let micro_movie_art = image::imageops::resize(
                        &orig_movie_art,
                        ((model::DVD_MICRO_ICON_HEIGHT as u32) * orig_width) / orig_height,
                        model::DVD_MICRO_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    micro_movie_art
                        .save(cached_micro_movie_art_path.clone())
                        .unwrap();
                }
            }
        }

        if localfs::check_exists(&cached_large_movie_art_path) {
            let large_bytes = fs::read(cached_large_movie_art_path).unwrap();
            large_movie_covers.insert(key.clone(), large_bytes);
        }

        if localfs::check_exists(&cached_semilarge_movie_art_path) {
            let large_bytes = fs::read(cached_semilarge_movie_art_path).unwrap();
            semilarge_movie_covers.insert(key.clone(), large_bytes);
        }

        if localfs::check_exists(&cached_regular_movie_art_path) {
            let regular_bytes = fs::read(cached_regular_movie_art_path).unwrap();
            regular_movie_covers.insert(key.clone(), regular_bytes);
        }

        if localfs::check_exists(&cached_small_movie_art_path) {
            let small_bytes = fs::read(cached_small_movie_art_path).unwrap();
            small_movie_covers.insert(key.clone(), small_bytes);
        }

        if localfs::check_exists(&cached_micro_movie_art_path) {
            let micro_bytes = fs::read(cached_micro_movie_art_path).unwrap();
            micro_movie_covers.insert(key.clone(), micro_bytes);
        }
    }

    println!("saw this many pre-cached movies: {}", found_cache_entries);

    model::MovieArt {
        large_movie_covers,
        semilarge_movie_covers,
        regular_movie_covers,
        small_movie_covers,
        micro_movie_covers,
    }
}
