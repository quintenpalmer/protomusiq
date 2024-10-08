use std::cmp;
use std::env;
use std::path;

pub fn best_effort_path_to_string<P: AsRef<path::Path>>(p: P) -> String {
    p.as_ref()
        .to_path_buf()
        .into_os_string()
        .to_string_lossy()
        .to_string()
}

pub fn get_default_config_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let config_path = path::Path::new(&home_dir)
        .join(".config")
        .join("musiqapp")
        .join("config.json");

    config_path
}

pub fn get_default_data_path() -> path::PathBuf {
    let home_dir = env::var_os("HOME").unwrap();
    let data_path = path::Path::new(&home_dir)
        .join(".local")
        .join("share")
        .join("musiq")
        .join("v1");

    data_path
}

pub fn levenshtein(first: &str, second: &str) -> usize {
    let first_len = first.chars().count();
    let second_len = second.chars().count();
    let mut two_d_matrix = vec![vec![0; second_len + 1]; first_len + 1];

    for i in 1..(first_len + 1) {
        two_d_matrix[i][0] = i;
    }

    for j in 1..(second_len + 1) {
        two_d_matrix[0][j] = j;
    }

    for (i, first_char) in first.chars().enumerate() {
        for (j, second_char) in second.chars().enumerate() {
            let substitution_cost = if first_char == second_char { 0 } else { 1 };

            let mut iplus_jplus_value = two_d_matrix[i][j + 1] + 1;
            iplus_jplus_value = cmp::min(iplus_jplus_value, two_d_matrix[i + 1][j] + 1);
            iplus_jplus_value = cmp::min(iplus_jplus_value, two_d_matrix[i][j] + substitution_cost);

            two_d_matrix[i + 1][j + 1] = iplus_jplus_value;
        }
    }

    two_d_matrix[first_len][second_len]
}
