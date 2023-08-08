use std::io;

use html_parser;

#[allow(unused)]
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
    pub fn simple_debug(&'a self) -> String {
        match self {
            Action::ExploreChild(c) => format!("Exploring: {}", node_debug_repr(c)),
            Action::TraverseBackUp => "Traversing back up...".to_string(),
            Action::Done => "Done!".to_string(),
        }
    }
}

pub fn explore_node<'a>(node: &'a html_parser::Node) -> Action<'a> {
    println!("{}", node_debug_repr(node));
    match node {
        html_parser::Node::Text(_t) => Action::Done,
        html_parser::Node::Element(e) => match prompt_for_child(e) {
            Action::ExploreChild(child) => match explore_node(child) {
                Action::ExploreChild(inner_child) => explore_node(inner_child),
                Action::TraverseBackUp => explore_node(node),
                Action::Done => Action::Done,
            },
            Action::TraverseBackUp => Action::TraverseBackUp,
            Action::Done => Action::Done,
        },
        html_parser::Node::Comment(_c) => Action::Done,
    }
}

fn prompt_for_child<'a>(element: &'a html_parser::Element) -> Action<'a> {
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
                prompt_for_child(element)
            }
        }
        Err(_) => {
            println!("must input a number for index");
            prompt_for_child(element)
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
