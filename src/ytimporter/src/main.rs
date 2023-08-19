use std::fs;

#[macro_use]
extern crate html5ever;

mod ever;
mod parser;
#[allow(unused)]
mod proto;

pub enum RunMode {
    Parser,
    Ever,
}

fn main() {
    println!("Hello, world!");
    let html_str = fs::read_to_string("imports/tidied-yt-watch-history.html").unwrap();
    println!("we have the string read from disc");
    match RunMode::Ever {
        RunMode::Parser => {
            let html_dom = html_parser::Dom::parse(html_str.as_str()).unwrap();
            println!("type: {:?}", html_dom.tree_type);
            for child in html_dom.children.iter() {
                println!(
                    "final action was:\t{}",
                    parser::explore_node(&child).simple_debug()
                );
            }
        }
        RunMode::Ever => ever::find_all_watch_info(&html_str),
    }
    println!("Goodbye.");
}
