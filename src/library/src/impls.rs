use std::path;

use crate::model::{Error, FullTrackMetadata, RawLibrary};
use crate::organizer;
use crate::scanner;

impl RawLibrary {
    pub fn new<P: AsRef<path::Path>>(scan_prefix: P) -> Result<Self, Error> {
        let tracks = scanner::find_files(&scan_prefix).map_err(Error::IO)?;

        RawLibrary::from_track_list(Some(scan_prefix), tracks)
    }

    pub fn from_track_list<P: AsRef<path::Path>>(
        scan_prefix: Option<P>,
        tracks: Vec<FullTrackMetadata>,
    ) -> Result<Self, Error> {
        let (tree, conflicts) = organizer::organize_tracks(tracks);
        if !conflicts.is_empty() {
            Err(Error::Conflicts(conflicts))
        } else {
            Ok(RawLibrary {
                scan_prefix: scan_prefix.map(|x| x.as_ref().to_path_buf()),
                artists: tree,
            })
        }
    }
}
