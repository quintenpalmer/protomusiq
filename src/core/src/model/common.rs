use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub enum LoadMode {
    NoCache,
    Latest,
    Json,
    Sqlite,
}

#[derive(Serialize, Deserialize)]
pub enum AlbumSizeWithOrig {
    Micro,
    Mini,
    Small,
    Centi,
    Regular,
    Large,
    Original,
}

impl AlbumSizeWithOrig {
    pub fn get_filename(&self) -> String {
        match self {
            AlbumSizeWithOrig::Micro => "micro.png",
            AlbumSizeWithOrig::Mini => "mini.png",
            AlbumSizeWithOrig::Centi => "centi.png",
            AlbumSizeWithOrig::Small => "small.png",
            AlbumSizeWithOrig::Regular => "regular.png",
            AlbumSizeWithOrig::Large => "large.png",
            AlbumSizeWithOrig::Original => "orig.jpg",
        }
        .to_string()
    }
}
