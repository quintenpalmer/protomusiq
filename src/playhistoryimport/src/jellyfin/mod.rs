mod compute;
mod debuginfo;
mod jmodel;
mod release;
mod repl;

pub fn translate_jellyfin_play_history() {
    let mut resulting_information = compute::compute_jellyfin_play_history_for_musiqapp();

    resulting_information.sort_relevant();

    debuginfo::write_debug_info_to_disc(&resulting_information);

    let release_information = release::compute_release_files(&resulting_information);

    release::write_release_files(&release_information);
}
