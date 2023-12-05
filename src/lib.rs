pub mod element;
pub mod writer;

use element::Element;
use roxmltree::Document;
use std::fs::read_to_string;

pub fn read_tree(path: &str) -> element::Element {
    let xml: String = read_to_string(path).unwrap();
    let doc: Document = Document::parse(&xml).unwrap();

    Element::new(doc.root().first_child().unwrap())
}
