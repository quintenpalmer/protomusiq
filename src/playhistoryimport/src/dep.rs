fn _translate_manual_mapping() {
    let library_path = "/home/quinten/storage/media/music/bestexisting";
    let lib_path = path::PathBuf::from(&library_path);

    let tracks = musiqlibrary::find_files(&lib_path, &lib_path).unwrap();

    let tracks_by_uniq_id: BTreeMap<
        musiqlibrary::TrackUniqueIdentifier,
        musiqlibrary::FullTrackMetadata,
    > = tracks
        .into_iter()
        .map(|track| {
            (
                musiqlibrary::TrackUniqueIdentifier::from_track(&track),
                track,
            )
        })
        .collect();

    let manual_mapping_file_reader =
        fs::File::open("jellyfin/intermediate/manual_mapping.json").unwrap();

    let existing_manual_mapping: BTreeMap<String, (musiqlibrary::TrackUniqueIdentifier, u32)> =
        serde_json::from_reader(manual_mapping_file_reader).unwrap();

    let resulting_manual_mapping: BTreeMap<String, (musiqlibrary::FullTrackMetadata, u32)> =
        existing_manual_mapping
            .into_iter()
            .map(|(j_key, (uniq_id, play_count))| {
                (
                    j_key,
                    (tracks_by_uniq_id.get(&uniq_id).unwrap().clone(), play_count),
                )
            })
            .collect();

    let manual_file = fs::File::create("jellyfin/output/debug/5_manual_mapping.json").unwrap();
    serde_json::to_writer_pretty(manual_file, &resulting_manual_mapping).unwrap();
}
