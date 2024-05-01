use std::path::Path;

#[derive(Debug, Clone)]
pub struct AppImages {
    artists_image: Vec<u8>,
    albums_image: Vec<u8>,
    tracks_image: Vec<u8>,
    tag_image: Vec<u8>,
    playlists_image: Vec<u8>,
    search_image: Vec<u8>,
    settings_image: Vec<u8>,
    dvd_image: Vec<u8>,
    game_controller_image: Vec<u8>,
}

impl AppImages {
    pub fn new<P: AsRef<Path>>(_app_data_path: P) -> Self {
        let artists_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/artists.png",));
        let albums_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/albums.png"));
        let tracks_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/tracks.png"));
        let tag_image = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/tag.png"));
        let playlists_image = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/embedded/playlists.png"
        ));
        let search_image = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/embedded/search_mag.png"
        ));
        let settings_image = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/embedded/settings_gear.png",
        ));

        let dvd_image = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/dvd.png",));

        let game_controller_image = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/embedded/game_controller.png",
        ));

        AppImages {
            artists_image: artists_image.to_vec(),
            albums_image: albums_image.to_vec(),
            tracks_image: tracks_image.to_vec(),
            tag_image: tag_image.to_vec(),
            playlists_image: playlists_image.to_vec(),
            search_image: search_image.to_vec(),
            settings_image: settings_image.to_vec(),
            dvd_image: dvd_image.to_vec(),
            game_controller_image: game_controller_image.to_vec(),
        }
    }

    pub fn get_artists_image(&self) -> &Vec<u8> {
        &self.artists_image
    }

    pub fn get_albums_image(&self) -> &Vec<u8> {
        &self.albums_image
    }

    pub fn get_tracks_image(&self) -> &Vec<u8> {
        &self.tracks_image
    }

    pub fn get_tag_image(&self) -> &Vec<u8> {
        &self.tag_image
    }

    pub fn get_playlists_image(&self) -> &Vec<u8> {
        &self.playlists_image
    }

    pub fn get_search_image(&self) -> &Vec<u8> {
        &self.search_image
    }

    pub fn get_settings_image(&self) -> &Vec<u8> {
        &self.settings_image
    }

    pub fn get_dvd_image(&self) -> &Vec<u8> {
        &self.dvd_image
    }

    pub fn get_game_controller_image(&self) -> &Vec<u8> {
        &self.game_controller_image
    }
}
