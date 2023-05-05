mod gplaymusic;
mod jellyfin;
mod model;
mod spotify;

use std::env;

pub enum RunMode {
    Noop,
    Jellyfin,
    GPlayMusic,
    Spotify,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut run_mode = RunMode::Noop;

    for arg in args.into_iter() {
        if arg == "jellyfin" {
            run_mode = RunMode::Jellyfin;
        }
        if arg == "gplaymusic" {
            run_mode = RunMode::GPlayMusic;
        }
        if arg == "spotify" {
            run_mode = RunMode::Spotify;
        }
    }

    match run_mode {
        RunMode::Noop => println!("performing no action with no arguments provided"),
        RunMode::Jellyfin => jellyfin::translate_jellyfin_play_history(),
        RunMode::GPlayMusic => gplaymusic::translate_gplay_music_play_history(),
        RunMode::Spotify => spotify::translate_gplay_music_play_history(),
    }
}
