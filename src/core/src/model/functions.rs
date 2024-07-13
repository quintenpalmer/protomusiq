use std::cmp;

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
