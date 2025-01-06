use std::path::PathBuf;

use musiqlibrary::shows;

use super::commands::AppCmd;

pub struct ShowTreeViewer {}

impl AppCmd for ShowTreeViewer {
    fn operate(&self, path: PathBuf) {
        let shows = shows::find_shows_in_dir(path.clone());
        let structured = shows::Shows::from_vec(&shows);
        println!("All Shows:");
        let show_count = structured.shows.len() - 1;
        for (show_index, show) in structured.shows.values().enumerate() {
            println!("\t{}Show: {}", tree_arm(show_index, show_count), show.name);
            let season_count = show.seasons.len() - 1;
            for (season_index, season) in show.seasons.values().enumerate() {
                println!(
                    "\t{}\t{}Season: {}",
                    tree_stem(show_index, show_count),
                    tree_arm(season_index, season_count),
                    pretty_number_and_name(season.number, &season.name)
                );
                let episode_count = season.episodes.len() - 1;
                for (episode_index, episode) in season.episodes.values().enumerate() {
                    println!(
                        "\t{}\t{}\t{}Episode: {}",
                        tree_stem(show_index, show_count),
                        tree_stem(season_index, season_count),
                        tree_arm(episode_index, episode_count),
                        pretty_number_and_name(episode.episode_sort, &episode.episode_id)
                    );
                }
            }
        }
    }
}

fn tree_stem(parent_index: usize, parent_len: usize) -> String {
    if parent_index == parent_len {
        " ".to_string()
    } else {
        "│".to_string()
    }
}
fn tree_arm(current_index: usize, total_len: usize) -> String {
    if current_index == total_len {
        "└── ".to_string()
    } else {
        "├── ".to_string()
    }
}

fn pretty_number_and_name(num: u32, maybe_name: &Option<String>) -> String {
    match maybe_name {
        Some(name) => format!("{} - {}", num, name),
        None => format!("{}", num),
    }
}
