use std::collections::BTreeMap;

use html5ever;
use markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;

pub fn debug_display_tree(html_bytes: &str) {
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

    let mut title_map = BTreeMap::new();

    for entry in find_videos_from_top(&document.document).iter() {
        let watched = title_map.entry(entry.title.clone()).or_insert(Vec::new());
        watched.push((entry.watched_at.clone(), entry.other_watched_ats.clone()));
    }

    let mut sorted_by_views = title_map
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();

    sorted_by_views.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    for (title, views) in sorted_by_views.iter() {
        println!("title: {}", title);
        for (view, other_views) in views.iter() {
            println!("    {}", view);
            for other_watched_at in other_views.iter() {
                println!("    {}", other_watched_at);
            }
        }
    }
}

struct Entry {
    pub title: String,
    pub watched_at: String,
    pub other_watched_ats: Vec<String>,
}

fn find_videos_from_top(handle: &rcdom::Handle) -> Vec<Entry> {
    let mut entries = Vec::new();
    let node = handle;
    if is_element_with_class(
        handle,
        "div",
        "class",
        Some("outer-cell mdl-cell mdl-cell--12-col mdl-shadow--2dp"),
    ) {
        match find_non_ad_video(node) {
            Some(v) => entries.push(v),
            None => (),
        }
    }

    for child in node.children.borrow().iter() {
        let mut more_entries = find_videos_from_top(child);
        entries.append(&mut more_entries);
    }
    entries
}

fn find_non_ad_video(handle: &rcdom::Handle) -> Option<Entry> {
    walk(4, handle);
    if is_element_with_class(
        handle,
        "div",
        "class",
        Some("outer-cell mdl-cell mdl-cell--12-col mdl-shadow--2dp"),
    ) {
        match handle.children.borrow().iter().as_slice() {
            &[ref _linebreak1, ref single_element, ref _linebreak2] => {
                if is_element_with_class(single_element, "div", "class", Some("mdl-grid")) {
                    match &single_element.children.borrow().iter().as_slice() {
                        &[ref _linebreak1, ref _header, ref _linebreak2, ref contents, ref _linebreak3, ref _right_space, ref _linebreak4, ref caption, ref _linebreak5] =>
                        {
                            let watch_info = get_watch_info(contents);
                            let is_ad = get_is_ad(caption);
                            if !is_ad {
                                Some(watch_info)
                            } else {
                                None
                            }
                        }
                        _ => panic!("why don't we see the watch info and ad info as expected"),
                    }
                } else {
                    panic!("why wasn't the single element we saw an mdl-grid div");
                }
            }
            _ => panic!("why didn't we see a single div mdl-grid element"),
        }
    } else {
        panic!("why wasn't it the content class div");
    }
}

fn get_watch_info(handle: &rcdom::Handle) -> Entry {
    if is_element_with_class(
        handle,
        "div",
        "class",
        Some("content-cell mdl-cell mdl-cell--6-col mdl-typography--body-1"),
    ) {
        match handle.children.borrow().iter().as_slice() {
            &[ref _watched_text, ref a_href, ref _br1, ref watched_info @ ..] => {
                if is_element_with_class(a_href, "a", "href", None) {
                    Entry {
                        title: "title".to_string(),
                        watched_at: "watched at".to_string(),
                        other_watched_ats: vec!["other watched ats".to_string()],
                    }
                } else {
                    panic!("should be an a href");
                }
            }
            _ => panic!("should always be the content "),
        }
    } else {
        panic!("should always have the content class");
    }
}

fn get_is_ad(handle: &rcdom::Handle) -> bool {
    if is_element_with_class(
        handle,
        "div",
        "class",
        Some("content-cell mdl-cell mdl-cell--12-col mdl-typography--caption"),
    ) {
        match handle.children.borrow().iter().as_slice() {
            &[ref _linebreak1, ref _bold_products, ref _br1, ref _youtube, ref _br2, ref _linebreak2, ref _bold_details, ref _br3, ref from_google_ads, ref _br4, ref _linebreak3, ref _bold_why, ref _br5, ref _this_activity, ref _ahref, ref _closing_period] => {
                if is_text_that_reads(from_google_ads, "From Google Ads") {
                    true
                } else {
                    panic!("if we saw this many elements, it should always be the google ads disclaimer");
                }
            }
            &[ref _linebreak1, ref _bold_products, ref _br1, ref _youtube, ref _br2, ref _linebreak2, ref _bold_why, ref _br5, ref _this_activity, ref _ahref, ref _closing_period] => {
                false
            }
            _ => panic!("should always be the ad structure"),
        }
    } else {
        panic!("should always have the ad class");
    }
}

