use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;
use std::time::SystemTime;

use claxon;

use super::generic::{trimmer, MetadataParser};

pub struct FlacMetadataParser {
    tag_map: BTreeMap<String, String>,
    stream_info: claxon::metadata::StreamInfo,
    last_mod: SystemTime,
    path: PathBuf,
}

impl FlacMetadataParser {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        /*
        println!("and another:");
        for (key, value) in tag_map.iter() {
            println!("\t{}:\t{}", key, value);
        }
        */

        let last_mod = fs::metadata(path.as_ref().clone())
            .map_err(|e| format!("{:?}", e))?
            .modified()
            .map_err(|e| format!("{:?}", e))?;

        let reader = claxon::FlacReader::open(path.as_ref().clone()).map_err(|_e| {
            format!(
                "could not load flac file: {:?}",
                path.as_ref().to_str().unwrap()
            )
        })?;

        let stream_info = reader.streaminfo();

        let tag_map = reader
            .tags()
            .map(|(k, v)| (k.to_string().to_lowercase(), v.to_string()))
            .collect::<BTreeMap<String, String>>();

        Ok(FlacMetadataParser {
            tag_map: tag_map,
            stream_info: stream_info,
            last_mod: last_mod,
            path: path.as_ref().to_path_buf(),
        })
    }
}

impl MetadataParser for FlacMetadataParser {
    fn album(&self) -> Option<String> {
        self.tag_map.get("album").map(|x| trimmer(x.to_string()))
    }

    fn album_artist(&self) -> Option<String> {
        self.tag_map
            .get("albumartist")
            .map(|x| trimmer(x.to_string()))
    }

    fn artist(&self) -> String {
        trimmer(self.tag_map.get("artist").unwrap().to_string())
    }

    fn disc(&self) -> Option<u64> {
        self.tag_map
            .get("discnumber")
            .map(|x| get_pair(x))
            .and_then(|x| get_first(x))
    }

    fn disc_total(&self) -> Option<u64> {
        match self.tag_map.get("disctotal") {
            Some(x) => Some(x.parse::<u64>()).map(|x| x.unwrap()),
            None => self
                .tag_map
                .get("discnumber")
                .map(|x| get_pair(x))
                .and_then(|x| get_second(x)),
        }
    }

    fn track(&self) -> Option<u64> {
        self.tag_map
            .get("tracknumber")
            .map(|x| get_pair(x))
            .and_then(|x| get_first(x))
    }

    fn title(&self) -> String {
        trimmer(self.tag_map.get("title").unwrap().to_string())
    }

    fn genre(&self) -> Option<String> {
        self.tag_map.get("genre").map(|x| trimmer(x.to_string()))
    }

    fn date(&self) -> Option<String> {
        match self.tag_map.get("date") {
            Some(v) => Some(v),
            None => self.tag_map.get("year"),
        }
        .map(|x| trimmer(x.to_string()))
    }

    fn duration(&self) -> time::Duration {
        let num_samples = self.stream_info.samples.unwrap();
        let sample_rate = self.stream_info.sample_rate;

        let total_duration =
            time::Duration::from_secs_f32((num_samples as f32) / (sample_rate as f32));
        total_duration
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn last_mod(&self) -> SystemTime {
        self.last_mod.clone()
    }

    fn ext(&self) -> String {
        "flac".to_string()
    }
}

fn get_pair(full_value: &str) -> Option<(u64, Option<u64>)> {
    let mut split = full_value.splitn(2, &['\0', '/'][..]);
    let a = split.next()?.parse().ok()?;
    let b = split.next().and_then(|s| s.parse().ok());
    Some((a, b))
}

fn get_first<T>(maybe_found: Option<(T, Option<T>)>) -> Option<T> {
    match maybe_found {
        Some((first, _second)) => Some(first),
        None => None,
    }
}

fn get_second<T>(maybe_found: Option<(T, Option<T>)>) -> Option<T> {
    match maybe_found {
        Some((_first, second)) => second,
        None => None,
    }
}
