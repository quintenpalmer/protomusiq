use super::{nav, top};

impl nav::NavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(self)
    }
}

impl nav::NavRelMsg {
    pub fn into_message(self) -> top::Message {
        top::Message::NavRelative(self)
    }
}

impl nav::PlaylistNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Playlist(self))
    }
}

impl nav::MusicNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Music(self))
    }
}

impl nav::MusicGenreNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Music(nav::MusicNavMessage::Genres(self)))
    }
}

impl nav::ArtistNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Music(nav::MusicNavMessage::Artist(self)))
    }
}

impl nav::ArtistViewType {
    pub fn into_message(self, artist_id: musiqlibrary::ID) -> top::Message {
        top::Message::Nav(nav::NavMessage::Music(nav::MusicNavMessage::Artist(
            nav::ArtistNavMessage::ArtistView(artist_id, self),
        )))
    }
}

impl nav::ArtistAlbumView {
    pub fn into_message(
        self,
        artist_id: musiqlibrary::ID,
        album_id: musiqlibrary::ID,
    ) -> top::Message {
        top::Message::Nav(nav::NavMessage::Music(nav::MusicNavMessage::Artist(
            nav::ArtistNavMessage::AlbumView(artist_id, album_id, self),
        )))
    }
}

impl nav::MovieNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Movie(self))
    }
}
