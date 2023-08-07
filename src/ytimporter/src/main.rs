use std::fs;
use std::io;

use html_parser;

fn main() {
    println!("Hello, world!");
    let html_str = fs::read_to_string("imports/test.html").unwrap();
    println!("we have the string read from disc");
    let html_dom = html_parser::Dom::parse(html_str.as_str()).unwrap();
    println!("type: {:?}", html_dom.tree_type);
    for child in html_dom.children.iter() {
        println!(
            "final action was:\t{}",
            explore_node(&child, 0).simple_debug()
        );
    }
    println!("Goodbye.");
}

pub fn simple_debug_display(node: &html_parser::Node, indent_level: usize) {
    match node {
        html_parser::Node::Text(t) => println!("{}\"{}\"", "    ".repeat(indent_level), t.trim()),
        html_parser::Node::Element(e) => {
            println!("{}<{}>", "    ".repeat(indent_level), e.name);
            for child in e.children.iter() {
                simple_debug_display(child, indent_level + 1);
            }
            println!("{}<\\{}>", "    ".repeat(indent_level), e.name);
        }
        html_parser::Node::Comment(c) => println!("{}{}", "    ".repeat(indent_level), c),
    }
}

pub enum Action<'a> {
    ExploreChild(&'a html_parser::Node),
    TraverseBackUp,
    Done,
}

impl<'a> Action<'a> {
    fn simple_debug(&'a self) -> String {
        match self {
            Action::ExploreChild(c) => format!("Exploring: {}", node_debug_repr(c)),
            Action::TraverseBackUp => "Traversing back up...".to_string(),
            Action::Done => "Done!".to_string(),
        }
    }
}

pub fn explore_node<'a>(node: &'a html_parser::Node, indent_level: usize) -> Action<'a> {
    println!("{}", node_debug_repr(node));
    match node {
        html_parser::Node::Text(_t) => Action::Done,
        html_parser::Node::Element(e) => match prompt_for_child(e, indent_level + 1) {
            Action::ExploreChild(child) => explore_node(child, indent_level + 1),
            Action::TraverseBackUp => explore_node(node, indent_level),
            Action::Done => Action::Done,
        },
        html_parser::Node::Comment(_c) => Action::Done,
    }
}

fn prompt_for_child<'a>(element: &'a html_parser::Element, indent_level: usize) -> Action<'a> {
    let mut children_by_index = Vec::new();
    for (index, child) in element.children.iter().enumerate() {
        children_by_index.push(child);
        println!("{}:\t{}", index, node_debug_repr(child));
    }
    println!("select index");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input = input.trim_end().to_string();

    if input == "up" {
        return Action::TraverseBackUp;
    }

    match input.parse::<usize>() {
        Ok(i) => {
            if i < children_by_index.len() {
                Action::ExploreChild(children_by_index.get(i).unwrap())
            } else {
                println!("index must be within range of elements seen");
                prompt_for_child(element, indent_level)
            }
        }
        Err(_) => {
            println!("must input a number for index");
            prompt_for_child(element, indent_level)
        }
    }
}

fn node_debug_repr(node: &html_parser::Node) -> String {
    match node {
        html_parser::Node::Text(t) => format!("\"{}\"", t.trim()),
        html_parser::Node::Element(e) => {
            format!("<{}>...<\\{}>", e.name, e.name)
        }
        html_parser::Node::Comment(c) => format!("{}", c),
    }
}
