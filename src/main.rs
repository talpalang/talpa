use std::fs::File;
use std::io::prelude::*;
use gpl::Parser;

fn main() {
    // the .gpl is a temporary file extension (General Programming Language)
    // the example file should be updated with all working components
    let mut file = File::open("./src/example.gpl").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    match gpl::Parser::parse(contents) {
        Err(err) => println!("{}", err),
        Ok(res) => println!("{:?}", res.functions),
    }
}
