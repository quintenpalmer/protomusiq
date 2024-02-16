use std::fs;
use std::io;

use serde_json;

use super::jmodel::ResultingInformation;

pub fn write_debug_info_to_disc(resulting_information: &ResultingInformation) {
    if resulting_information.all_zero_line_items.len() > 0 {
        println!(
            "WARN:\twriting {} length file of all zeros",
            resulting_information.all_zero_line_items.len()
        );
    } else {
        println!("INFO:\twriting 0 length file of all zeros");
    }
    let zeros_file = fs::File::create("jellyfin/output/debug/0_zeros.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(zeros_file),
        &resulting_information.all_zero_line_items,
    )
    .unwrap();

    if resulting_information.uuid_line_items.len() > 0 {
        println!(
            "WARN:\twriting {} length file of uuids",
            resulting_information.uuid_line_items.len()
        );
    } else {
        println!("INFO:\twriting 0 length file of uuids");
    }
    let uuids_file = fs::File::create("jellyfin/output/debug/1_uuids.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(uuids_file),
        &resulting_information.uuid_line_items,
    )
    .unwrap();

    println!(
        "INFO:\twriting {} length file of existing library with no new matches",
        resulting_information
            .existing_library_with_zero_new_count
            .len()
    );
    let existing_with_no_matches_file =
        fs::File::create("jellyfin/output/debug/2_existing_not_matched.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(existing_with_no_matches_file),
        &resulting_information.existing_library_with_zero_new_count,
    )
    .unwrap();

    if resulting_information.not_found.len() > 0 {
        println!(
            "ERROR:\twriting {} length file of entries not matched",
            resulting_information.not_found.len()
        );
    } else {
        println!("INFO:\twriting 0 length file of entries not matched");
    }
    let not_found_file = fs::File::create("jellyfin/output/debug/3_not_found.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(not_found_file),
        &resulting_information.not_found,
    )
    .unwrap();

    println!(
        "INFO:\twriting {} length file of manual mapping entries",
        resulting_information.manual_mapping.len()
    );
    let manual_file = fs::File::create("jellyfin/output/debug/5_manual_mapping.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(manual_file),
        &resulting_information.manual_mapping,
    )
    .unwrap();

    if resulting_information.matched_tracks_json_ready.len() > 0 {
        println!(
            "INFO:\twriting {} length file of entries matched",
            resulting_information.matched_tracks_json_ready.len()
        );
    } else {
        println!("WARN:\twriting 0 length file of entries matched");
    }
    let matched_file = fs::File::create("jellyfin/output/debug/6_matched.json").unwrap();
    serde_json::to_writer_pretty(
        io::BufWriter::new(matched_file),
        &resulting_information.matched_tracks_json_ready,
    )
    .unwrap();
}
