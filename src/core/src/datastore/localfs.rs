use std::fs;
use std::io;
use std::path::PathBuf;

pub fn build_tree_for_dirs<S: Into<String>>(
    app_data_path: &PathBuf,
    child_list: Vec<S>,
) -> PathBuf {
    let mut final_path = app_data_path.clone();
    confirm_dir(&final_path).unwrap();
    for child in child_list.into_iter() {
        final_path = final_path.join(child.into());
        confirm_dir(&final_path).unwrap();
    }

    final_path
}

pub fn build_tree_for_file<S: Into<String>>(
    app_data_path: &PathBuf,
    child_list: Vec<S>,
) -> PathBuf {
    let mut final_path = app_data_path.clone();
    confirm_dir(&final_path).unwrap();
    let last_index = child_list.len() - 1;
    for (index, child) in child_list.into_iter().enumerate() {
        final_path = final_path.join(child.into());
        if index != last_index {
            confirm_dir(&final_path).unwrap();
        }
    }

    final_path
}

pub fn confirm_dir(path: &PathBuf) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match fs::create_dir_all(path) {
                Ok(_) => Ok(()),
                Err(inner_e) => Err(format!("could not create path: {:?}", inner_e)),
            },
            _ => Err(format!("could not access path: {:?}", e)),
        },
    }
}

pub fn check_exists(path: &PathBuf) -> bool {
    fs::metadata(path).is_ok()
}
