use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use musiqlibrary;

use crate::model;

use super::super::localfs;
use super::common;

const EMPTY_PLAYLIST_VEC: Vec<model::playlist::PlaylistEntry> = Vec::new();

pub struct PlaylistData {
    pub json_db_path: PathBuf,
    pub inner: Option<InnerPlaylistData>,
}

impl PlaylistData {
    pub fn new(app_data_path: &PathBuf) -> Self {
        let json_db_path = localfs::build_tree_for_file(
            &app_data_path,
            vec!["data", "playlists", "playlists.json"],
        );

        let maybe_raw = common::maybe_get_existing_raw_data(&json_db_path);
        let inner = maybe_raw.map(|raw| InnerPlaylistData::from_raw(raw));

        PlaylistData {
            json_db_path,
            inner,
        }
    }

    pub fn add_playlist(&mut self, name: String) {
        let new_inner = match &self.inner {
            None => Some(InnerPlaylistData::new(name.clone())),
            Some(ref _v) => None,
        };

        match new_inner {
            Some(inner) => match self.inner {
                None => self.inner = Some(inner),
                Some(ref _v) => panic!("should not be trying to overwrite existing inner"),
            },
            None => match self.inner {
                Some(ref mut v) => v.add(name),
                None => {
                    panic!("should never have no playlists while calling `add_playlist`")
                }
            },
        };

        match &self.inner {
            Some(ref inner) => inner.write_json(&self.json_db_path),
            None => panic!("should never have no playlists after calling `add_playlist`"),
        };
    }

    pub fn get_playlist(&self, playlist_id: u32) -> Option<&model::playlist::PlaylistEntry> {
        match self.inner {
            Some(ref v) => v.playlists.get(&playlist_id),
            None => None,
        }
    }

    pub fn delete_playlist(&mut self, playlist_id: u32) -> Result<(), String> {
        match self.inner {
            Some(ref mut v) => {
                let ret = v.delete_playlist(playlist_id);
                v.write_json(&self.json_db_path);
                ret
            }
            None => Err("There are no playlists".to_string()),
        }
    }

    pub fn make_playlist_default(&mut self, playlist_id: u32) {
        match self.inner {
            Some(ref mut v) => {
                v.make_playlist_default(playlist_id);
                v.write_json(&self.json_db_path);
            }
            None => (),
        }
    }

    pub fn add_track_to_playlist(
        &mut self,
        playlist_id: u32,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.inner {
            Some(ref mut v) => {
                let ret = v.add_track_to_playlist(playlist_id, track_id);
                v.write_json(&self.json_db_path);
                ret
            }
            None => Err("There are no playlists".to_string()),
        }
    }

    pub fn remove_track_from_playlist(
        &mut self,
        playlist_id: u32,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.inner {
            Some(ref mut v) => {
                let ret = v.remove_track_from_playlist(playlist_id, track_id);
                v.write_json(&self.json_db_path);
                ret
            }
            None => Err("no playlists to remove from".to_string()),
        }
    }

    pub fn move_track_in_playlist(
        &mut self,
        playlist_id: u32,
        direction: model::Direction,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.inner {
            Some(ref mut v) => {
                let ret = v.move_track_in_playlist(playlist_id, direction, track_id);
                v.write_json(&self.json_db_path);
                ret
            }
            None => Err("no playlist to move track in".to_string()),
        }
    }

    pub fn is_default_playlist(&self, playlist_id: u32) -> bool {
        match self.inner {
            Some(ref v) => v.is_default_playlist(playlist_id),
            None => false,
        }
    }

    pub fn get_default_playlist_id(&self) -> Option<u32> {
        match self.inner {
            Some(ref v) => Some(v.selected_playlist_id),
            None => None,
        }
    }

    pub fn entries_as_vec(&self) -> Vec<model::playlist::PlaylistEntry> {
        match self.inner {
            Some(ref v) => v.to_vec(),
            None => EMPTY_PLAYLIST_VEC,
        }
    }
}

pub struct InnerPlaylistData {
    pub current_id: u32,
    pub selected_playlist_id: u32,
    pub playlists: BTreeMap<u32, model::playlist::PlaylistEntry>,
}

impl InnerPlaylistData {
    fn new(name: String) -> Self {
        InnerPlaylistData {
            current_id: 0,
            selected_playlist_id: 1,
            playlists: vec![(
                1,
                model::playlist::PlaylistEntry {
                    id: 1,
                    name: name,
                    tracks: Vec::new(),
                },
            )]
            .into_iter()
            .collect(),
        }
    }

