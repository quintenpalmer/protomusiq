pub fn format_duration(seconds: u64) -> String {
    let to_display_seconds = seconds % 60;
    let to_display_minutes = (seconds / 60) % 60;
    let to_display_hours = (seconds / 3600) % 60;
    if to_display_hours > 0 {
        format!(
            "{}:{:02}:{:02}",
            to_display_hours, to_display_minutes, to_display_seconds
        )
    } else {
        if to_display_minutes > 0 {
            format!("{}:{:02}", seconds / 60, seconds % 60)
        } else {
            format!("0:{:02}", seconds)
        }
    }
}

pub fn abr_str(mut s: String, mut new_len: usize) -> String {
    if s.len() < new_len {
        return s;
    }
    let panic_printable = s.clone();
    for _i in 0..16 {
        if s.is_char_boundary(new_len) {
            s.truncate(new_len);
            return format!("{}…", s);
        }
        new_len = new_len - 1;
    }
    panic!(
        "couldn't find char boundary within 16 chars of {} for {}",
        new_len, panic_printable
    );
}

pub fn get_page<'a, A>(list: &'a Vec<A>, page_number: usize, page_size: usize) -> &'a [A] {
    let mut chunks = list.chunks(page_size);
    if page_number < chunks.len() {
        chunks.nth(page_number).unwrap()
    } else {
        chunks.nth(chunks.len() - 1).unwrap()
    }
}

pub fn format_date_range(start_date: u32, _end_date: u32) -> String {
    format!("{:04}", start_date)
}

pub fn fold_strings<S: ToString>(strings: &Vec<S>) -> String {
    match strings
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .as_slice()
    {
        [] => "".to_string(),
        [single] => single.to_string(),
        to_pass => fold_known_many_strings(to_pass),
    }
}

fn fold_known_many_strings(strings: &[String]) -> String {
    format!("[{}]", strings.join(", "))
}
