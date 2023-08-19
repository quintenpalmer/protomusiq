use std::collections::BTreeMap;

#[derive(Debug)]
pub enum SimpleHtml {
    Text(String),
    Element(Element),
}

#[derive(Debug)]
pub struct Element {
    pub name: ElementName,
    pub attrs: BTreeMap<AttrName, String>,
    pub children: Vec<SimpleHtml>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ElementName {
    Br,
    Div,
    A,
    B,
    P,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AttrName {
    Class,
    Href,
}

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub artist: String,
    pub watched: String,
}
