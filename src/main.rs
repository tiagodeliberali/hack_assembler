use std::env;
use std::fs;

mod builder;
mod parser;

use crate::builder::build_content;
use crate::parser::parse_content;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Please supply a filename");

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (lines, locations) = build_content(content);

    let result = parse_content(lines, locations);

    fs::write(filename.replace(".asm", ".hack"), result.join("\r\n"))
        .expect("Something failed on write file to disk");
}
