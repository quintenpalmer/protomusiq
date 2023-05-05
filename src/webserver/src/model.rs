use musiqlibrary;

pub use musiqlibrary::model::{
    AlbumInfo, ArtistInfo, FullTrackMetadata, RawLibrary, SortedAlbumDiscs, SortedArtistAlbums,
    SortedDiscTracks, ID,
};

pub enum PlayPriority {
    Now,
    Next,
    Append,
}
