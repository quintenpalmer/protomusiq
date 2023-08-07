use std::fs;

use html_parser;

fn main() {
    println!("Hello, world!");
    let html_str = fs::read_to_string("imports/test.html").unwrap();
    let html_dom = html_parser::Dom::parse(html_str.as_str()).unwrap();
    println!("type: {:?}", html_dom.tree_type);
    for child in html_dom.children.iter() {
        explore_node(&child, 0)
    }
    println!("Goodbye.");
}

fn explore_node(node: &html_parser::Node, indent_level: usize) {
    match node {
        html_parser::Node::Text(t) => println!("{}\"{}\"", "    ".repeat(indent_level), t.trim()),
        html_parser::Node::Element(e) => {
            println!("{}<{}>", "    ".repeat(indent_level), e.name);
            for child in e.children.iter() {
                explore_node(child, indent_level + 1);
            }
            println!("{}<\\{}>", "    ".repeat(indent_level), e.name);
        }
        html_parser::Node::Comment(c) => println!("{}{}", "    ".repeat(indent_level), c),
    }
}
