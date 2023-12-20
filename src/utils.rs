use std::fs::File;
use std::io::prelude::*;

pub fn read_text_from_file(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

