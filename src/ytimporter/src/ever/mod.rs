use std::collections::BTreeMap;

mod convert;
mod find;
mod model;

use markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;

pub fn find_all_watch_info(html_bytes: &str) {
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

    let mut entry_title_map = BTreeMap::new();

    for entry in entries.into_iter() {
        let watched_vec = entry_title_map
            .entry((entry.title, entry.artist))
            .or_insert(Vec::new());
        watched_vec.push(entry.watched);
    }

    entry_title_map = entry_title_map
        .into_iter()
        .filter(|(_title, watch_info)| watch_info.len() > 1)
        .collect();

    let mut entry_by_title_sorted: Vec<(_, _)> = entry_title_map.into_iter().collect();

    entry_by_title_sorted.sort_by_key(|(_title, watch_info)| watch_info.len());

    for ((title, artist), watch_info) in entry_by_title_sorted.iter() {
        println!(
            "({} - {}) views: {}, artist: {}, title: {}",
            watch_info.first().unwrap(),
            watch_info.last().unwrap(),
            watch_info.len(),
            artist,
            title
        );
    }

    println!(
        "that was these with more than one view: {}",
        entry_by_title_sorted.len()
    );
}
