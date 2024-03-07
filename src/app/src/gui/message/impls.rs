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

impl nav::ArtistNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Artist(self))
    }
}

impl nav::ArtistViewType {
    pub fn into_message(self, artist_id: musiqlibrary::ID) -> top::Message {
        top::Message::Nav(nav::NavMessage::Artist(nav::ArtistNavMessage::ArtistView(
            artist_id, self,
        )))
    }
}

impl nav::MovieNavMessage {
    pub fn into_message(self) -> top::Message {
        top::Message::Nav(nav::NavMessage::Movie(self))
    }
}
