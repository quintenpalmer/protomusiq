use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct PlaylistEntry {
    pub id: u32,
    pub name: String,
    pub tracks: Vec<musiqlibrary::TrackUniqueIdentifier>,
}
