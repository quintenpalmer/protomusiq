use std::path::Path;

#[derive(Debug, Clone)]
pub struct AppImages {
    artists_image: Vec<u8>,
    albums_image: Vec<u8>,
    tracks_image: Vec<u8>,
    playlists_image: Vec<u8>,
}

impl AppImages {
    pub fn new<P: AsRef<Path>>(_app_data_path: P) -> Self {
        let artists_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/artists.png",));
        let albums_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/albums.png"));
        let tracks_image =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/embedded/tracks.png"));
        let playlists_image = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/embedded/playlists.png"
        ));

        AppImages {
            artists_image: artists_image.to_vec(),
            albums_image: albums_image.to_vec(),
            tracks_image: tracks_image.to_vec(),
            playlists_image: playlists_image.to_vec(),
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

    pub fn get_playlists_image(&self) -> &Vec<u8> {
        &self.playlists_image
    }
}
