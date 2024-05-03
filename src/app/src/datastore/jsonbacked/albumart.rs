use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path;
use std::path::PathBuf;

use image;
use image::io::Reader as ImageReader;

use crate::model;

use super::super::localfs;

trait CachedAlbumImageInfo {
    fn has_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> bool;

    fn get_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> Vec<u8>;

    fn get_path_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> path::PathBuf;

    fn write_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
        bytes: Vec<u8>,
    );
}

struct FilesystemAlbumArt {
    cache_album_dir: PathBuf,
}

struct FilesystemCachedAlbumArt {
    album_art: BTreeMap<musiqlibrary::AlbumUniqueIdentifier, FilesystemAlbumArt>,
}

impl FilesystemCachedAlbumArt {
    fn new(library: &musiqlibrary::RawLibrary, app_data_path: PathBuf) -> Self {
        let mut cache = FilesystemCachedAlbumArt {
            album_art: BTreeMap::new(),
        };

        let album_cache_path =
            localfs::build_tree_for_dirs(&app_data_path, vec!["cache", "images", "albums"]);

        for artist in library.artists.values() {
            for album in artist.albums.values() {
                let key = musiqlibrary::AlbumUniqueIdentifier::new(
                    artist.artist_info.artist_id,
                    album.album_info.album_id,
                );

                let local_path = album.album_info.relative_path.clone().join("cover.jpg");
                let local_dir = local_path.parent().unwrap();
                let cache_album_dir = album_cache_path.join(local_dir);
                localfs::confirm_dir(&cache_album_dir).unwrap();

                cache
                    .album_art
                    .insert(key, FilesystemAlbumArt { cache_album_dir });
            }
        }

        cache
    }

    fn get_cache_album_path(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> Option<PathBuf> {
        match self.album_art.get(key) {
            Some(v) => {
                let name = album_size.get_filename();
                let full_album_size_path = v.cache_album_dir.join(name);
                Some(full_album_size_path)
            }
            None => None,
        }
    }
}

impl CachedAlbumImageInfo for FilesystemCachedAlbumArt {
    fn has_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> bool {
        match self.get_cache_album_path(key, album_size) {
            Some(full_album_size_path) => localfs::check_exists(&full_album_size_path),
            None => panic!("why didn't i know about this {:?}", key),
        }
    }

    fn get_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> Vec<u8> {
        match self.get_cache_album_path(key, album_size) {
            Some(full_album_size_path) => fs::read(full_album_size_path.clone()).unwrap(),
            None => panic!("why didn't i know about this {:?}", key),
        }
    }

    fn get_path_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
    ) -> path::PathBuf {
        match self.get_cache_album_path(key, album_size) {
            Some(full_album_size_path) => full_album_size_path.clone(),
            None => panic!("why didn't i know about this {:?}", key),
        }
    }

    fn write_art_for_size(
        &self,
        key: &musiqlibrary::AlbumUniqueIdentifier,
        album_size: model::AlbumSizeWithOrig,
        bytes: Vec<u8>,
    ) {
        match self.get_cache_album_path(key, album_size) {
            Some(full_album_size_path) => {
                fs::write(full_album_size_path.clone(), bytes).unwrap();
            }
            None => panic!("why didn't i know about this {:?}", key),
        }
    }
}

impl model::AlbumSizeWithOrig {
    fn get_filename(&self) -> String {
        match self {
            model::AlbumSizeWithOrig::Micro => "micro.png",
            model::AlbumSizeWithOrig::Mini => "mini.png",
            model::AlbumSizeWithOrig::Centi => "centi.png",
            model::AlbumSizeWithOrig::Small => "small.png",
            model::AlbumSizeWithOrig::Regular => "regular.png",
            model::AlbumSizeWithOrig::Large => "large.png",
            model::AlbumSizeWithOrig::Original => "orig.jpg",
        }
        .to_string()
    }
}

