use std::fs;

use serde_json;

use super::gmodel;

pub fn write_debug_info_to_disc(resulting_information: &gmodel::BestEffortMatchedInformation) {
    // Zeroes
    if resulting_information.all_zero_line_items.len() > 0 {
        println!(
            "WARN:\twriting {} length file of all zeros",
            resulting_information.all_zero_line_items.len()
        );
    } else {
        println!("INFO:\twriting 0 length file of all zeros");
    }
    let zeros_file = fs::File::create("gplaymusic/output/debug/0_zeros.json").unwrap();
    serde_json::to_writer_pretty(zeros_file, &resulting_information.all_zero_line_items).unwrap();

    // Not Found
    if resulting_information.not_found.len() > 0 {
        println!(
            "ERROR:\twriting {} length file of entries not matched",
            resulting_information.not_found.len()
        );
    } else {
        println!("INFO:\twriting 0 length file of entries not matched");
    }
    let not_found_file = fs::File::create("gplaymusic/output/debug/1_not_found.json").unwrap();
    serde_json::to_writer_pretty(not_found_file, &resulting_information.not_found).unwrap();

    // No New Matches
    println!(
        "INFO:\twriting {} length file of existing library with no new matches",
        resulting_information
            .existing_library_with_zero_new_count
            .len()
    );
    let existing_with_no_matches_file =
        fs::File::create("gplaymusic/output/debug/2_existing_not_matched.json").unwrap();
    serde_json::to_writer_pretty(
        existing_with_no_matches_file,
        &resulting_information.existing_library_with_zero_new_count,
    )
    .unwrap();

    // Manual Track Mapping
    println!(
        "INFO:\twriting {} length file of manual track mapping entries",
        resulting_information.manual_track_mapping.len()
    );

    let manual_mapping_vec: Vec<(
        gmodel::GPlayMusicKey,
        (musiqlibrary::FullTrackMetadata, u32),
    )> = resulting_information
        .manual_track_mapping
        .iter()
        .map(|(key, (full, play_count))| (key.clone(), (full.clone(), play_count.clone())))
        .collect();

    let manual_track_file =
        fs::File::create("gplaymusic/output/debug/3_manual_track_mapping.json").unwrap();
    serde_json::to_writer_pretty(manual_track_file, &manual_mapping_vec).unwrap();

    // Manual Artist Mapping
    println!(
        "INFO:\twriting {} length file of manual artist mapping entries",
        resulting_information.manual_artist_mapping.len()
    );

    let manual_artist_file =
        fs::File::create("gplaymusic/output/debug/4_manual_artist_mapping.json").unwrap();
    serde_json::to_writer_pretty(
        manual_artist_file,
        &resulting_information.manual_artist_mapping,
    )
    .unwrap();

    // Manual Album Mapping
    println!(
        "INFO:\twriting {} length file of manual album mapping entries",
        resulting_information.manual_album_mapping.len()
    );

    let manual_album_mapping_vec: Vec<((String, String), musiqlibrary::ArtistAlbumInfo)> =
        resulting_information
            .manual_album_mapping
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

    let manual_track_file =
        fs::File::create("gplaymusic/output/debug/5_manual_album_mapping.json").unwrap();
    serde_json::to_writer_pretty(manual_track_file, &manual_album_mapping_vec).unwrap();

    // Ignore Albums
    println!(
        "INFO:\twriting {} length file of ignore album entries",
        resulting_information.manual_ignore_albums.len()
    );
    let manual_album_mapping_vec: Vec<(String, String)> = resulting_information
        .manual_ignore_albums
        .iter()
        .map(|key| key.clone())
        .collect();

    let manual_track_file =
        fs::File::create("gplaymusic/output/debug/6_manual_album_ignore.json").unwrap();
    serde_json::to_writer_pretty(manual_track_file, &manual_album_mapping_vec).unwrap();

    // Ignore Albums Map
    println!(
        "INFO:\twriting {} length file of ignore album map entries",
        resulting_information
            .ignore_album_info_with_play_counts
            .len()
    );
    let mut ignore_album_info_with_play_counts_vec: Vec<((String, String), u32)> =
        resulting_information
            .ignore_album_info_with_play_counts
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

    ignore_album_info_with_play_counts_vec.sort_by_key(|a| a.1);
    ignore_album_info_with_play_counts_vec.reverse();

    let ignore_album_info_with_play_counts_file =
        fs::File::create("gplaymusic/output/debug/7_manual_album_ignore_map.json").unwrap();
    serde_json::to_writer_pretty(
        ignore_album_info_with_play_counts_file,
        &ignore_album_info_with_play_counts_vec,
    )
    .unwrap();

    // Matched
    if resulting_information.matched_tracks_json_ready.len() > 0 {
        println!(
            "INFO:\twriting {} length file of entries matched",
            resulting_information.matched_tracks_json_ready.len()
        );
    } else {
        println!("WARN:\twriting 0 length file of entries matched");
    }
    let matched_file = fs::File::create("gplaymusic/output/debug/8_matched.json").unwrap();
    serde_json::to_writer_pretty(
        matched_file,
        &resulting_information.matched_tracks_json_ready,
    )
    .unwrap();
}
