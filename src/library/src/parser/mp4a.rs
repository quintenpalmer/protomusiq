use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use super::generic::MetadataParser;

pub struct MP4AMetadataParser {
    tag: mp4ameta::Tag,
    last_mod: time::SystemTime,
    path: PathBuf,
}

impl MP4AMetadataParser {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let last_mod = fs::metadata(path.as_ref())
            .map_err(|e| format!("{:?}", e))?
            .modified()
            .map_err(|e| format!("{:?}", e))?;

        let tag = mp4ameta::Tag::read_from_path(path.as_ref()).expect(
            format!(
                "could not load m4a file: {}",
                path.as_ref().to_str().unwrap()
            )
            .as_str(),
        );
        Ok(MP4AMetadataParser {
            tag,
            last_mod,
            path: path.as_ref().to_path_buf(),
        })
    }
}

impl MetadataParser for MP4AMetadataParser {
    fn album(&self) -> Option<String> {
        self.tag.album().map(|x| x.to_string())
    }

    fn album_artist(&self) -> Option<String> {
        self.tag.album_artist().map(|x| x.to_string())
    }

    fn artist(&self) -> String {
        self.tag.artist().unwrap().to_string()
    }

    fn disc(&self) -> Option<u64> {
        self.tag.disc_number().map(|x| x as u64)
    }

    fn disc_total(&self) -> Option<u64> {
        self.tag.total_discs().map(|x| x as u64)
    }

    fn track(&self) -> Option<u64> {
        self.tag.track_number().map(|x| x as u64)
    }

    fn title(&self) -> String {
        self.tag.title().unwrap().to_string()
    }

    fn genre(&self) -> Option<String> {
        self.tag.genre().map(|x| x.to_string())
    }

    fn date(&self) -> Option<String> {
        self.tag.year().map(|x| x.to_string())
    }

    fn duration(&self) -> time::Duration {
        self.tag.duration().unwrap()
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn last_mod(&self) -> time::SystemTime {
        self.last_mod
    }

    fn ext(&self) -> String {
        "m4a".to_string()
    }
}
