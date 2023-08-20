use super::ytmodel::*;

pub fn find_all_entries(body: Vec<SimpleHtml>) -> Vec<Entry> {
    match body.as_slice() {
        &[SimpleHtml::Text(ref _z), ref div, SimpleHtml::Text(ref _y)] => {
            match is_element_with_name_and_attr(
                div,
                ElementName::Div,
                AttrName::Class,
                Some("mdl-grid"),
            ) {
                Some(mdl_children) => {
                    println!("found the top level mdl-grid");
                    match mdl_children.as_slice() {
                        &[SimpleHtml::Text(ref _z), ref all_outer_cells @ .., SimpleHtml::Text(ref _y)] =>
                        {
                            println!("found the outer cells: {}", all_outer_cells.len());
                            let mut ret_entries = Vec::new();
                            for outer_cell in all_outer_cells.iter() {
                                match parse_single_outer_cell(outer_cell) {
                                    Some(entry) => ret_entries.push(entry),
                                    None => (),
                                }
                            }
                            ret_entries
                        }
                        _ => panic!("expectd all outer cells would be surrounded by text"),
                    }
                }
                None => panic!("expected an mdl-grid div at the top"),
            }
        }
        _ => panic!("expected line break text with mdl-grid div"),
    }
}

fn parse_single_outer_cell(outer_cell: &SimpleHtml) -> Option<Entry> {
    match is_element_with_name_and_attr(
        outer_cell,
        ElementName::Div,
        AttrName::Class,
        Some("outer-cell mdl-cell mdl-cell--12-col mdl-shadow--2dp"),
    ) {
        Some(outer_children) => match outer_children.as_slice() {
            &[ref _z, ref mdl_grid, ref _y] => {
                match is_element_with_name_and_attr(
                    mdl_grid,
                    ElementName::Div,
                    AttrName::Class,
                    Some("mdl-grid"),
                ) {
                    Some(mdl_children) => {
                        let is_ad = find_is_ad_from_many(mdl_children);
                        match is_ad {
                            true => None,
                            false => find_entry_from_many(mdl_children),
                        }
                    }
                    None => panic!("assumed mdl_grid"),
                }
            }
            _ => panic!("expected mdl_grid surrounded by text"),
        },
        None => None,
    }
}

fn find_is_ad_from_many(handles: &Vec<SimpleHtml>) -> bool {
    for child in handles.iter() {
        match is_element_with_name_and_attr(
            child,
            ElementName::Div,
            AttrName::Class,
            Some("content-cell mdl-cell mdl-cell--12-col mdl-typography--caption"),
        ) {
            Some(ad_children) => return find_is_ad_from_ad(ad_children),
            None => (),
        }
    }
    panic!("could not find caption section");
}

fn find_is_ad_from_ad(handles: &Vec<SimpleHtml>) -> bool {
    match handles.as_slice() {
        &[ref _z, ref _b1, ref _br1, ref _yt, ref _br2, ref _y, ref _b2, ref _br_why, ref _because, ref _a1, ref _final] => {
            false
        }
        &[ref _z, ref _b1, ref _br1, ref _yt, ref _br2, ref _y, ref _details, ref _br3, ref google_ads, ref _br4, ref _x, ref _b_why, ref _br5, ref _because, ref _a1, ref _final] => {
            match is_text_that_contains(google_ads, "From Google Ads") {
                true => true,
                false => panic!("somehow found details that didn't say google ads"),
            }
        }
        _ => panic!("caption section of unexpected format"),
    }
}

fn find_entry_from_many(handles: &Vec<SimpleHtml>) -> Option<Entry> {
    for child in handles.iter() {
        match is_element_with_name_and_attr(
            child,
            ElementName::Div,
            AttrName::Class,
            Some("content-cell mdl-cell mdl-cell--6-col mdl-typography--body-1"),
        ) {
            Some(content_children) => return find_entry_from_content(content_children),
            None => (),
        }
    }
    panic!("could not find an entry");
}

