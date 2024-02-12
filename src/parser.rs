use std::fs;
pub struct Document {
    rows: Vec<String>,
    len: usize,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(String::from(line));
        }
        let len = lines.len();
        Ok(Self { rows: lines, len })
    }
}

pub fn parser() {
    let mut stack: Vec<u8> = Vec::new();

    let doc = Document::open("browser/index.html");
    match doc {
        Ok(dom) => println!("Success, {}", dom.len),
        Err(_) => println!("Failure, boo"),
    }
}
