use std::path;

pub fn clean_filename_stem(path: &path::PathBuf) -> String {
    path.file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("<unknown>".to_string())
}

pub fn clean_filename_to_game_name(path: &path::PathBuf) -> String {
    let unsplit = clean_filename_stem(path);

    let unstripped = unsplit.split("(").collect::<Vec<_>>()[0].to_string();

    let stripped_and_split = unstripped
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    stripped_and_split
}

pub fn get_game_region_info(path: &path::PathBuf) -> String {
    let unsplit = clean_filename_stem(path);

    let mut unjoined = unsplit.split("(").collect::<Vec<_>>();

    unjoined.remove(0);

    let unstripped = unjoined.join("(");

    let stripped_and_split = unstripped
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unstripped.clone());

    stripped_and_split
}
