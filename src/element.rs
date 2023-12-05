use roxmltree::Node;

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub children: Vec<Element>,
    pub(crate) attributes: Vec<(String, String)>,
    pub(crate) value: Option<String>,
}

impl Element {
    pub(crate) fn new(node: Node) -> Self {
        Self {
            name: get_name(node),
            attributes: get_attributes(node),
            value: get_value(node),
            children: get_children(node),
        }
    }

    pub fn value(&self) -> String {
        match self.value.as_ref() {
            Some(v) => v.clone(),
            None => String::new(),
        }
    }

    pub fn find_all(&self, nm: &str) -> Vec<&Element> {
        self.children.iter().filter(|b| b.name == nm).collect()
    }

    pub fn find_all_mut(&mut self, nm: &str) -> Vec<&mut Element> {
        self.children.iter_mut().filter(|b| b.name == nm).collect()
    }
}

// Helpers for converting roxmltree Nodes to Elements

#[inline]
fn get_name(node: Node) -> String {
    node.tag_name().name().to_string()
}

#[inline]
fn get_attributes(node: Node) -> Vec<(String, String)> {
    node.attributes()
        .map(|a| (a.name().to_string(), a.value().to_string()))
        .collect()
}

#[inline]
fn get_value(node: Node) -> Option<String> {
    match node.text() {
        Some(text) => Some(text.to_string().replace("&", "&amp;")),
        None => None,
    }
}

#[inline]
fn get_children(node: Node) -> Vec<Element> {
    node.children()
        .filter(|n| n.is_element())
        .map(|n| Element::new(n))
        .collect()
}
