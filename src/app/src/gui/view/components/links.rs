use crate::model;

use crate::gui::message;

pub fn track_link(track: &musiqlibrary::FullTrackMetadata) -> message::Message {
    message::ArtistAlbumView::ArtistAlbumTrackView(
        model::AlbumSize::Regular,
        Some(musiqlibrary::TrackUniqueIdentifier::from_track(track)),
        None,
    )
    .into_message(track.album_artist_id, track.album_id)
}