pub fn process_cache_and_get_album_art(
    library: &musiqlibrary::RawLibrary,
    app_data_path: PathBuf,
) -> model::AlbumArt {
    let mut large = BTreeMap::new();
    let mut regular = BTreeMap::new();
    let mut small = BTreeMap::new();
    let mut centi = BTreeMap::new();
    let mut mini = BTreeMap::new();
    let mut micro = BTreeMap::new();

    let mut found_cache_entries = 0;

    let cached_album_art_checker: Box<dyn CachedAlbumImageInfo> =
        Box::new(FilesystemCachedAlbumArt::new(library, app_data_path));

    for artist in library.artists.values() {
        for album in artist.albums.values() {
            let key = musiqlibrary::AlbumUniqueIdentifier::new(
                artist.artist_info.artist_id,
                album.album_info.album_id,
            );

            let has_micro =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Micro);
            let has_mini =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Mini);
            let has_centi =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Centi);
            let has_small =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Small);
            let has_regular =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Regular);
            let has_large =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Large);
            let has_orig =
                cached_album_art_checker.has_art_for_size(&key, model::AlbumSizeWithOrig::Original);

            let full_album_cover_path = album.album_info.path.clone().join("cover.jpg");
            let local_path = album.album_info.relative_path.clone().join("cover.jpg");

            if has_micro
                && has_mini
                && has_centi
                && has_small
                && has_regular
                && has_large
                && has_orig
            {
                found_cache_entries += 1;
            } else {
                println!(
                    "some missing data for {:?}, from {:?}",
                    local_path, full_album_cover_path
                );
                if !has_orig {
                    println!(
                        "copying original album art to cache dir for {:?}",
                        local_path
                    );
                    let album_cover_bytes = fs::read(full_album_cover_path.clone()).unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Original,
                        album_cover_bytes.clone(),
                    );
                }

                let orig_album_art = match ImageReader::open(full_album_cover_path.clone())
                    .unwrap()
                    .decode()
                {
                    Ok(v) => v,
                    Err(_e) => ImageReader::with_format(
                        io::BufReader::new(fs::File::open(full_album_cover_path.clone()).unwrap()),
                        image::ImageFormat::Png,
                    )
                    .decode()
                    .unwrap(),
                };

                if !has_large {
                    println!(
                        "translating large size album art to cache dir for {:?}",
                        local_path
                    );

                    let large_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::LARGE_ICON_WIDTH as u32,
                        model::LARGE_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    large_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Large,
                        buf.into_inner(),
                    );
                }

                if !has_regular {
                    println!(
                        "translating regular size album art to cache dir for {:?}",
                        local_path
                    );
                    let regular_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::REGULAR_ICON_WIDTH as u32,
                        model::REGULAR_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    regular_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Regular,
                        buf.into_inner(),
                    );
                }

                if !has_small {
                    println!(
                        "translating small size album art to cache dir for {:?}",
                        local_path
                    );
                    let small_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::SMALL_ICON_WIDTH as u32,
                        model::SMALL_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    small_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Small,
                        buf.into_inner(),
                    );
                }

                if !has_centi {
                    println!(
                        "translating centi size album art to cache dir for {:?}",
                        local_path
                    );
                    let centi_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::CENTI_ICON_WIDTH as u32,
                        model::CENTI_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    centi_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Centi,
                        buf.into_inner(),
                    );
                }

                if !has_mini {
                    println!(
                        "translating mini size album art to cache dir for {:?}",
                        local_path
                    );
                    let mini_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::MINI_ICON_WIDTH as u32,
                        model::MINI_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    mini_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Mini,
                        buf.into_inner(),
                    );
                }

                if !has_micro {
                    println!(
                        "translating micro size album art to cache dir for {:?}",
                        local_path
                    );
                    let micro_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::MICRO_ICON_WIDTH as u32,
                        model::MICRO_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );

                    let mut buf = io::Cursor::new(Vec::new());
                    micro_album_art
                        .write_to(&mut buf, image::ImageFormat::Png)
                        .unwrap();

                    cached_album_art_checker.write_art_for_size(
                        &key,
                        model::AlbumSizeWithOrig::Micro,
                        buf.into_inner(),
                    );
                }
            }

            let large_path =
                cached_album_art_checker.get_path_for_size(&key, model::AlbumSizeWithOrig::Large);
            large.insert(key.clone(), large_path);

            let regular_bytes =
                cached_album_art_checker.get_art_for_size(&key, model::AlbumSizeWithOrig::Regular);
            regular.insert(key.clone(), regular_bytes);

            let small_bytes =
                cached_album_art_checker.get_art_for_size(&key, model::AlbumSizeWithOrig::Small);
            small.insert(key.clone(), small_bytes);

            let centi_bytes =
                cached_album_art_checker.get_art_for_size(&key, model::AlbumSizeWithOrig::Centi);
            centi.insert(key.clone(), centi_bytes);

            let mini_bytes =
                cached_album_art_checker.get_art_for_size(&key, model::AlbumSizeWithOrig::Mini);
            mini.insert(key.clone(), mini_bytes);

            let micro_bytes =
                cached_album_art_checker.get_art_for_size(&key, model::AlbumSizeWithOrig::Micro);
            micro.insert(key.clone(), micro_bytes);
        }
    }

    println!("all cache data existed for {} entries", found_cache_entries);

    model::AlbumArt {
        large_album_covers: large,
        album_covers: regular,
        small_album_covers: small,
        centi_album_covers: centi,
        mini_album_covers: mini,
        micro_album_covers: micro,
    }
}

