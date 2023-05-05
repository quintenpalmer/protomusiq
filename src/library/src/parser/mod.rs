mod flac;
mod generic;
mod id3;
mod mp4a;

pub use self::flac::FlacMetadataParser;
pub use self::id3::ID3MetadataParser;
pub use self::mp4a::MP4AMetadataParser;
pub use generic::{resolve_metadata_from_parser, MetadataParser};