    fn add(&mut self, name: String) {
        let new_entry = model::playlist::PlaylistEntry {
            id: self.current_id + 1,
            name,
            tracks: Vec::new(),
        };

        self.current_id += 1;

        self.playlists.insert(new_entry.id, new_entry);
    }

    fn delete_playlist(&mut self, playlist_id: u32) -> Result<(), String> {
        if self.is_default_playlist(playlist_id) {
            return Err(format!(
                "playlist with id {} is the active playlist",
                playlist_id
            ));
        } else {
            match self.playlists.remove(&playlist_id) {
                Some(_v) => {
                    println!("removed playlist with id {}", playlist_id);
                    Ok(())
                }
                None => Err(format!("no playlist with id {}", playlist_id)),
            }
        }
    }

    fn make_playlist_default(&mut self, playlist_id: u32) {
        if self.playlists.contains_key(&playlist_id) {
            self.selected_playlist_id = playlist_id;
        }
    }

    fn is_default_playlist(&self, playlist_id: u32) -> bool {
        self.selected_playlist_id == playlist_id
    }

    pub fn add_track_to_playlist(
        &mut self,
        playlist_id: u32,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.playlists.get_mut(&playlist_id) {
            Some(ref mut playlist) => {
                playlist.tracks.push(track_id);
                Ok(())
            }
            None => Err(format!("playlist with id: {} does not exist", playlist_id)),
        }
    }

    fn remove_track_from_playlist(
        &mut self,
        playlist_id: u32,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.playlists.get_mut(&playlist_id) {
            Some(ref mut playlist) => {
                playlist.tracks.retain(|track| !(*track == track_id));
                Ok(())
            }
            None => Err(format!("playlist with id: {} does not exist", playlist_id)),
        }
    }

    pub fn move_track_in_playlist(
        &mut self,
        playlist_id: u32,
        direction: model::Direction,
        track_id: musiqlibrary::TrackUniqueIdentifier,
    ) -> Result<(), String> {
        match self.playlists.get_mut(&playlist_id) {
            Some(ref mut playlist) => {
                if playlist.tracks.contains(&track_id) {
                    let mut found_before = Vec::new();
                    let mut found_after = Vec::new();
                    let mut found = None;

                    for iter_track in playlist.tracks.iter() {
                        if *iter_track == track_id {
                            found = Some(iter_track.clone());
                        } else {
                            match found {
                                None => found_before.push(iter_track.clone()),
                                Some(_) => found_after.push(iter_track.clone()),
                            }
                        }
                    }

                    let new_playlist = match direction {
                        model::Direction::Down => {
                            let mut constructed = found_before;
                            if found_after.len() > 0 {
                                let old_first_of_after = found_after.remove(0);
                                constructed.push(old_first_of_after);
                                constructed.push(track_id);
                            } else {
                                constructed.push(track_id);
                            }
                            constructed.append(&mut found_after);
                            constructed
                        }
                        model::Direction::Up => {
                            let mut constructed = found_before;
                            match constructed.pop() {
                                Some(old_before_end) => {
                                    constructed.push(track_id);
                                    constructed.push(old_before_end);
                                }
                                None => {
                                    constructed.push(track_id);
                                }
                            }
                            constructed.append(&mut found_after);
                            constructed
                        }
                    };

                    playlist.tracks = new_playlist;
                    Ok(())
                } else {
                    Err("playlist did not contain track".to_string())
                }
            }
            None => Err(format!("playlist with id: {} does not exist", playlist_id)),
        }
    }

    fn write_json(&self, json_db_path: &PathBuf) {
        serde_json::to_writer(
            io::BufWriter::new(fs::File::create(&json_db_path).unwrap()),
            &self.to_raw(),
        )
        .unwrap()
    }

    fn to_vec(&self) -> Vec<model::playlist::PlaylistEntry> {
        self.playlists
            .iter()
            .map(|(_key, value)| value.clone())
            .collect()
    }

    fn to_raw(&self) -> RawPlaylistData {
        RawPlaylistData {
            current_id: self.current_id,
            selected_playlist_id: self.selected_playlist_id,
            playlists: self.to_vec(),
        }
    }

    fn from_raw(raw: RawPlaylistData) -> Self {
        InnerPlaylistData {
            current_id: raw.current_id,
            selected_playlist_id: raw.selected_playlist_id,
            playlists: raw
                .playlists
                .into_iter()
                .map(|value| (value.id, value.clone()))
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct RawPlaylistData {
    pub current_id: u32,
    pub selected_playlist_id: u32,
    pub playlists: Vec<model::playlist::PlaylistEntry>,
}