fn is_element_with_class(
    handle: &rcdom::Handle,
    requested_name: &'static str,
    requested_attr_name: &'static str,
    maybe_requested_class: Option<&'static str>,
) -> bool {
    match &handle.data {
        rcdom::NodeData::Element { name, attrs, .. } => {
            assert!(name.ns == ns!(html));
            if name.local.to_string() == requested_name {
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    //print!(" {}=\"{}\"", attr.name.local, attr.value);
                    match maybe_requested_class {
                        Some(requested_class) => {
                            if attr.name.local.to_string() == requested_attr_name
                                && attr.value.to_string() == requested_class
                            {
                                return true;
                            }
                        }
                        None => {
                            if attr.name.local.to_string() == requested_attr_name {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }

    return false;
}

fn is_text_that_reads(handle: &rcdom::Handle, expected_contents: &'static str) -> bool {
    match &handle.data {
        rcdom::NodeData::Text {
            contents: raw_contents,
        } => {
            let mut contents = raw_contents.borrow().to_string();
            contents.retain(|x| x != '\n');
            contents = contents.trim().to_string();
            if contents.contains(expected_contents) {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn find_video_and_is_ad(handle: &rcdom::Handle) -> Option<Entry> {
    let mut entry = None;
    let mut is_ad = false;
    let node = handle;
    match &node.data {
        rcdom::NodeData::Element { name, attrs, .. } => {
            assert!(name.ns == ns!(html));
            //print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                //print!(" {}=\"{}\"", attr.name.local, attr.value);
                if attr.name.local.to_string() == "class"
                    && attr.value.to_string()
                        == "content-cell mdl-cell mdl-cell--6-col mdl-typography--body-1"
                {
                    match find_videos_from_entry(node) {
                        Some(v) => entry = Some(v),
                        None => (),
                    }
                }
                if attr.name.local.to_string() == "class"
                    && attr.value.to_string()
                        == "content-cell mdl-cell mdl-cell--12-col mdl-typography--caption"
                {
                    walk(4, &node);
                    if find_is_ad(node) {
                        is_ad = true;
                    }
                }
            }
        }
        rcdom::NodeData::ProcessingInstruction { .. } => unreachable!(),
        _ => (),
    }

    match entry {
        Some(v) => {
            if !is_ad {
                return Some(v);
            }
        }
        None => {
            for child in node.children.borrow().iter() {
                match find_video_and_is_ad(child) {
                    Some(v) => return Some(v),
                    _ => (),
                };
            }
        }
    }
    return None;
}

fn find_is_ad(handle: &rcdom::Handle) -> bool {
    println!("will this be an ad?");
    for node in handle.children.borrow().iter() {
        println!("node: {:?}", node);
        match &node.data {
            rcdom::NodeData::Text {
                contents: raw_contents,
            } => {
                println!("it is text... {}", raw_contents.borrow().to_string());
                let mut contents = raw_contents.borrow().to_string();
                contents.retain(|x| x != '\n');
                contents = contents.trim().to_string();
                if contents.contains("From Google Ads") {
                    println!("it is an ad");
                    return true;
                }
            }
            rcdom::NodeData::Comment { contents } => {
                println!("Comment: <!-- {} -->", contents.escape_default())
            }
            rcdom::NodeData::Element { name, .. } => {
                assert!(name.ns == ns!(html));
                println!("it was an element: {}", name.local);
                for subnode in node.children.borrow().iter() {
                    match &subnode.data {
                        rcdom::NodeData::Element { name, .. } => {
                            assert!(name.ns == ns!(html));
                            println!("it was a subelement: {}", name.local);
                        }
                        rcdom::NodeData::Text { contents } => {
                            println!("it is subtext... {}", contents.borrow().to_string());
                        }
                        rcdom::NodeData::Comment { contents } => {
                            println!("Subcomment: <!-- {} -->", contents.escape_default())
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    println!("assumed an ad");
    return false;
}

fn find_videos_from_entry(handle: &rcdom::Handle) -> Option<Entry> {
    //println!("found entry, let's extract the info");
    let mut video_name = None;
    let mut watched_at = None;
    let mut other_watched_ats = Vec::new();
    for node in handle.children.borrow().iter() {
        match &node.data {
            rcdom::NodeData::Element { name, .. } => {
                assert!(name.ns == ns!(html));
                //println!("name: {}", name.local);
                if name.local.to_string() == "a" {
                    match node
                        .children
                        .borrow()
                        .iter()
                        .map(|x| &x.data)
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        &[rcdom::NodeData::Text {
                            contents: raw_contents,
                        }] => {
                            let mut contents = raw_contents.borrow().to_string();
                            contents.retain(|x| x != '\n');
                            contents = contents.trim().to_string();
                            if video_name.is_none() {
                                video_name = Some(contents)
                            }
                        }
                        _ => println!("some other not-just-text"),
                    }
                }
            }
            rcdom::NodeData::Text {
                contents: raw_contents,
            } => {
                let mut contents = raw_contents.borrow().to_string();
                contents.retain(|x| x != '\n');
                contents = contents.trim().to_string();
                if contents.starts_with("Watched at") {
                    other_watched_ats.push(contents);
                } else {
                    watched_at = Some(contents);
                }
            }
            _ => (),
        }
    }

    match (video_name, watched_at) {
        (Some(v), Some(w)) => Some(Entry {
            title: v,
            watched_at: w,
            other_watched_ats,
        }),
        _ => None,
    }
}

#[allow(unused)]
pub fn walk(indent: usize, handle: &rcdom::Handle) {
    let node = handle;
    for _ in 0..indent {
        print!(" ");
    }
    match &node.data {
        rcdom::NodeData::Document => println!("#Document"),

        rcdom::NodeData::Doctype {
            name,
            public_id,
            system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        rcdom::NodeData::Text { contents } => {
            println!("#text: {}", contents.borrow().escape_default())
        }

        rcdom::NodeData::Comment { contents } => {
            println!("<!-- {} -->", contents.escape_default())
        }

        rcdom::NodeData::Element { name, attrs, .. } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        rcdom::NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child);
    }
}
