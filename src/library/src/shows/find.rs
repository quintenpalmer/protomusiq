use std::sync::mpsc;
use std::{fs, path, str, time};

pub use super::smodel::{Error, ShowMetadata};

/// Recursively find all show metadata
pub fn find_shows_in_dir<P: AsRef<path::Path>>(show_root_path: P) -> Vec<ShowMetadata> {
    let all_paths = find_show_file_paths(show_root_path.as_ref().to_path_buf());

    let mut all_show_metadata = Vec::new();

    let num_threads = std::thread::available_parallelism()
        .map(|x| x.into())
        .unwrap_or(1);

    eprintln!("creating thread pool with {} threads", num_threads);

    let tpool = threadpool::ThreadPool::new(num_threads);
    let (tx, rx) = mpsc::channel();

    for specific_show_path in all_paths.into_iter() {
        let tx = tx.clone();

        let show_path = show_root_path.as_ref().to_path_buf().clone();

        tpool.execute(
            move || match find_show_metadata(&show_path, &specific_show_path) {
                Ok(v) => tx
                    .send(v)
                    .expect("I hope the show metadata rx is receiving"),
                Err(ref e) => eprintln!("error: {:?}", e),
            },
        );
    }

    drop(tx);

    for show_metadata in rx {
        all_show_metadata.push(show_metadata);
    }

    all_show_metadata
}

/// Recursively find the paths for all show files in a directory
pub fn find_show_file_paths(current_path: path::PathBuf) -> Vec<path::PathBuf> {
    if current_path.is_file() {
        let fileext = current_path
            .extension()
            .expect("filename should have ext")
            .to_str()
            .unwrap();

        return match fileext {
            "m4v" | "mp4" => vec![current_path.clone()],
            "jpg" | "json" => Vec::new(), // shows should have jpg box art and json metadata
            // files, but there's nothing to do with them here
            _ => {
                eprintln!("unexpected file extension: {}", fileext);
                Vec::new()
            }
        };
    }
    if current_path.is_dir() {
        let mut ret = Vec::new();
        for entry in current_path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let mut children = find_show_file_paths(entry.path());
            ret.append(&mut children);
        }
        return ret;
    }
    eprintln!(
        "unexpected file type: {}",
        current_path.into_os_string().to_string_lossy()
    );

    Vec::new()
}

/// Extract the show metadata from a given show file path
pub fn find_show_metadata(
    orig_scan_path: &path::PathBuf,
    show_path: &path::PathBuf,
) -> Result<ShowMetadata, Error> {
    eprintln!(
        "path: {}",
        show_path.clone().into_os_string().to_string_lossy()
    );
    let fileext = path::Path::new(show_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();

    match fileext {
        "m4v" | "mp4" => find_mp4_metadata(orig_scan_path, show_path),
        _ => Err(Error::NonMP4File),
    }
}

/// Extract the show metadata for a "MPEG-4 Part 14" or "MP4" file
/// https://en.wikipedia.org/wiki/MP4_file_format
/// https://en.wikipedia.org/wiki/Comparison_of_video_container_formats
fn find_mp4_metadata(
    orig_scan_path: &path::PathBuf,
    show_path: &path::PathBuf,
) -> Result<ShowMetadata, Error> {
    let show_file = fs::File::open(show_path).unwrap();
    let raw_metadata = mp4::read_mp4(show_file).unwrap();

    let tag = mp4ameta::Tag::read_from_path(show_path).unwrap_or_else(|_| {
        panic!(
            "could not load m4a file: {}",
            show_path
                .clone()
                .into_os_string()
                .to_string_lossy()
                .to_string()
        )
    });

    // for title and maybe more metadata
    let udta = raw_metadata.moov.udta.unwrap();
    // for duration
    let mvhd = raw_metadata.moov.mvhd;

    let title = match udta.meta.unwrap() {
        mp4::MetaBox::Mdir { ilst } => {
            let ilst = ilst.unwrap();
            //eprintln!("ilst {:?}", ilst.items);
            let title_data = &ilst.items.get(&mp4::MetadataKey::Title).unwrap().data;
            if title_data.data_type == mp4::DataType::Text {
                Ok(str::from_utf8(title_data.data.as_slice())
                    .unwrap()
                    .to_string())
            } else {
                Err(Error::NonTextTitle)
            }
        }
        _ => Err(Error::NonMP4File),
    }?;

    let duration = time::Duration::from_millis(mvhd.duration);

    let last_mod = fs::metadata(show_path).unwrap().modified().unwrap();

    Ok(ShowMetadata {
        full_path: show_path.clone(),
        rel_path: show_path
            .to_path_buf()
            .strip_prefix(orig_scan_path)
            .unwrap()
            .to_path_buf(),
        last_modified: last_mod,
        duration,

        show: tag.tv_show_name().unwrap().to_string(),
        album: tag.album().unwrap().to_string(),
        season_number: tag.tv_season().unwrap(),
        grouping: tag.grouping().map(|x| x.to_string()),
        episode_id: tag.tv_episode_name().map(|x| x.to_string()),
        episode_sort: tag.tv_episode().unwrap(),
        episode_sort_tiebreak: None,
        title,
    })
}
