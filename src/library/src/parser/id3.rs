use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use id3::{self, TagLike};
use mp3_duration;

use super::generic::{trimmer, MetadataParser};

pub struct ID3MetadataParser {
    tag: id3::Tag,
    duration: time::Duration,
    last_mod: time::SystemTime,
    path: PathBuf,
}

impl ID3MetadataParser {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let last_mod = fs::metadata(path.as_ref())
            .map_err(|e| format!("{:?}", e))?
            .modified()
            .map_err(|e| format!("{:?}", e))?;

        let tag = id3::Tag::read_from_path(path.as_ref()).expect(
            format!(
                "could not load mp3 file: {}",
                path.as_ref().to_str().unwrap()
            )
            .as_str(),
        );

        let duration = mp3_duration::from_path(path.as_ref()).unwrap();

        /*
        println!("and another:");
        for frame in tag.frames() {
            println!("\t{}:\t{}", frame.id(), frame.content());
        }
        */

        Ok(ID3MetadataParser {
            tag: tag,
            duration: duration,
            last_mod: last_mod,
            path: path.as_ref().to_path_buf(),
        })
    }
}

impl MetadataParser for ID3MetadataParser {
    fn album(&self) -> Option<String> {
        self.tag.album().map(|x| trimmer(x.to_string()))
    }

    fn album_artist(&self) -> Option<String> {
        self.tag.album_artist().map(|x| trimmer(x.to_string()))
    }

    fn artist(&self) -> String {
        trimmer(self.tag.artist().unwrap().to_string())
    }

    fn disc(&self) -> Option<u64> {
        self.tag.disc().map(|x| x as u64)
    }

    fn disc_total(&self) -> Option<u64> {
        self.tag.total_discs().map(|x| x as u64).or_else(|| {
            let mut found = None;
            for frame in self.tag.frames() {
                if frame.id() == "TXXX" {
                    match frame.content().extended_text() {
                        Some(extended_text) => {
                            if extended_text.description == "DISCTOTAL" {
                                match extended_text
                                    .value
                                    .trim()
                                    .trim_matches(char::from(0))
                                    .parse::<u64>()
                                {
                                    Ok(disctotal) => found = Some(disctotal),
                                    Err(_e) => (),
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
            found
        })
    }

    fn track(&self) -> Option<u64> {
        self.tag.track().map(|x| x as u64).or_else(|| {
            self.tag
                .get("TRCK")
                .and_then(|x| x.content().text())
                .map(|x| x.trim())
                .map(|x| x.trim_matches(char::from(0)))
                .map(|x| x.parse::<u64>())
                .map(|x| x.unwrap())
        })
    }

    fn title(&self) -> String {
        trimmer(self.tag.title().unwrap().to_string())
    }

    fn genre(&self) -> Option<String> {
        self.tag.genre().map(|x| trimmer(x.to_string()))
    }

    fn date(&self) -> Option<String> {
        match self.tag.year() {
            Some(v) => Some(v.to_string()),
            None => match self.tag.date_released() {
                Some(v) => Some(v.to_string()),
                None => match self.tag.date_recorded() {
                    Some(v) => Some(v.to_string()),
                    None => None,
                },
            },
        }
        .map(|x| trimmer(x))
    }

    fn duration(&self) -> time::Duration {
        self.duration
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn last_mod(&self) -> time::SystemTime {
        self.last_mod.clone()
    }

    fn ext(&self) -> String {
        "mp3".to_string()
    }
}
