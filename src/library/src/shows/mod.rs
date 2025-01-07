mod find;
mod smodel;

pub use find::find_show_file_paths;
pub use find::find_shows_in_dir;
pub use smodel::ShowMetadata;
pub use smodel::{Show, ShowKey, ShowSeason, Shows};

// Example output from ffprobe

/*
    show            : Avatar: The Last Airbender
    album           : Avatar: The Last Airbender
    season_number   : 1
    grouping        : Book One: Water
    episode_id      : The King of Omashu
    episode_sort    : 5
    title           : Avatar: The Last Airbender; Book One: Water; The King of Omashu

    show            : Pokémon: Indigo League
    album           : Pokémon: Indigo League
    season_number   : 1
    grouping        : Season One
    episode_id      : Abra and the Psychic Showdown (Part 1)
    episode_sort    : 22
    title           : Pokémon: Indigo League; Abra and the Psychic Showdown (Part 1)
*/
