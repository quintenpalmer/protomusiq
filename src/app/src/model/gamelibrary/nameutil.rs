use std::path;

pub fn clean_filename_to_game_name(path: &path::PathBuf) -> String {
    let unsplit = path
        .file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or("<unknown>".to_string());

    let unstripped = unsplit
        .strip_suffix(" ")
        .map(|x| x.to_string())
        .unwrap_or(unsplit.clone());

    let stripped = unstripped.split("(").collect::<Vec<_>>()[0].to_string();

    stripped
}
