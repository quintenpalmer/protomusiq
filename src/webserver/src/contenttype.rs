pub fn audio_content_type_from_ext<S: ToString>(ext: S) -> &'static str {
    match ext.to_string().as_str() {
        "flac" => "audio/flac",
        "mp3" => "audio/mp3",
        "m4a" => "audio/mp4",
        ext => panic!("unknown media file requested: {}", ext),
    }
}
