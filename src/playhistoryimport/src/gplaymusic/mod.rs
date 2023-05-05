mod cleanup;
mod debuginfo;
mod gmodel;
mod matching;
mod release;
mod repl;
mod util;

pub fn translate_gplay_music_play_history() {
    cleanup::maybe_group_all_csvs_into_one();

    cleanup::maybe_clean_and_convert_to_json();

    let mut best_effort_matching = matching::match_gplay_music_data_for_musiqapp();

    best_effort_matching.sort_relevant();

    debuginfo::write_debug_info_to_disc(&best_effort_matching);

    let release_information = release::compute_release_files(&best_effort_matching);

    release::write_release_files(&release_information);
}