#[allow(unused)]
pub fn old_process_cache_and_get_album_art(
    library: &musiqlibrary::RawLibrary,
    app_data_path: PathBuf,
) -> model::AlbumArt {
    let mut large = BTreeMap::new();
    let mut regular = BTreeMap::new();
    let mut small = BTreeMap::new();
    let mut centi = BTreeMap::new();
    let mut mini = BTreeMap::new();
    let mut micro = BTreeMap::new();

    let mut found_cache_entries = 0;

    let album_cache_path =
        localfs::build_tree_for_dirs(&app_data_path, vec!["cache", "images", "albums"]);

    for artist in library.artists.values() {
        for album in artist.albums.values() {
            let key = musiqlibrary::AlbumUniqueIdentifier::new(
                artist.artist_info.artist_id,
                album.album_info.album_id,
            );

            let full_album_cover_path = album.album_info.path.clone().join("cover.jpg");
            let local_path = album.album_info.relative_path.clone().join("cover.jpg");
            let local_dir = local_path.parent().unwrap();
            let cache_album_dir = album_cache_path.join(local_dir);
            localfs::confirm_dir(&cache_album_dir).unwrap();

            let cached_large_album_art_path = cache_album_dir.join("large.png");
            let cached_regular_album_art_path = cache_album_dir.join("regular.png");
            let cached_small_album_art_path = cache_album_dir.join("small.png");
            let cached_centi_album_art_path = cache_album_dir.join("centi.png");
            let cached_mini_album_art_path = cache_album_dir.join("mini.png");
            let cached_micro_album_art_path = cache_album_dir.join("micro.png");
            let cached_orig_album_art_path = cache_album_dir.join("orig.jpg");

            if localfs::check_exists(&cached_large_album_art_path)
                && localfs::check_exists(&cached_regular_album_art_path)
                && localfs::check_exists(&cached_small_album_art_path)
                && localfs::check_exists(&cached_centi_album_art_path)
                && localfs::check_exists(&cached_mini_album_art_path)
                && localfs::check_exists(&cached_micro_album_art_path)
                && localfs::check_exists(&cached_orig_album_art_path)
            {
                found_cache_entries += 1;
            } else {
                println!(
                    "some missing data for {:?}, from {:?}",
                    local_path, full_album_cover_path
                );
                if !localfs::check_exists(&cached_orig_album_art_path) {
                    println!(
                        "copying original album art to cache dir for {:?}",
                        local_path
                    );
                    let album_cover_bytes = fs::read(full_album_cover_path.clone()).unwrap();
                    fs::write(
                        cached_orig_album_art_path.clone(),
                        album_cover_bytes.clone(),
                    )
                    .unwrap();
                }

                let orig_album_art = match ImageReader::open(full_album_cover_path.clone())
                    .unwrap()
                    .decode()
                {
                    Ok(v) => v,
                    Err(_e) => ImageReader::with_format(
                        io::BufReader::new(fs::File::open(full_album_cover_path.clone()).unwrap()),
                        image::ImageFormat::Png,
                    )
                    .decode()
                    .unwrap(),
                };

                if !localfs::check_exists(&cached_large_album_art_path) {
                    println!(
                        "translating large size album art to cache dir for {:?}",
                        local_path
                    );
                    let large_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::LARGE_ICON_WIDTH as u32,
                        model::LARGE_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    large_album_art
                        .save(cached_large_album_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_regular_album_art_path) {
                    println!(
                        "translating regular size album art to cache dir for {:?}",
                        local_path
                    );
                    let regular_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::REGULAR_ICON_WIDTH as u32,
                        model::REGULAR_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    regular_album_art
                        .save(cached_regular_album_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_small_album_art_path) {
                    println!(
                        "translating small size album art to cache dir for {:?}",
                        local_path
                    );
                    let small_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::SMALL_ICON_WIDTH as u32,
                        model::SMALL_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    small_album_art
                        .save(cached_small_album_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_centi_album_art_path) {
                    println!(
                        "translating centi size album art to cache dir for {:?}",
                        local_path
                    );
                    let centi_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::CENTI_ICON_WIDTH as u32,
                        model::CENTI_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    centi_album_art
                        .save(cached_centi_album_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_mini_album_art_path) {
                    println!(
                        "translating mini size album art to cache dir for {:?}",
                        local_path
                    );
                    let mini_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::MINI_ICON_WIDTH as u32,
                        model::MINI_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    mini_album_art
                        .save(cached_mini_album_art_path.clone())
                        .unwrap();
                }

                if !localfs::check_exists(&cached_micro_album_art_path) {
                    println!(
                        "translating micro size album art to cache dir for {:?}",
                        local_path
                    );
                    let micro_album_art = image::imageops::resize(
                        &orig_album_art,
                        model::MICRO_ICON_WIDTH as u32,
                        model::MICRO_ICON_HEIGHT as u32,
                        image::imageops::FilterType::Lanczos3,
                    );
                    micro_album_art
                        .save(cached_micro_album_art_path.clone())
                        .unwrap();
                }
            }

            large.insert(key.clone(), cached_large_album_art_path);

            let regular_bytes = fs::read(cached_regular_album_art_path).unwrap();
            regular.insert(key.clone(), regular_bytes);

            let small_bytes = fs::read(cached_small_album_art_path).unwrap();
            small.insert(key.clone(), small_bytes);

            let centi_bytes = fs::read(cached_centi_album_art_path).unwrap();
            centi.insert(key.clone(), centi_bytes);

            let mini_bytes = fs::read(cached_mini_album_art_path).unwrap();
            mini.insert(key.clone(), mini_bytes);

            let micro_bytes = fs::read(cached_micro_album_art_path).unwrap();
            micro.insert(key.clone(), micro_bytes);
        }
    }

    println!("all cache data existed for {} entries", found_cache_entries);

    model::AlbumArt {
        large_album_covers: large,
        album_covers: regular,
        small_album_covers: small,
        centi_album_covers: centi,
        mini_album_covers: mini,
        micro_album_covers: micro,
    }
}
