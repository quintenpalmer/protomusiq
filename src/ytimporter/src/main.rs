use std::fs;

fn main() {
    println!("Hello, world!");
    let html_str = fs::read_to_string("imports/test.html").unwrap();
    let html_dom = html_parser::Dom::parse(html_str.as_str()).unwrap();
    println!("type: {:?}", html_dom.tree_type);
    //for child in html_dom.children.iter() {
    //    println!("child: {:?}", child);
    //}
    println!("Goodbye.");
}
