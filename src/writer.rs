use crate::element::Element;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_tree(root: &Element, path: &str) {
    let file: File = File::create(path).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    write_xml_header(&mut writer);
    write_branch(&mut writer, root, 0);
}

#[inline]
fn write_xml_header(writer: &mut BufWriter<File>) {
    let header: &[u8; 22] = b"<?xml version=\"1.0\"?>\n";
    writer.write(header).unwrap();
}

fn write_branch(mut writer: &mut BufWriter<File>, branch: &Element, depth: usize) {
    let mut buffer: Vec<u8> = Vec::new();

    for _ in 0..depth {
        buffer.extend_from_slice(b"    ");
    }

    buffer.extend_from_slice(b"<");
    buffer.extend_from_slice(branch.name.as_bytes());

    if depth == 0 {
        buffer.extend_from_slice(b" xmlns:exsl=\"http://exslt.org/common\"");
    }

    branch.attributes.iter().for_each(|(key, value)| {
        buffer.extend_from_slice(b" ");
        buffer.extend_from_slice(key.as_bytes());
        buffer.extend_from_slice(b"=\"");
        buffer.extend_from_slice(value.as_bytes());
        buffer.extend_from_slice(b"\"");
    });

    buffer.extend_from_slice(b">\n");
    writer.write(&buffer).unwrap();
    buffer.clear();

    branch.children.iter().for_each(|child| {
        if child.children.len() > 0 {
            write_branch(&mut writer, child, depth + 1);
        } else {
            write_leaf(&mut writer, child, depth + 1);
        }
    });

    for _ in 0..depth {
        buffer.extend_from_slice(b"    ");
    }

    buffer.extend_from_slice(b"</");
    buffer.extend_from_slice(branch.name.as_bytes());
    buffer.extend_from_slice(b">\n");

    writer.write(&buffer).unwrap();
}

fn write_leaf(writer: &mut BufWriter<File>, leaf: &Element, depth: usize) {
    let mut buffer: Vec<u8> = Vec::new();

    for _ in 0..depth {
        buffer.extend_from_slice(b"    ");
    }

    buffer.extend_from_slice(b"<");
    buffer.extend_from_slice(leaf.name.as_bytes());

    leaf.attributes.iter().for_each(|(key, value)| {
        buffer.extend_from_slice(b" ");
        buffer.extend_from_slice(key.as_bytes());
        buffer.extend_from_slice(b"=\"");
        buffer.extend_from_slice(value.as_bytes());
        buffer.extend_from_slice(b"\"");
    });

    if let Some(value) = leaf.value.as_ref() {
        buffer.extend_from_slice(b">");
        buffer.extend_from_slice(value.as_bytes());
        buffer.extend_from_slice(b"</");
        buffer.extend_from_slice(leaf.name.as_bytes());
        buffer.extend_from_slice(b">\n");
    } else {
        buffer.extend_from_slice(b"/>\n");
    }

    writer.write(&buffer).unwrap();
}
