use std::collections::BTreeMap;

use markup5ever_rcdom as rcdom;

use super::model::*;

pub fn walk_top(handle: &rcdom::Handle) -> Vec<SimpleHtml> {
    let node = handle;
    match &node.data {
        rcdom::NodeData::Document => match node.children.borrow().as_slice() {
            &[ref html_child] => walk_html(html_child),
            _ => panic!("should have just been html under document"),
        },
        _ => panic!("should only be document at top"),
    }
}

fn walk_html(handle: &rcdom::Handle) -> Vec<SimpleHtml> {
    let node = handle;
    match &node.data {
        rcdom::NodeData::Element { name, .. } => {
            assert!(name.ns == ns!(html));
            for child in node.children.borrow().iter() {
                match walk_maybe_body(child) {
                    Some(v) => return v,
                    None => (),
                }
            }
        }
        _ => panic!("should just be an html element under the document"),
    };
    return Vec::new();
}

fn walk_maybe_body(handle: &rcdom::Handle) -> Option<Vec<SimpleHtml>> {
    let node = handle;
    match &node.data {
        rcdom::NodeData::Element { name, .. } => {
            assert!(name.ns == ns!(html));
            if name.local.to_string() == "body" {
                let mut ret = Vec::new();
                for child in node.children.borrow().iter() {
                    match walk_body(child) {
                        Some(v) => ret.push(v),
                        None => (),
                    }
                }
                return Some(ret);
            } else {
                ()
            }
        }
        _ => (),
    };
    return None;
}

pub fn walk_body(handle: &rcdom::Handle) -> Option<SimpleHtml> {
    let node = handle;
    match &node.data {
        rcdom::NodeData::Document => panic!("shouldn't see a document down here"),
        rcdom::NodeData::Doctype { .. } => panic!("shouldn't see a doctype down here"),
        rcdom::NodeData::Comment { .. } => None,

        rcdom::NodeData::Text {
            contents: raw_contents,
        } => {
            let mut contents = raw_contents.borrow().to_string();
            contents.retain(|x| x != '\n');
            contents = contents.trim().to_string();
            Some(SimpleHtml::Text(contents))
        }

        rcdom::NodeData::Element { name, attrs, .. } => {
            assert!(name.ns == ns!(html));
            let element_name = match name.local.to_string().as_str() {
                "div" => ElementName::Div,
                "a" => ElementName::A,
                "b" => ElementName::B,
                "br" => ElementName::Br,
                "p" => ElementName::P,
                _ => panic!("unexpected element name: {}", name.local),
            };
            let mut gathered_attrs = BTreeMap::new();
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                let attr_name = match attr.name.local.to_string().as_str() {
                    "class" => AttrName::Class,
                    "href" => AttrName::Href,
                    _ => panic!("unexpected attr name: {}", attr.name.local),
                };
                gathered_attrs.insert(attr_name, attr.value.to_string());
            }

            let mut children = Vec::new();

            for child in node.children.borrow().iter() {
                match walk_body(child) {
                    Some(simple_child) => children.push(simple_child),
                    None => (),
                };
            }

            Some(SimpleHtml::Element(Element {
                name: element_name,
                attrs: gathered_attrs,
                children,
            }))
        }

        rcdom::NodeData::ProcessingInstruction { .. } => unreachable!(),
    }
}