fn find_entry_from_content(handles: &Vec<SimpleHtml>) -> Option<Entry> {
    match handles.as_slice() {
        &[ref _z, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs,
            children: ref _children,
        }), ref _y] => None,
        &[ref _z, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs,
            children: ref _children,
        }), ref _y, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs1,
            children: ref _children1,
        }), ref _x, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs2,
            children: ref _children2,
        }), ref _w] => None,
        &[ref _z, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs,
            children: ref _children,
        }), ref _y, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs1,
            children: ref _children1,
        }), ref _x, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs2,
            children: ref _children2,
        }), ref _w, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs3,
            children: ref _children3,
        }), ref _v] => None,
        &[ref _z, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs,
            children: ref _children,
        }), ref _y, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs1,
            children: ref _children1,
        }), ref _x, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs2,
            children: ref _children2,
        }), ref _w, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs3,
            children: ref _children3,
        }), ref _v, SimpleHtml::Element(Element {
            name: ElementName::Br,
            attrs: ref _attrs4,
            children: ref _children4,
        }), ref _u] => None,
        &[ref _watched, ref title_a, ref _br, ref _y, ref artist_a, ref rest @ ..] => {
            let title = match is_element_with_name_and_attr(
                title_a,
                ElementName::A,
                AttrName::Href,
                None,
            ) {
                Some(contents_text_as_vec) => match contents_text_as_vec.as_slice() {
                    &[SimpleHtml::Text(ref contents)] => contents.clone(),
                    _ => panic!("expected a single text in the a href for the title"),
                },
                None => panic!("expected a single a href for the title"),
            };
            let artist =
                match is_element_with_name_and_attr(artist_a, ElementName::A, AttrName::Href, None)
                {
                    Some(contents_text_as_vec) => match contents_text_as_vec.as_slice() {
                        &[SimpleHtml::Text(ref contents)] => contents.clone(),
                        _ => panic!("expected a single text in the a href for the artist"),
                    },
                    None => panic!("expected a single a href for the artist"),
                };
            let watched = parse_watch_info(rest);
            Some(Entry {
                title,
                artist,
                watched,
            })
        }
        &[ref _watched, ref title_a, ref _br, ref _y, ..] => {
            let _title = match is_element_with_name_and_attr(
                title_a,
                ElementName::A,
                AttrName::Href,
                None,
            ) {
                Some(contents_text_as_vec) => match contents_text_as_vec.as_slice() {
                    &[SimpleHtml::Text(ref contents)] => contents.clone(),
                    _ => panic!("expected a single text in the a href for the title"),
                },
                None => panic!("expected a single a href for the title"),
            };
            None
        }
        _ => panic!("entry did not match expected scheme"),
    }
}

fn parse_watch_info(nodes: &[SimpleHtml]) -> String {
    match nodes {
        &[ref _br, ref text_element] => match extract_text(text_element) {
            Some(t) => t,
            None => panic!("was not text as expected"),
        },
        &[ref _br1, ref _watched_at_text, ref _br2, ref text_element] => {
            match extract_text(text_element) {
                Some(t) => t,
                None => panic!("was not text as expected"),
            }
        }
        _ => panic!("unexpected watch info format"),
    }
}

fn extract_text(node: &SimpleHtml) -> Option<String> {
    match node {
        SimpleHtml::Text(ref contents) => Some(contents.clone()),
        _ => None,
    }
}

fn is_text_that_contains(node: &SimpleHtml, expected_text: &'static str) -> bool {
    match node {
        SimpleHtml::Text(found_text) => found_text.contains(expected_text),
        _ => false,
    }
}

fn is_element_with_name_and_attr<'a>(
    node: &'a SimpleHtml,
    expected_element_name: ElementName,
    expected_attr_name: AttrName,
    maybe_expected_attr_value: Option<&'static str>,
) -> Option<&'a Vec<SimpleHtml>> {
    match node {
        SimpleHtml::Element(found_element) => {
            if found_element.name == expected_element_name {
                for (found_attr_name, found_attr_value) in found_element.attrs.iter() {
                    if *found_attr_name == expected_attr_name {
                        match maybe_expected_attr_value {
                            Some(expected_attr_value) => {
                                if found_attr_value == expected_attr_value {
                                    return Some(&found_element.children);
                                } else {
                                    return None;
                                }
                            }
                            None => return Some(&found_element.children),
                        }
                    }
                }
                None
            } else {
                None
            }
        }
        _ => None,
    }
}
