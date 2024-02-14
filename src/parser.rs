#![allow(dead_code)]
use std::fs;
#[derive(Debug, PartialEq)]
pub enum TagKind {
    HTML,
    HEAD,
    P,
    NoTag,
}
#[derive(Debug)]
pub struct Tag {
    kind: TagKind,
    closing: bool,
}

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice).split_whitespace().collect(),
            len: slice.len(),
        }
    }
}

impl Row {
    pub fn find_tags(&self) -> Option<Tag> {
        let tag_start = self.string.find('<')?;
        let tag_end = self.string.find('>')?;

        let slice = self.string.get(tag_start + 1..=tag_end - 1).unwrap();
        let closing = if slice.starts_with('/') { true } else { false };
        let offset = if closing { 1 } else { 0 };
        let kind = match &slice[offset..] {
            "html" => TagKind::HTML,
            "head" => TagKind::HEAD,
            "p" => TagKind::P,
            _ => TagKind::NoTag,
        };
        match kind {
            TagKind::NoTag => return None,
            _ => return Some(Tag { kind, closing }),
        }
    }
}
pub struct Document {
    rows: Vec<Row>,
    len: usize,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Row::from(line));
        }
        let len = lines.len();
        Ok(Self { rows: lines, len })
    }

    pub fn check_valid_html(&self) -> bool {
        true
    }
}

pub fn parser() {
    let mut stack: Vec<Tag> = Vec::new();

    let doc = Document::open("browser/index.html").unwrap();

    for item in doc.rows {
        let tag = item.find_tags();
        if let Some(tag) = tag {
            if !tag.closing {
                println!("Pushing tag {:?} on the stack", tag);
                stack.push(tag);
            } else if tag.closing && stack.last().unwrap().kind == tag.kind {
                stack.pop();
                println!("Popping tag {:?} off the stack", tag);
            };
        }
    }

    println!("{:?}", stack);
}
