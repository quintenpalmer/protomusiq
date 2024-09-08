use std::fs;
use std::path;

use html5ever::tendril::TendrilSink;
use markup5ever_rcdom as rcdom;

use crate::youtube::ytmodel;

use super::parse::{convert, find};

pub fn get_library() -> musiqlibrary::RawLibrary {
    let library_path = "/home/quinten/storage/media/music/bestexisting";
    let lib_path = path::PathBuf::from(&library_path);

    let tracks = musiqlibrary::find_files(&lib_path).unwrap();

    let raw_library =
        musiqlibrary::RawLibrary::from_track_list(Some(&library_path), tracks).unwrap();

    raw_library
}

pub fn get_entries() -> Vec<ytmodel::Entry> {
    let html_bytes = fs::read_to_string("youtube/input/tidied-yt-watch-history.html").unwrap();
    let opts = html5ever::ParseOpts {
        tree_builder: html5ever::tree_builder::TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let document = html5ever::parse_document(rcdom::RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html_bytes.as_bytes())
        .unwrap();

    let simple = convert::walk_top(&document.document);

    let entries = find::find_all_entries(simple);

    println!("found these entries: {}", entries.len());

    entries
}
