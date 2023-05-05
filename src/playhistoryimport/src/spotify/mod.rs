mod debuginfo;
mod matching;
mod release;
mod repl;
mod smodel;
mod translate;
mod util;

pub fn translate_gplay_music_play_history() {
    translate::maybe_clean_and_convert_json();

    let debug_info = matching::match_spotify_music_data_for_musiqapp();

    debuginfo::write_debug_info_to_disc(&debug_info);

    release::compute_and_write_release_info(&debug_info);
}
