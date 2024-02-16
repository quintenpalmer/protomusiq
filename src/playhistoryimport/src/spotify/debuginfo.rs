use std::fs;
use std::io;

use serde_json;

use super::smodel;

pub fn write_debug_info_to_disc(resulting_information: &smodel::DebugInfo) {
    // Track Counts
    {
        println!(
            "INFO:\twriting {} length file of all tracks with play count",
            resulting_information.play_info_lines_count.len()
        );

        let sort_file = fs::File::create("spotify/output/debug/10_sorted.json").unwrap();
        serde_json::to_writer_pretty(
            io::BufWriter::new(sort_file),
            &resulting_information.play_info_lines_count,
        )
        .unwrap();
    }

    // Not Found
    {
        if resulting_information.not_found.len() > 0 {
            println!(
                "ERROR:\twriting {} length file of entries not matched",
                resulting_information.not_found.len()
            );
        } else {
            println!("INFO:\twriting 0 length file of entries not matched");
        }
        let mut not_found_vec: Vec<(smodel::SpotifyKey, (Vec<smodel::CleanedLineItem>, usize))> =
            resulting_information
                .not_found
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone(), value.len())))
                .collect();

        not_found_vec.sort_by_key(|a| a.1 .1);
        not_found_vec.reverse();

        let not_found_file = fs::File::create("spotify/output/debug/0_not_found.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(not_found_file), &not_found_vec).unwrap();
    }

    // Not Found Albums
    {
        println!(
            "INFO:\twriting {} length file of entries not matched",
            resulting_information.not_found_albums.len()
        );
        let mut not_found_vec: Vec<((String, String), usize)> = resulting_information
            .not_found_albums
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        not_found_vec.sort_by_key(|a| a.1);
        not_found_vec.reverse();

        let not_found_file = fs::File::create("spotify/output/debug/1_not_found.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(not_found_file), &not_found_vec).unwrap();
    }

    // Keyed Library
    {
        println!(
            "INFO:\twriting {} length file of all library tracks, keyed",
            resulting_information.keyed_library_items.len()
        );

        let keyed_vec: Vec<(smodel::SpotifyKey, musiqlibrary::FullTrackMetadata)> =
            resulting_information
                .keyed_library_items
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect();

        let sort_file = fs::File::create("spotify/output/debug/2_keyed_library.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(sort_file), &keyed_vec).unwrap();
    }

    // Manual Track Mapping
    {
        println!(
            "INFO:\twriting {} length file of track maps",
            resulting_information.manual_track_mapping.len()
        );

        let keyed_vec: Vec<_> = resulting_information
            .manual_track_mapping
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let sort_file =
            fs::File::create("spotify/output/debug/3_manual_track_mapping.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(sort_file), &keyed_vec).unwrap();
    }

    // Manual Album Mapping
    {
        println!(
            "INFO:\twriting {} length file of album maps",
            resulting_information.manual_album_mapping.len()
        );

        let keyed_vec: Vec<_> = resulting_information
            .manual_album_mapping
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let sort_file =
            fs::File::create("spotify/output/debug/4_manual_album_mapping.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(sort_file), &keyed_vec).unwrap();
    }

    // Manual Album Ingore
    {
        println!(
            "INFO:\twriting {} length file of album to ignore",
            resulting_information.manual_ignore_albums.len()
        );

        let keyed_vec: Vec<_> = resulting_information
            .manual_ignore_albums
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        let sort_file =
            fs::File::create("spotify/output/debug/5_manual_album_ignore.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(sort_file), &keyed_vec).unwrap();
    }

    // Matched
    {
        println!(
            "INFO:\twriting {} length file of matched entries",
            resulting_information.found_keys_in_library_matches.len()
        );
        let mut matched_vec: Vec<(
            smodel::SpotifyKey,
            (
                musiqlibrary::FullTrackMetadata,
                Vec<smodel::CleanedLineItem>,
                usize,
            ),
        )> = resulting_information
            .found_keys_in_library_matches
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    (value.0.clone(), value.1.clone(), value.1.len()),
                )
            })
            .collect();

        matched_vec.sort_by_key(|a| a.1 .2);
        matched_vec.reverse();

        let matched_json_file = fs::File::create("spotify/output/debug/7_matched.json").unwrap();
        serde_json::to_writer_pretty(io::BufWriter::new(matched_json_file), &matched_vec).unwrap();
    }
}
