use crate::model;

use crate::gui::message::{self, user_nav_message};

pub fn track_link(track: &musiqlibrary::FullTrackMetadata) -> message::Message {
    user_nav_message(message::NavMessage::ArtistAlbumView(
        track.album_artist_id,
        track.album_id,
        model::AlbumSize::Regular,
        Some(musiqlibrary::TrackUniqueIdentifier::from_track(track)),
    ))
}
